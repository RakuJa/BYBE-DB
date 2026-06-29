DROP TABLE pf_resistance_double_vs_table;
DROP TABLE pf_resistance_exception_vs_table;

CREATE TABLE IF NOT EXISTS pf_resistance (
    id    BIGSERIAL PRIMARY KEY,
    name  TEXT    NOT NULL,
    value INTEGER NOT NULL,
    UNIQUE (name, value)
);

INSERT INTO pf_resistance (name, value)
SELECT DISTINCT name, value
FROM pf_resistance_table
ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS pf_creature_resistance_association_table (
    creature_id    BIGINT NOT NULL,
    resistance_id  BIGINT NOT NULL,

    PRIMARY KEY (creature_id, resistance_id),

    FOREIGN KEY (creature_id)   REFERENCES pf_creature_table(id)  ON DELETE CASCADE,
    FOREIGN KEY (resistance_id) REFERENCES pf_resistance(id)       ON DELETE CASCADE
);

INSERT INTO pf_creature_resistance_association_table (creature_id, resistance_id)
    SELECT
        old.creature_id,
        new_r.id AS resistance_id
    FROM pf_resistance_table old
        JOIN pf_resistance new_r ON new_r.name  = old.name AND new_r.value = old.value
ON CONFLICT DO NOTHING;

DROP TABLE pf_resistance_table CASCADE;
ALTER TABLE pf_resistance RENAME TO pf_resistance_table;


CREATE TABLE IF NOT EXISTS pf_resistance_double_vs_table (
    resistance_id BIGINT NOT NULL,
    vs_name       TEXT NOT NULL,
    PRIMARY KEY (resistance_id, vs_name),
    FOREIGN KEY (resistance_id) REFERENCES pf_resistance_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_resistance_exception_vs_table (
    resistance_id BIGINT NOT NULL,
    vs_name       TEXT NOT NULL,
    PRIMARY KEY (resistance_id, vs_name),
    FOREIGN KEY (resistance_id) REFERENCES pf_resistance_table(id) ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS pf_weakness (
    id    BIGSERIAL PRIMARY KEY,
    name  TEXT    NOT NULL,
    value INTEGER NOT NULL,
    UNIQUE (name, value)
);

INSERT INTO pf_weakness (name, value)
SELECT DISTINCT name, value
FROM pf_weakness_table
ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS pf_creature_weakness_association_table (
    creature_id    BIGINT NOT NULL,
    weakness_id  BIGINT NOT NULL,

    PRIMARY KEY (creature_id, weakness_id),

    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id)  ON DELETE CASCADE,
    FOREIGN KEY (weakness_id) REFERENCES pf_weakness(id)       ON DELETE CASCADE
);

INSERT INTO pf_creature_weakness_association_table (creature_id, weakness_id)
SELECT
    old.creature_id,
    new_w.id AS weakness_id
FROM pf_weakness_table old
         JOIN pf_weakness new_w ON new_w.name = old.name AND new_w.value = old.value
ON CONFLICT DO NOTHING;

DROP TABLE pf_weakness_table CASCADE;
ALTER TABLE pf_weakness RENAME TO pf_weakness_table;



DROP TABLE sf_resistance_double_vs_table;
DROP TABLE sf_resistance_exception_vs_table;

CREATE TABLE IF NOT EXISTS sf_resistance (
    id    BIGSERIAL PRIMARY KEY,
    name  TEXT    NOT NULL,
    value INTEGER NOT NULL,
    UNIQUE (name, value)
);

INSERT INTO sf_resistance (name, value)
SELECT DISTINCT name, value
FROM sf_resistance_table
ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS sf_creature_resistance_association_table (
    creature_id    BIGINT NOT NULL,
    resistance_id  BIGINT NOT NULL,

    PRIMARY KEY (creature_id, resistance_id),

    FOREIGN KEY (creature_id)   REFERENCES sf_creature_table(id)  ON DELETE CASCADE,
    FOREIGN KEY (resistance_id) REFERENCES sf_resistance(id)       ON DELETE CASCADE
);

INSERT INTO sf_creature_resistance_association_table (creature_id, resistance_id)
SELECT
    old.creature_id,
    new_r.id AS resistance_id
FROM sf_resistance_table old
         JOIN sf_resistance new_r ON new_r.name  = old.name AND new_r.value = old.value
ON CONFLICT DO NOTHING;

DROP TABLE sf_resistance_table CASCADE;
ALTER TABLE sf_resistance RENAME TO sf_resistance_table;


CREATE TABLE IF NOT EXISTS sf_resistance_double_vs_table (
    resistance_id BIGINT NOT NULL,
    vs_name       TEXT NOT NULL,
    PRIMARY KEY (resistance_id, vs_name),
    FOREIGN KEY (resistance_id) REFERENCES sf_resistance_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_resistance_exception_vs_table (
    resistance_id BIGINT NOT NULL,
    vs_name       TEXT NOT NULL,
    PRIMARY KEY (resistance_id, vs_name),
    FOREIGN KEY (resistance_id) REFERENCES sf_resistance_table(id) ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS sf_weakness (
    id    BIGSERIAL PRIMARY KEY,
    name  TEXT    NOT NULL,
    value INTEGER NOT NULL,
    UNIQUE (name, value)
);

INSERT INTO sf_weakness (name, value)
SELECT DISTINCT name, value
FROM sf_weakness_table
ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS sf_creature_weakness_association_table (
    creature_id    BIGINT NOT NULL,
    weakness_id  BIGINT NOT NULL,

    PRIMARY KEY (creature_id, weakness_id),

    FOREIGN KEY (creature_id) REFERENCES sf_creature_table(id)  ON DELETE CASCADE,
    FOREIGN KEY (weakness_id) REFERENCES sf_weakness(id)       ON DELETE CASCADE
);

INSERT INTO sf_creature_weakness_association_table (creature_id, weakness_id)
SELECT
    old.creature_id,
    new_w.id AS weakness_id
FROM sf_weakness_table old
         JOIN sf_weakness new_w ON new_w.name = old.name AND new_w.value = old.value
ON CONFLICT DO NOTHING;

DROP TABLE sf_weakness_table CASCADE;
ALTER TABLE sf_weakness RENAME TO sf_weakness_table;



ALTER TABLE pf_immunity_creature_association_table RENAME TO pf_creature_immunity_association_table;
ALTER TABLE sf_immunity_creature_association_table RENAME TO sf_creature_immunity_association_table;
