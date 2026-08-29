CREATE TABLE IF NOT EXISTS pf_action_strike_table (
    id              BIGSERIAL PRIMARY KEY,
    slug            TEXT,
    label           TEXT,
    attack_modifier INTEGER,
    category        TEXT,
    weapon_group    TEXT,
    dmg_type        TEXT,
    number_of_dice  INTEGER,
    die_size        INTEGER,
    bonus_dmg       INTEGER NOT NULL,
    range_increment INTEGER,
    max_range       INTEGER,
    action_id       BIGINT NOT NULL,
    FOREIGN KEY (action_id) REFERENCES pf_action_table(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX pf_action_strike_no_duplicates
    ON pf_action_strike_table (
        action_id,
        COALESCE(slug, '__NULL__'),
        COALESCE(label, '__NULL__'),
        COALESCE(attack_modifier, -1),
        COALESCE(category, '__NULL__'),
        COALESCE(weapon_group, '__NULL__'),
        COALESCE(dmg_type, '__NULL__'),
        COALESCE(number_of_dice, -1),
        COALESCE(die_size, -1),
        bonus_dmg,
        COALESCE(range_increment, -1),
        COALESCE(max_range, -1)
    );

CREATE TABLE IF NOT EXISTS pf_trait_action_strike_association_table (
    action_strike_id BIGINT NOT NULL,
    trait_id         TEXT NOT NULL,
    PRIMARY KEY (action_strike_id, trait_id),
    FOREIGN KEY (action_strike_id) REFERENCES pf_action_strike_table(id) ON DELETE CASCADE,
    FOREIGN KEY (trait_id)         REFERENCES pf_trait_table(name)      ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_action_strike_table (
    id              BIGSERIAL PRIMARY KEY,
    slug            TEXT,
    label           TEXT,
    attack_modifier INTEGER,
    category        TEXT,
    weapon_group    TEXT,
    dmg_type        TEXT,
    number_of_dice  INTEGER,
    die_size        INTEGER,
    bonus_dmg       INTEGER NOT NULL,
    range_increment INTEGER,
    max_range       INTEGER,
    action_id       BIGINT NOT NULL,
    FOREIGN KEY (action_id) REFERENCES sf_action_table(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX sf_action_strike_no_duplicates
    ON sf_action_strike_table (
        action_id,
        COALESCE(slug, '__NULL__'),
        COALESCE(label, '__NULL__'),
        COALESCE(attack_modifier, -1),
        COALESCE(category, '__NULL__'),
        COALESCE(weapon_group, '__NULL__'),
        COALESCE(dmg_type, '__NULL__'),
        COALESCE(number_of_dice, -1),
        COALESCE(die_size, -1),
        bonus_dmg,
        COALESCE(range_increment, -1),
        COALESCE(max_range, -1)
    );

CREATE TABLE IF NOT EXISTS sf_trait_action_strike_association_table (
    action_strike_id BIGINT NOT NULL,
    trait_id         TEXT NOT NULL,
    PRIMARY KEY (action_strike_id, trait_id),
    FOREIGN KEY (action_strike_id) REFERENCES sf_action_strike_table(id) ON DELETE CASCADE,
    FOREIGN KEY (trait_id)         REFERENCES sf_trait_table(name)      ON DELETE CASCADE
);
