DROP TABLE pf_weapon_attack_effect_table;
DROP TABLE sf_weapon_attack_effect_table;

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
        rarity
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
        rarity
    ) NULLS NOT DISTINCT;


CREATE TABLE IF NOT EXISTS pf_weapon_action_association_table (
    weapon_id   BIGINT NOT NULL,
    action_id   BIGINT NOT NULL,
    PRIMARY KEY (weapon_id, action_id),
    FOREIGN KEY (weapon_id) REFERENCES pf_weapon_table(id) ON DELETE CASCADE,
    FOREIGN KEY (action_id) REFERENCES pf_action_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_weapon_action_association_table (
    weapon_id   BIGINT NOT NULL,
    action_id   BIGINT NOT NULL,
    PRIMARY KEY (weapon_id, action_id),
    FOREIGN KEY (weapon_id) REFERENCES sf_weapon_table(id) ON DELETE CASCADE,
    FOREIGN KEY (action_id) REFERENCES sf_action_table(id) ON DELETE CASCADE
);