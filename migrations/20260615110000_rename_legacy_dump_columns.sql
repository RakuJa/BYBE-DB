-- Several tables' current column names (as defined in 20260510000001/2 and later migrations)
-- were renamed at some point, but only via editing the CREATE TABLE IF NOT EXISTS definition
-- directly -- never via a proper ALTER ... RENAME COLUMN migration. So a database bootstrapped from db/bybe_postgres.sql (converted
-- straight from the SQLite source) already has these tables under their *original* SQLite
-- column names
DO $$
DECLARE
    renames TEXT[][] := ARRAY[
        ['pf_spell_table', 'saving_throw_statistic', 'saving_throw'],
        ['pf_spell_table', 'saving_throw_is_basic', 'basic_saving_throw'],
        ['pf_spell_table', 'action', 'actions'],
        ['sf_spell_table', 'saving_throw_statistic', 'saving_throw'],
        ['sf_spell_table', 'saving_throw_is_basic', 'basic_saving_throw'],
        ['sf_spell_table', 'action', 'actions'],
        ['pf_armor_table', 'bonus_ac', 'ac_bonus'],
        ['pf_shield_table', 'bonus_ac', 'ac_bonus'],
        ['sf_armor_table', 'bonus_ac', 'ac_bonus'],
        ['sf_shield_table', 'bonus_ac', 'ac_bonus']
    ];
    old_name TEXT;
    new_name TEXT;
    tbl TEXT;
    rename_row TEXT[];
BEGIN
    FOREACH rename_row SLICE 1 IN ARRAY renames
    LOOP
        tbl := rename_row[1];
        old_name := rename_row[2];
        new_name := rename_row[3];
        IF EXISTS (
            SELECT 1 FROM information_schema.columns
            WHERE table_name = tbl AND column_name = old_name
        ) THEN
            EXECUTE format('ALTER TABLE %I RENAME COLUMN %I TO %I', tbl, old_name, new_name);
        END IF;
    END LOOP;
END $$;
