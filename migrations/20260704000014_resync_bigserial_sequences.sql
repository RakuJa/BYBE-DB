-- db/bybe_postgres.sql was converted from SQLite using plain `INSERT INTO t VALUES (explicit_id, ...)`
-- statements, with no `setval()` calls anywhere. So every BIGSERIAL sequence starts at its default (1) even though
-- the seeded tables already hold real rows. This migration resyncs every BIGSERIAL/SERIAL sequence in the
-- public schema to match its table's actual max id, so subsequent plain inserts get fresh ids
-- regardless of how the table was seeded.
DO $$
DECLARE
    r RECORD;
    seq_name TEXT;
    max_id BIGINT;
BEGIN
    FOR r IN
        SELECT c.table_name, c.column_name
        FROM information_schema.columns c
        WHERE c.table_schema = 'public'
          AND c.column_default LIKE 'nextval(%'
    LOOP
        seq_name := pg_get_serial_sequence(quote_ident(r.table_name), r.column_name);
        IF seq_name IS NOT NULL THEN
            EXECUTE format('SELECT COALESCE(MAX(%I), 0) FROM %I', r.column_name, r.table_name) INTO max_id;
            IF max_id > 0 THEN
                PERFORM setval(seq_name, max_id, true);
            ELSE
                PERFORM setval(seq_name, 1, false);
            END IF;
        END IF;
    END LOOP;
END $$;
