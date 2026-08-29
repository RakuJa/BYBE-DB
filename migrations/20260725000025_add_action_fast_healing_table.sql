CREATE TABLE IF NOT EXISTS pf_action_fast_healing_table (
    id             BIGSERIAL PRIMARY KEY,
    healing_type   TEXT,
    value          INTEGER NOT NULL,
    deactivated_by TEXT[] NOT NULL DEFAULT '{}',
    action_id      BIGINT NOT NULL,
    FOREIGN KEY (action_id) REFERENCES pf_action_table(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX pf_action_fast_healing_no_duplicates
    ON pf_action_fast_healing_table (
        action_id,
        COALESCE(healing_type, '__NULL__'),
        value,
        deactivated_by
    );

CREATE TABLE IF NOT EXISTS sf_action_fast_healing_table (
    id             BIGSERIAL PRIMARY KEY,
    healing_type   TEXT,
    value          INTEGER NOT NULL,
    deactivated_by TEXT[] NOT NULL DEFAULT '{}',
    action_id      BIGINT NOT NULL,
    FOREIGN KEY (action_id) REFERENCES sf_action_table(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX sf_action_fast_healing_no_duplicates
    ON sf_action_fast_healing_table (
        action_id,
        COALESCE(healing_type, '__NULL__'),
        value,
        deactivated_by
    );
