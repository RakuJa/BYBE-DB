import sqlite3
import re
import argparse
from collections import defaultdict, deque


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
            f.write(f"DROP TABLE IF EXISTS \"{table}\" CASCADE;\n")
            f.write(create_sql + ";\n\n")

            # Figure out which columns are declared BOOLEAN so we can
            # convert SQLite's 0/1 ints into Postgres TRUE/FALSE.
            col_types = get_column_types(conn, table)
            bool_col_indexes = {i for i, t in enumerate(col_types) if t == 'BOOLEAN'}

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
    conn.close()
    print(f"Done! Output written to {output_path}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Convert SQLite database to PostgreSQL SQL dump")
    parser.add_argument("sqlite_path", help="Path to the SQLite .db file")
    parser.add_argument("output_path", help="Path for the output .sql file")
    args = parser.parse_args()
    sqlite_to_pg(args.sqlite_path, args.output_path)