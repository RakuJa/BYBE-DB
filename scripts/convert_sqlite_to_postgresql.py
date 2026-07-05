import sqlite3
import re
import argparse
from collections import defaultdict, deque

# Narrow, documented workarounds for known bugs in the source SQLite schema itself (not
# SQLite->Postgres translation gaps). Each entry corrects one column's declared type on one
# table, rather than a general rule, since these are one-off data-modeling mistakes, not a
# pattern to apply broadly.
#
# pf_action_hazard_association_table / sf_action_hazard_association_table declare `hazard_id`
# as TEXT, but it's a foreign key to {pf,sf}_hazard_table(id), which is INTEGER -- almost
# certainly copy-pasted from a sibling association table (e.g. trait_hazard_association_table)
# where the referenced key legitimately is text (trait_table.name).
KNOWN_BAD_COLUMN_TYPES = {
    "pf_action_hazard_association_table": {"hazard_id": ("TEXT", "BIGINT")},
    "sf_action_hazard_association_table": {"hazard_id": ("TEXT", "BIGINT")},
}


def fix_known_bad_column_types(create_sql, table):
    fixes = KNOWN_BAD_COLUMN_TYPES.get(table)
    if not fixes:
        return create_sql
    for column, (wrong_type, correct_type) in fixes.items():
        create_sql = re.sub(
            rf'\b{re.escape(column)}\s+{re.escape(wrong_type)}\b',
            f'{column} {correct_type}',
            create_sql,
            count=1,
            flags=re.IGNORECASE,
        )
    return create_sql


def get_table_dependencies(conn, tables):
    """Returns a dict of table -> set of tables it depends on (via FK)."""
    cur = conn.cursor()
    deps = {t: set() for t in tables}
    table_set = set(tables)
    for table in tables:
        cur.execute(f"PRAGMA foreign_key_list('{table}')")
        for row in cur.fetchall():
            referenced_table = row[2]  # 'table' column in FK list
            if referenced_table in table_set and referenced_table != table:
                deps[table].add(referenced_table)
    return deps


def topological_sort(deps):
    """Kahn's algorithm — returns tables in safe creation order."""
    in_degree = {t: 0 for t in deps}
    graph = defaultdict(set)
    for table, parents in deps.items():
        for parent in parents:
            graph[parent].add(table)
            in_degree[table] += 1
    queue = deque([t for t, d in in_degree.items() if d == 0])
    order = []
    while queue:
        node = queue.popleft()
        order.append(node)
        for child in graph[node]:
            in_degree[child] -= 1
            if in_degree[child] == 0:
                queue.append(child)
    if len(order) != len(deps):
        raise ValueError("Circular dependency detected among tables!")
    return order


def get_column_types(conn, table):
    """Returns an ordered list of declared SQLite types for each column."""
    cur = conn.cursor()
    cur.execute(f"PRAGMA table_info('{table}')")
    # row layout: (cid, name, type, notnull, dflt_value, pk)
    return [row[2].upper() for row in cur.fetchall()]


def fix_boolean_defaults(create_sql, conn, table):
    """SQLite stores booleans as 0/1 ints, including in DEFAULT clauses (e.g.
    `is_melee BOOL NOT NULL DEFAULT 0`), which Postgres rejects for a boolean column.
    Only touch the DEFAULT immediately following a column declared BOOL/BOOLEAN, so
    legitimate `DEFAULT 0` on integer columns elsewhere is left alone."""
    cur = conn.cursor()
    cur.execute(f"PRAGMA table_info('{table}')")
    bool_columns = [row[1] for row in cur.fetchall() if row[2].upper() in ('BOOL', 'BOOLEAN')]
    for col in bool_columns:
        create_sql = re.sub(
            rf'(\b{re.escape(col)}\s+BOOL(?:EAN)?\b[^,()]*?DEFAULT\s+)0\b',
            r'\1FALSE',
            create_sql,
            flags=re.IGNORECASE,
        )
        create_sql = re.sub(
            rf'(\b{re.escape(col)}\s+BOOL(?:EAN)?\b[^,()]*?DEFAULT\s+)1\b',
            r'\1TRUE',
            create_sql,
            flags=re.IGNORECASE,
        )
    return create_sql


