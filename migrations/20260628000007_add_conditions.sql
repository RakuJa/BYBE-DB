CREATE TABLE IF NOT EXISTS pf_condition_table (
    name TEXT PRIMARY KEY NOT NULL,
    rule TEXT NOT NULL,
    note TEXT,
    summary TEXT,
    license TEXT NOT NULL,
    remaster BOOLEAN NOT NULL,
    source TEXT NOT NULL,
    is_perpetual BOOLEAN NOT NULL,
    is_stackable BOOLEAN NOT NULL,
    condition_group TEXT
);

CREATE TABLE IF NOT EXISTS sf_condition_table (
    name TEXT PRIMARY KEY NOT NULL,
    rule TEXT NOT NULL,
    note TEXT,
    summary TEXT,
    license TEXT NOT NULL,
    remaster BOOLEAN NOT NULL,
    source TEXT NOT NULL,
    is_perpetual BOOLEAN NOT NULL,
    is_stackable BOOLEAN NOT NULL,
    condition_group TEXT
);

CREATE TABLE IF NOT EXISTS pf_condition_overrides_table (
    source_condition_id TEXT NOT NULL,
    overridden_condition_id TEXT NOT NULL,
    PRIMARY KEY (source_condition_id, overridden_condition_id),
    FOREIGN KEY (source_condition_id) REFERENCES pf_condition_table(name) ON DELETE CASCADE,
    FOREIGN KEY (overridden_condition_id) REFERENCES pf_condition_table(name) ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED
);

CREATE TABLE IF NOT EXISTS sf_condition_overrides_table (
    source_condition_id TEXT NOT NULL,
    overridden_condition_id TEXT NOT NULL,
    PRIMARY KEY (source_condition_id, overridden_condition_id),
    FOREIGN KEY (source_condition_id) REFERENCES sf_condition_table(name) ON DELETE CASCADE,
    FOREIGN KEY (overridden_condition_id) REFERENCES sf_condition_table(name) ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED
);

CREATE TABLE IF NOT EXISTS pf_creature_condition_association_table (
    creature_id BIGINT NOT NULL,
    condition_id TEXT NOT NULL,
    PRIMARY KEY (creature_id, condition_id),
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id) ON DELETE CASCADE,
    FOREIGN KEY (condition_id) REFERENCES pf_condition_table(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_creature_condition_association_table (
    creature_id BIGINT NOT NULL,
    condition_id TEXT NOT NULL,
    PRIMARY KEY (creature_id, condition_id),
    FOREIGN KEY (creature_id) REFERENCES sf_creature_table(id) ON DELETE CASCADE,
    FOREIGN KEY (condition_id) REFERENCES sf_condition_table(name) ON DELETE CASCADE
)