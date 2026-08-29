CREATE TABLE IF NOT EXISTS pf_spell_heightening_interval_table (
    id       BIGSERIAL PRIMARY KEY,
    interval INTEGER NOT NULL,
    spell_id BIGINT NOT NULL,
    FOREIGN KEY (spell_id) REFERENCES pf_spell_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_spell_heightening_interval_damage_table (
    id             BIGSERIAL PRIMARY KEY,
    number_of_dice INTEGER,
    die_size       INTEGER,
    bonus_dmg      INTEGER NOT NULL,
    heightening_id BIGINT NOT NULL,
    FOREIGN KEY (heightening_id) REFERENCES pf_spell_heightening_interval_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_spell_heightening_fixed_level_table (
    id         BIGSERIAL PRIMARY KEY,
    level      INTEGER NOT NULL,
    target     TEXT,
    range      TEXT,
    area_type  TEXT,
    area_value INTEGER,
    spell_id   BIGINT NOT NULL,
    FOREIGN KEY (spell_id) REFERENCES pf_spell_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_spell_heightening_fixed_level_damage_table (
    id              BIGSERIAL PRIMARY KEY,
    dmg_type        TEXT NOT NULL,
    category        TEXT,
    kinds           TEXT[] NOT NULL DEFAULT '{}',
    number_of_dice  INTEGER,
    die_size        INTEGER,
    bonus_dmg       INTEGER NOT NULL,
    fixed_level_id  BIGINT NOT NULL,
    FOREIGN KEY (fixed_level_id) REFERENCES pf_spell_heightening_fixed_level_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_spell_heightening_interval_table (
    id       BIGSERIAL PRIMARY KEY,
    interval INTEGER NOT NULL,
    spell_id BIGINT NOT NULL,
    FOREIGN KEY (spell_id) REFERENCES sf_spell_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_spell_heightening_interval_damage_table (
    id             BIGSERIAL PRIMARY KEY,
    number_of_dice INTEGER,
    die_size       INTEGER,
    bonus_dmg      INTEGER NOT NULL,
    heightening_id BIGINT NOT NULL,
    FOREIGN KEY (heightening_id) REFERENCES sf_spell_heightening_interval_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_spell_heightening_fixed_level_table (
    id         BIGSERIAL PRIMARY KEY,
    level      INTEGER NOT NULL,
    target     TEXT,
    range      TEXT,
    area_type  TEXT,
    area_value INTEGER,
    spell_id   BIGINT NOT NULL,
    FOREIGN KEY (spell_id) REFERENCES sf_spell_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_spell_heightening_fixed_level_damage_table (
    id              BIGSERIAL PRIMARY KEY,
    dmg_type        TEXT NOT NULL,
    category        TEXT,
    kinds           TEXT[] NOT NULL DEFAULT '{}',
    number_of_dice  INTEGER,
    die_size        INTEGER,
    bonus_dmg       INTEGER NOT NULL,
    fixed_level_id  BIGINT NOT NULL,
    FOREIGN KEY (fixed_level_id) REFERENCES sf_spell_heightening_fixed_level_table(id) ON DELETE CASCADE
);