def fix_unquoted_text_defaults(create_sql, conn, table):
    """SQLite tolerates unquoted bareword string literals in DEFAULT clauses (e.g.
    `alignment TEXT NOT NULL DEFAULT NO`, meaning the literal string "NO"); Postgres
    requires them quoted, since an unquoted bareword there is parsed as an identifier."""
    cur = conn.cursor()
    cur.execute(f"PRAGMA table_info('{table}')")
    for _cid, name, col_type, _notnull, dflt_value, _pk in cur.fetchall():
        if col_type.upper() not in ('TEXT', 'VARCHAR', 'CHAR'):
            continue
        if dflt_value is None:
            continue
        dflt_value = dflt_value.strip()
        if not dflt_value or dflt_value[0] in ("'", '"') or dflt_value.upper() == 'NULL':
            continue  # already quoted or NULL; nothing to fix
        quoted = "'" + dflt_value.replace("'", "''") + "'"
        create_sql = re.sub(
            rf'(\b{re.escape(name)}\s+(?:TEXT|VARCHAR(?:\(\d+\))?|CHAR(?:\(\d+\))?)\b[^,()]*?DEFAULT\s+)'
            rf'{re.escape(dflt_value)}\b',
            lambda m: m.group(1) + quoted,
            create_sql,
            count=1,
            flags=re.IGNORECASE,
        )
    return create_sql


def get_single_integer_pk_column(conn, table):
    """Returns the auto-increment id column name if the table has exactly one INTEGER
    primary key column (i.e. the ones converted to BIGSERIAL below), else None."""
    cur = conn.cursor()
    cur.execute(f"PRAGMA table_info('{table}')")
    pk_cols = [row for row in cur.fetchall() if row[5] > 0]
    if len(pk_cols) == 1 and pk_cols[0][2].upper() == "INTEGER":
        return pk_cols[0][1]
    return None


def namespace_constraint_names(create_sql, table):
    """SQLite scopes constraint names per-table, so two tables can legally reuse the same
    name (e.g. both pf_creature_table and sf_creature_table declaring `character_stats`).
    Postgres requires constraint-backing index names to be unique per-schema, so prefix
    every constraint name with its owning table to guarantee that."""
    return re.sub(
        r'\bCONSTRAINT\s+"?(\w+)"?',
        lambda m: f'CONSTRAINT "{table}_{m.group(1)}"',
        create_sql,
    )


def _find_matching_paren(s, open_idx):
    """s[open_idx] == '('. Returns the index of its matching close paren."""
    depth = 0
    i = open_idx
    while i < len(s):
        if s[i] == '(':
            depth += 1
        elif s[i] == ')':
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise ValueError("unbalanced parentheses")


def extract_description_unique_constraints(create_sql, table):
    """`description` columns can hold arbitrarily long HTML, which overflows Postgres's btree
    index row size limit when included directly in a UNIQUE constraint -- and Postgres's inline
    table-level `CONSTRAINT ... UNIQUE(...)` syntax doesn't accept expressions like md5(...)
    anyway. The hand-written migrations work around both issues for the equivalent tables by
    hashing `description` and declaring the uniqueness as a separate CREATE UNIQUE INDEX (see
    e.g. item_stats/pf_action_no_duplicates). Mirror that here: pull any UNIQUE constraint that
    references `description` out of the CREATE TABLE body and re-emit it as a standalone index.
    Returns (modified_create_sql, [index_statement, ...]).
    """
    index_statements = []
    pattern = re.compile(r',?\s*CONSTRAINT\s+"(\w+)"\s+UNIQUE\s*\(', re.IGNORECASE)

    search_from = 0
    while True:
        m = pattern.search(create_sql, search_from)
        if not m:
            break
        open_idx = m.end() - 1
        close_idx = _find_matching_paren(create_sql, open_idx)
        cols = create_sql[open_idx + 1:close_idx]
        if not re.search(r'\bdescription\b', cols, re.IGNORECASE):
            search_from = close_idx + 1  # leave this constraint alone, keep scanning past it
            continue
        cols = re.sub(
            r'\bdescription\b(\s+COLLATE\s+\w+)?',
            "md5(COALESCE(description, '__NULL__'))",
            cols,
            flags=re.IGNORECASE,
        )
        constraint_name = m.group(1)
        index_statements.append(
            f'CREATE UNIQUE INDEX IF NOT EXISTS "{constraint_name}" ON "{table}" ({cols});'
        )
        create_sql = create_sql[:m.start()] + create_sql[close_idx + 1:]
        search_from = m.start()  # re-scan from here; positions after this shifted

    return create_sql, index_statements


