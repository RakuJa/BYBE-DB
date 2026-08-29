CREATE TABLE IF NOT EXISTS pf_action_aura_table (
    id        BIGSERIAL PRIMARY KEY,
    radius    INTEGER,
    slug      TEXT,
    action_id BIGINT NOT NULL,
    FOREIGN KEY (action_id) REFERENCES pf_action_table(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX pf_action_aura_no_duplicates
    ON pf_action_aura_table (
        action_id,
        COALESCE(radius, -1),
        COALESCE(slug, '__NULL__')
    );

CREATE TABLE IF NOT EXISTS pf_action_aura_effect_table (
    id              BIGSERIAL PRIMARY KEY,
    effect_name     TEXT,
    affects         TEXT NOT NULL,
    events          TEXT[] NOT NULL DEFAULT '{}',
    removes_on_exit BOOLEAN NOT NULL,
    includes_self   BOOLEAN NOT NULL,
    action_aura_id  BIGINT NOT NULL,
    FOREIGN KEY (action_aura_id) REFERENCES pf_action_aura_table(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX pf_action_aura_effect_no_duplicates
    ON pf_action_aura_effect_table (
        action_aura_id,
        COALESCE(effect_name, '__NULL__'),
        affects,
        events,
        removes_on_exit,
        includes_self
    );

CREATE TABLE IF NOT EXISTS pf_trait_action_aura_association_table (
    action_aura_id BIGINT NOT NULL,
    trait_id       TEXT NOT NULL,
    PRIMARY KEY (action_aura_id, trait_id),
    FOREIGN KEY (action_aura_id) REFERENCES pf_action_aura_table(id) ON DELETE CASCADE,
    FOREIGN KEY (trait_id)       REFERENCES pf_trait_table(name)     ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_action_aura_table (
    id        BIGSERIAL PRIMARY KEY,
    radius    INTEGER,
    slug      TEXT,
    action_id BIGINT NOT NULL,
    FOREIGN KEY (action_id) REFERENCES sf_action_table(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX sf_action_aura_no_duplicates
    ON sf_action_aura_table (
        action_id,
        COALESCE(radius, -1),
        COALESCE(slug, '__NULL__')
    );

CREATE TABLE IF NOT EXISTS sf_action_aura_effect_table (
    id              BIGSERIAL PRIMARY KEY,
    effect_name     TEXT,
    affects         TEXT NOT NULL,
    events          TEXT[] NOT NULL DEFAULT '{}',
    removes_on_exit BOOLEAN NOT NULL,
    includes_self   BOOLEAN NOT NULL,
    action_aura_id  BIGINT NOT NULL,
    FOREIGN KEY (action_aura_id) REFERENCES sf_action_aura_table(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX sf_action_aura_effect_no_duplicates
    ON sf_action_aura_effect_table (
        action_aura_id,
        COALESCE(effect_name, '__NULL__'),
        affects,
        events,
        removes_on_exit,
        includes_self
    );

CREATE TABLE IF NOT EXISTS sf_trait_action_aura_association_table (
    action_aura_id BIGINT NOT NULL,
    trait_id       TEXT NOT NULL,
    PRIMARY KEY (action_aura_id, trait_id),
    FOREIGN KEY (action_aura_id) REFERENCES sf_action_aura_table(id) ON DELETE CASCADE,
    FOREIGN KEY (trait_id)       REFERENCES sf_trait_table(name)     ON DELETE CASCADE
);
