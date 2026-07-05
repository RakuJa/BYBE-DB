-- The next migration (20260616000006) creates a UNIQUE index on pf_action_table/sf_action_table
-- to prevent future duplicates/ A database bulk-restored from a historical (sqlite)
-- snapshot doesn't get that per-insert dedup, so it can contain genuine
-- duplicate action rows that would make that CREATE UNIQUE INDEX fail.
DO $$
DECLARE
    gs TEXT;
    assoc_table TEXT;
    other_col TEXT;
BEGIN
    FOREACH gs IN ARRAY ARRAY['pf', 'sf']
    LOOP
        EXECUTE format($f$
            CREATE TEMP TABLE action_id_map AS
            SELECT id AS old_id, MIN(id) OVER (PARTITION BY
                name, action_type, n_of_actions, category,
                md5(COALESCE(description, '__NULL__')), license, remaster, source, slug, rarity
            ) AS canonical_id
            FROM %I
        $f$, gs || '_action_table');

        FOR assoc_table, other_col IN
            SELECT * FROM (VALUES
                (gs || '_creature_action_association_table', 'creature_id'),
                (gs || '_action_hazard_association_table', 'hazard_id'),
                (gs || '_trait_action_association_table', 'trait_id')
            ) AS t(assoc_table, other_col)
        LOOP
            EXECUTE format($f$
                INSERT INTO %1$I (%2$I, action_id)
                SELECT t.%2$I, m.canonical_id
                FROM %1$I t
                JOIN action_id_map m ON m.old_id = t.action_id
                WHERE m.old_id <> m.canonical_id
                ON CONFLICT DO NOTHING
            $f$, assoc_table, other_col);

            EXECUTE format($f$
                DELETE FROM %1$I
                WHERE action_id IN (SELECT old_id FROM action_id_map WHERE old_id <> canonical_id)
            $f$, assoc_table);
        END LOOP;

        EXECUTE format($f$
            DELETE FROM %I
            WHERE id IN (SELECT old_id FROM action_id_map WHERE old_id <> canonical_id)
        $f$, gs || '_action_table');

        DROP TABLE action_id_map;
    END LOOP;
END $$;
