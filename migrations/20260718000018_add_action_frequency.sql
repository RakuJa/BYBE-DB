ALTER TABLE pf_action_table ADD COLUMN frequency_max INTEGER;
ALTER TABLE pf_action_table ADD COLUMN frequency_per TEXT;
ALTER TABLE sf_action_table ADD COLUMN frequency_max INTEGER;
ALTER TABLE sf_action_table ADD COLUMN frequency_per TEXT;

DROP INDEX pf_action_no_duplicates;
DROP INDEX sf_action_no_duplicates;

CREATE UNIQUE INDEX pf_action_no_duplicates
    ON pf_action_table (
        name,
        action_type,
        n_of_actions,
        category,
        md5(COALESCE(description, '__NULL__')),
        license,
        remaster,
        source,
        slug,
        rarity,
        frequency_max,
        frequency_per
    ) NULLS NOT DISTINCT;

CREATE UNIQUE INDEX sf_action_no_duplicates
    ON sf_action_table (
        name,
        action_type,
        n_of_actions,
        category,
        md5(COALESCE(description, '__NULL__')),
        license,
        remaster,
        source,
        slug,
        rarity,
        frequency_max,
        frequency_per
    ) NULLS NOT DISTINCT;
