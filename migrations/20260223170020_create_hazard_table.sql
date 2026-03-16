CREATE TABLE IF NOT EXISTS pf_hazard_table (
    id INTEGER PRIMARY KEY NOT NULL,
    foundry_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,

    ac INTEGER NOT NULL,
    hardness INTEGER NOT NULL,
    has_health BOOLEAN NOT NULL,
    hp INTEGER NOT NULL,
    stealth INTEGER NOT NULL,
    stealth_detail TEXT NOT NULL,

    description TEXT NOT NULL,
    disable_description TEXT NOT NULL,
    reset_description TEXT NOT NULL,
    routine_description TEXT NOT NULL,
    is_complex BOOLEAN NOT NULL,
    level INTEGER NOT NULL,
    license TEXT NOT NULL,
    remaster BOOLEAN NOT NULL,
    source TEXT NOT NULL,

    fortitude INTEGER,
    reflex INTEGER,
    will INTEGER,
    fortitude_detail TEXT NOT NULL,
    reflex_detail TEXT NOT NULL,
    will_detail TEXT NOT NULL,

    rarity TEXT NOT NULL,
    size TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS pf_action_hazard_association_table (
    action_id INTEGER NOT NULL,
    hazard_id TEXT NOT NULL,
    PRIMARY KEY (action_id, hazard_id),
    FOREIGN KEY (action_id) REFERENCES pf_action_table(id) ON DELETE CASCADE,
    FOREIGN KEY (hazard_id) REFERENCES pf_hazard_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_trait_hazard_association_table (
    hazard_id INTEGER NOT NULL,
    trait_id TEXT NOT NULL,
    PRIMARY KEY (hazard_id, trait_id),
    FOREIGN KEY (hazard_id) REFERENCES pf_hazard_table(id) ON DELETE CASCADE,
    FOREIGN KEY (trait_id) REFERENCES pf_trait_table(name) ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS sf_hazard_table (
    id INTEGER PRIMARY KEY NOT NULL,
    foundry_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,

    ac INTEGER NOT NULL,
    hardness INTEGER NOT NULL,
    has_health BOOLEAN NOT NULL,
    hp INTEGER NOT NULL,
    stealth INTEGER NOT NULL,
    stealth_detail TEXT NOT NULL,

    description TEXT NOT NULL,
    disable_description TEXT NOT NULL,
    reset_description TEXT NOT NULL,
    routine_description TEXT NOT NULL,
    is_complex BOOLEAN NOT NULL,
    level INTEGER NOT NULL,
    license TEXT NOT NULL,
    remaster BOOLEAN NOT NULL,
    source TEXT NOT NULL,

    fortitude INTEGER,
    reflex INTEGER,
    will INTEGER,
    fortitude_detail TEXT NOT NULL,
    reflex_detail TEXT NOT NULL,
    will_detail TEXT NOT NULL,

    rarity TEXT NOT NULL,
    size TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sf_action_hazard_association_table (
    action_id INTEGER NOT NULL,
    hazard_id TEXT NOT NULL,
    PRIMARY KEY (action_id, hazard_id),
    FOREIGN KEY (action_id) REFERENCES sf_action_table(id) ON DELETE CASCADE,
    FOREIGN KEY (hazard_id) REFERENCES sf_hazard_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_trait_hazard_association_table (
    hazard_id INTEGER NOT NULL,
    trait_id TEXT NOT NULL,
    PRIMARY KEY (hazard_id, trait_id),
    FOREIGN KEY (hazard_id) REFERENCES sf_hazard_table(id) ON DELETE CASCADE,
    FOREIGN KEY (trait_id) REFERENCES sf_trait_table(name) ON DELETE CASCADE
);
