CREATE TABLE IF NOT EXISTS pf_hazard_weakness_association_table (
    hazard_id  BIGINT NOT NULL,
    weakness_id  BIGINT NOT NULL,

    PRIMARY KEY (hazard_id, weakness_id),

    FOREIGN KEY (hazard_id) REFERENCES pf_hazard_table(id) ON DELETE CASCADE,
    FOREIGN KEY (weakness_id) REFERENCES pf_weakness_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_hazard_resistance_association_table (
    hazard_id    BIGINT NOT NULL,
    resistance_id  BIGINT NOT NULL,

    PRIMARY KEY (hazard_id, resistance_id),

    FOREIGN KEY (hazard_id)   REFERENCES pf_hazard_table(id) ON DELETE CASCADE,
    FOREIGN KEY (resistance_id) REFERENCES pf_resistance_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_hazard_immunity_association_table (
    hazard_id    BIGINT NOT NULL,
    immunity_id  TEXT NOT NULL,

    PRIMARY KEY (hazard_id, immunity_id),

    FOREIGN KEY (hazard_id)   REFERENCES pf_hazard_table(id) ON DELETE CASCADE,
    FOREIGN KEY (immunity_id) REFERENCES pf_immunity_table(name) ON DELETE CASCADE
);



CREATE TABLE IF NOT EXISTS sf_hazard_weakness_association_table (
    hazard_id  BIGINT NOT NULL,
    weakness_id  BIGINT NOT NULL,

    PRIMARY KEY (hazard_id, weakness_id),

    FOREIGN KEY (hazard_id) REFERENCES sf_hazard_table(id) ON DELETE CASCADE,
    FOREIGN KEY (weakness_id) REFERENCES sf_weakness_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_hazard_resistance_association_table (
    hazard_id    BIGINT NOT NULL,
    resistance_id  BIGINT NOT NULL,

    PRIMARY KEY (hazard_id, resistance_id),

    FOREIGN KEY (hazard_id)   REFERENCES sf_hazard_table(id) ON DELETE CASCADE,
    FOREIGN KEY (resistance_id) REFERENCES sf_resistance_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_hazard_immunity_association_table (
    hazard_id    BIGINT NOT NULL,
    immunity_id  TEXT NOT NULL,

    PRIMARY KEY (hazard_id, immunity_id),

    FOREIGN KEY (hazard_id)   REFERENCES sf_hazard_table(id) ON DELETE CASCADE,
    FOREIGN KEY (immunity_id) REFERENCES sf_immunity_table(name) ON DELETE CASCADE
);