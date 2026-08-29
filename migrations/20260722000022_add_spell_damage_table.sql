CREATE TABLE IF NOT EXISTS pf_spell_damage_table (
    id             BIGSERIAL PRIMARY KEY,
    bonus_dmg      INTEGER NOT NULL,
    dmg_type       TEXT,
    category       TEXT,
    kinds          TEXT[] NOT NULL DEFAULT '{}',
    number_of_dice INTEGER,
    die_size       INTEGER,
    spell_id       BIGINT NOT NULL,
    FOREIGN KEY (spell_id) REFERENCES pf_spell_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_spell_damage_table (
    id             BIGSERIAL PRIMARY KEY,
    bonus_dmg      INTEGER NOT NULL,
    dmg_type       TEXT,
    category       TEXT,
    kinds          TEXT[] NOT NULL DEFAULT '{}',
    number_of_dice INTEGER,
    die_size       INTEGER,
    spell_id       BIGINT NOT NULL,
    FOREIGN KEY (spell_id) REFERENCES sf_spell_table(id) ON DELETE CASCADE
);
