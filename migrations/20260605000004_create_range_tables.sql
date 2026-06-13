CREATE TABLE IF NOT EXISTS pf_range_table (
    id         BIGSERIAL PRIMARY KEY,
    value      TEXT NOT NULL,
    increment  TEXT,
    max        TEXT,
    CONSTRAINT pf_range_stats UNIQUE NULLS NOT DISTINCT (
        value, increment, max
    )
);

CREATE TABLE IF NOT EXISTS pf_range_sense_association_table (
    range_id    BIGINT NOT NULL,
    sense_id    BIGINT NOT NULL,
    PRIMARY KEY (range_id, sense_id),
    FOREIGN KEY (range_id) REFERENCES pf_range_table(id) ON DELETE CASCADE,
    FOREIGN KEY (sense_id) REFERENCES pf_sense_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_weapon_range_association_table (
    range_id    BIGINT NOT NULL,
    weapon_id   BIGINT NOT NULL,
    PRIMARY KEY (range_id, weapon_id),
    FOREIGN KEY (range_id) REFERENCES pf_range_table(id) ON DELETE CASCADE,
    FOREIGN KEY (weapon_id) REFERENCES pf_weapon_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_spell_range_association_table (
    range_id    BIGINT NOT NULL,
    spell_id    BIGINT NOT NULL,
    PRIMARY KEY (range_id, spell_id),
    FOREIGN KEY (range_id) REFERENCES pf_range_table(id) ON DELETE CASCADE,
    FOREIGN KEY (spell_id) REFERENCES pf_spell_table(id) ON DELETE CASCADE
);

ALTER TABLE pf_spell_table DROP COLUMN range;
ALTER TABLE pf_weapon_table DROP COLUMN range;
ALTER TABLE pf_sense_table DROP COLUMN range;

--- Starfinder

CREATE TABLE IF NOT EXISTS sf_range_table (
    id         BIGSERIAL PRIMARY KEY,
    value      TEXT NOT NULL,
    increment  TEXT,
    max        TEXT,
    CONSTRAINT sf_range_stats UNIQUE NULLS NOT DISTINCT (
        value, increment, max
    )
);

CREATE TABLE IF NOT EXISTS sf_range_sense_association_table (
    range_id    BIGINT NOT NULL,
    sense_id    BIGINT NOT NULL,
    PRIMARY KEY (range_id, sense_id),
    FOREIGN KEY (range_id) REFERENCES sf_range_table(id) ON DELETE CASCADE,
    FOREIGN KEY (sense_id) REFERENCES sf_sense_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_weapon_range_association_table (
    range_id    BIGINT NOT NULL,
    weapon_id   BIGINT NOT NULL,
    PRIMARY KEY (range_id, weapon_id),
    FOREIGN KEY (range_id) REFERENCES sf_range_table(id) ON DELETE CASCADE,
    FOREIGN KEY (weapon_id) REFERENCES sf_weapon_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_spell_range_association_table (
    range_id    BIGINT NOT NULL,
    spell_id    BIGINT NOT NULL,
    PRIMARY KEY (range_id, spell_id),
    FOREIGN KEY (range_id) REFERENCES sf_range_table(id) ON DELETE CASCADE,
    FOREIGN KEY (spell_id) REFERENCES sf_spell_table(id) ON DELETE CASCADE
);

ALTER TABLE sf_spell_table DROP COLUMN range;
ALTER TABLE sf_weapon_table DROP COLUMN range;
ALTER TABLE sf_sense_table DROP COLUMN range;