CREATE TABLE IF NOT EXISTS pf_action_movement_change_table (
    id            BIGSERIAL PRIMARY KEY,
    movement_type TEXT NOT NULL,
    value         INTEGER NOT NULL,
    action_id     BIGINT NOT NULL,
    FOREIGN KEY (action_id) REFERENCES pf_action_table(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX pf_action_movement_change_no_duplicates
    ON pf_action_movement_change_table (action_id, movement_type, value);

CREATE TABLE IF NOT EXISTS pf_action_creature_size_table (
    id        BIGSERIAL PRIMARY KEY,
    size      TEXT NOT NULL,
    action_id BIGINT NOT NULL,
    FOREIGN KEY (action_id) REFERENCES pf_action_table(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX pf_action_creature_size_no_duplicates
    ON pf_action_creature_size_table (action_id, size);

CREATE TABLE IF NOT EXISTS sf_action_movement_change_table (
    id            BIGSERIAL PRIMARY KEY,
    movement_type TEXT NOT NULL,
    value         INTEGER NOT NULL,
    action_id     BIGINT NOT NULL,
    FOREIGN KEY (action_id) REFERENCES sf_action_table(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX sf_action_movement_change_no_duplicates
    ON sf_action_movement_change_table (action_id, movement_type, value);

CREATE TABLE IF NOT EXISTS sf_action_creature_size_table (
    id        BIGSERIAL PRIMARY KEY,
    size      TEXT NOT NULL,
    action_id BIGINT NOT NULL,
    FOREIGN KEY (action_id) REFERENCES sf_action_table(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX sf_action_creature_size_no_duplicates
    ON sf_action_creature_size_table (action_id, size);