def sqlite_to_pg(sqlite_path, output_path):
    conn = sqlite3.connect(sqlite_path)
    cur = conn.cursor()
    cur.execute("SELECT name FROM sqlite_master WHERE type='table'")
    tables = [row[0] for row in cur.fetchall()]
    deps = get_table_dependencies(conn, tables)
    tables = topological_sort(deps)
    with open(output_path, 'w') as f:
        f.write("-- PostgreSQL dump converted from SQLite\n")
        f.write("SET client_encoding = 'UTF8';\n\n")
        for table in tables:
            # Get CREATE TABLE statement
            cur.execute("SELECT sql FROM sqlite_master WHERE type='table' AND name=?", (table,))
            create_sql = cur.fetchone()[0]
            # Convert SQLite syntax to PostgreSQL
            create_sql = re.sub(r'INTEGER PRIMARY KEY AUTOINCREMENT', 'BIGSERIAL PRIMARY KEY', create_sql)
            create_sql = re.sub(r'INTEGER PRIMARY KEY', 'BIGSERIAL PRIMARY KEY', create_sql)
            create_sql = re.sub(r'\bBLOB\b', 'BYTEA', create_sql)
            create_sql = re.sub(r'\bREAL\b', 'DOUBLE PRECISION', create_sql)
            create_sql = re.sub(r"DEFAULT \(datetime\('now'\)\)", 'DEFAULT NOW()', create_sql)
            create_sql = fix_boolean_defaults(create_sql, conn, table)
            create_sql = fix_unquoted_text_defaults(create_sql, conn, table)
            create_sql = fix_known_bad_column_types(create_sql, table)
            # SQLite scopes constraint names per-table; Postgres needs them unique per-schema.
            create_sql = namespace_constraint_names(create_sql, table)
            # `description` inside a UNIQUE constraint can overflow Postgres's btree index row
            # size limit (and inline CONSTRAINT syntax can't use expressions like md5() anyway);
            # pull it out into a separate hashed index, matching the hand-written migrations.
            create_sql, extra_indexes = extract_description_unique_constraints(create_sql, table)
            # SQLite's NOCASE collation has no Postgres equivalent; drop it (falls back to
            # the default, case-sensitive collation) rather than fail with an invalid name.
            create_sql = re.sub(r'\s+COLLATE\s+NOCASE\b', '', create_sql, flags=re.IGNORECASE)
            f.write(f"DROP TABLE IF EXISTS \"{table}\" CASCADE;\n")
            f.write(create_sql + ";\n\n")
            for stmt in extra_indexes:
                f.write(stmt + "\n")
            if extra_indexes:
                f.write("\n")

            # Figure out which columns are declared BOOLEAN so we can
            # convert SQLite's 0/1 ints into Postgres TRUE/FALSE.
            col_types = get_column_types(conn, table)
            bool_col_indexes = {i for i, t in enumerate(col_types) if t in ('BOOL', 'BOOLEAN')}

            # Dump all rows
            cur.execute(f"SELECT * FROM \"{table}\"")
            rows = cur.fetchall()
            if rows:
                for row in rows:
                    values = []
                    for i, val in enumerate(row):
                        if val is None:
                            values.append("NULL")
                        elif i in bool_col_indexes:
                            # SQLite stores booleans as 0/1 ints
                            values.append("TRUE" if val else "FALSE")
                        elif isinstance(val, (int, float)):
                            values.append(str(val))
                        else:
                            escaped = str(val).replace("'", "''")
                            values.append(f"'{escaped}'")
                    f.write(f"INSERT INTO \"{table}\" VALUES ({', '.join(values)});\n")
                f.write("\n")

            # These plain INSERTs bypass the BIGSERIAL sequence entirely, so it stays at its
            # default (1) even though the table now holds real rows. Resync it to match the
            # actual max id, so any later plain `INSERT ... RETURNING id` gets a fresh value
            # instead of colliding with data seeded here.
            pk_col = get_single_integer_pk_column(conn, table)
            if pk_col is not None:
                pk_index = [row[1] for row in conn.execute(f"PRAGMA table_info('{table}')")].index(pk_col)
                ids = [row[pk_index] for row in rows if row[pk_index] is not None]
                seq_expr = f"pg_get_serial_sequence('{table}', '{pk_col}')"
                if ids:
                    f.write(f"SELECT setval({seq_expr}, {max(ids)}, true);\n\n")
                else:
                    f.write(f"SELECT setval({seq_expr}, 1, false);\n\n")
    conn.close()
    print(f"Done! Output written to {output_path}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Convert SQLite database to PostgreSQL SQL dump")
    parser.add_argument("sqlite_path", help="Path to the SQLite .db file")
    parser.add_argument("output_path", help="Path for the output .sql file")
    args = parser.parse_args()
    sqlite_to_pg(args.sqlite_path, args.output_path)