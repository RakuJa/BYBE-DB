CREATE TABLE IF NOT EXISTS  pf_creature_action_association_table (
    creature_id INTEGER NOT NULL,
    action_id   INTEGER NOT NULL,
    PRIMARY KEY (creature_id, action_id),
    FOREIGN KEY (creature_id)
        REFERENCES pf_creature_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (action_id)
        REFERENCES pf_action_table(id)
        ON DELETE CASCADE
);

ALTER TABLE pf_action_table
RENAME TO pf_action_table_old;

CREATE TABLE IF NOT EXISTS pf_action_table (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    action_type TEXT NOT NULL,
    n_of_actions INTEGER,
    category TEXT,
    description TEXT NOT NULL,
    license TEXT NOT NULL,
    remaster BOOLEAN NOT NULL,
    source TEXT NOT NULL,
    slug TEXT,
    rarity TEXT NOT NULL
);

INSERT INTO pf_action_table (
    id,
    name,
    action_type,
    n_of_actions,
    category,
    description,
    license,
    remaster,
    source,
    slug,
    rarity
)
SELECT
    id,
    name,
    action_type,
    n_of_actions,
    category,
    description,
    license,
    remaster,
    source,
    slug,
    rarity
FROM pf_action_table_old;

INSERT INTO pf_creature_action_association_table (creature_id, action_id)
SELECT creature_id, id
FROM pf_action_table_old;
DROP TABLE pf_action_table_old;



ALTER TABLE pf_trait_action_association_table
RENAME TO pf_trait_action_association_table_old;

CREATE TABLE IF NOT EXISTS pf_trait_action_association_table (
    action_id INTEGER NOT NULL,
    trait_id TEXT NOT NULL,
    PRIMARY KEY (action_id, trait_id),
    FOREIGN KEY (action_id)
        REFERENCES pf_action_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (trait_id)
        REFERENCES pf_trait_table(name)
        ON DELETE CASCADE
);

INSERT INTO pf_trait_action_association_table
SELECT * FROM pf_trait_action_association_table_old;

DROP TABLE pf_trait_action_association_table_old;

ALTER TABLE pf_action_hazard_association_table
RENAME TO pf_action_hazard_association_table_old;

CREATE TABLE IF NOT EXISTS pf_action_hazard_association_table (
    action_id INTEGER NOT NULL,
    hazard_id TEXT NOT NULL,
    PRIMARY KEY (action_id, hazard_id),
    FOREIGN KEY (action_id)
        REFERENCES pf_action_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (hazard_id)
        REFERENCES pf_hazard_table(id)
        ON DELETE CASCADE
);

INSERT INTO pf_action_hazard_association_table
SELECT * FROM pf_action_hazard_association_table_old;
DROP TABLE pf_action_hazard_association_table_old;

ALTER TABLE pf_creature_action_association_table
RENAME TO pf_creature_action_association_table_old;

CREATE TABLE IF NOT EXISTS pf_creature_action_association_table (
    creature_id INTEGER NOT NULL,
    action_id   INTEGER NOT NULL,
    PRIMARY KEY (creature_id, action_id),
    FOREIGN KEY (creature_id)
        REFERENCES pf_creature_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (action_id)
        REFERENCES pf_action_table(id)
        ON DELETE CASCADE
);

INSERT INTO pf_creature_action_association_table
SELECT * FROM pf_creature_action_association_table_old;
DROP TABLE pf_creature_action_association_table_old;








CREATE TABLE IF NOT EXISTS sf_creature_action_association_table (
    creature_id INTEGER NOT NULL,
    action_id   INTEGER NOT NULL,
    PRIMARY KEY (creature_id, action_id),
    FOREIGN KEY (creature_id)
        REFERENCES sf_creature_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (action_id)
        REFERENCES sf_action_table(id)
        ON DELETE CASCADE
);

ALTER TABLE sf_action_table
RENAME TO sf_action_table_old;

CREATE TABLE IF NOT EXISTS sf_action_table (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    action_type TEXT NOT NULL,
    n_of_actions INTEGER,
    category TEXT,
    description TEXT NOT NULL,
    license TEXT NOT NULL,
    remaster BOOLEAN NOT NULL,
    source TEXT NOT NULL,
    slug TEXT,
    rarity TEXT NOT NULL
);

INSERT INTO sf_action_table (
    id,
    name,
    action_type,
    n_of_actions,
    category,
    description,
    license,
    remaster,
    source,
    slug,
    rarity
)
SELECT
    id,
    name,
    action_type,
    n_of_actions,
    category,
    description,
    license,
    remaster,
    source,
    slug,
    rarity
FROM sf_action_table_old;

INSERT INTO sf_creature_action_association_table (creature_id, action_id)
SELECT creature_id, id
FROM sf_action_table_old;
DROP TABLE sf_action_table_old;

ALTER TABLE sf_trait_action_association_table
RENAME TO sf_trait_action_association_table_old;

CREATE TABLE IF NOT EXISTS sf_trait_action_association_table (
    action_id INTEGER NOT NULL,
    trait_id TEXT NOT NULL,
    PRIMARY KEY (action_id, trait_id),
    FOREIGN KEY (action_id)
        REFERENCES sf_action_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (trait_id)
        REFERENCES sf_trait_table(name)
        ON DELETE CASCADE
);

INSERT INTO sf_trait_action_association_table
SELECT * FROM sf_trait_action_association_table_old;
DROP TABLE sf_trait_action_association_table_old;

ALTER TABLE sf_action_hazard_association_table
RENAME TO sf_action_hazard_association_table_old;

CREATE TABLE IF NOT EXISTS sf_action_hazard_association_table (
    action_id INTEGER NOT NULL,
    hazard_id TEXT NOT NULL,
    PRIMARY KEY (action_id, hazard_id),
    FOREIGN KEY (action_id)
        REFERENCES sf_action_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (hazard_id)
        REFERENCES sf_hazard_table(id)
        ON DELETE CASCADE
);

INSERT INTO sf_action_hazard_association_table
SELECT * FROM sf_action_hazard_association_table_old;
DROP TABLE sf_action_hazard_association_table_old;

ALTER TABLE sf_creature_action_association_table
RENAME TO sf_creature_action_association_table_old;

CREATE TABLE IF NOT EXISTS sf_creature_action_association_table (
    creature_id INTEGER NOT NULL,
    action_id   INTEGER NOT NULL,
    PRIMARY KEY (creature_id, action_id),
    FOREIGN KEY (creature_id)
        REFERENCES sf_creature_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (action_id)
        REFERENCES sf_action_table(id)
        ON DELETE CASCADE
);

INSERT INTO sf_creature_action_association_table
SELECT * FROM sf_creature_action_association_table_old;
DROP TABLE sf_creature_action_association_table_old;
