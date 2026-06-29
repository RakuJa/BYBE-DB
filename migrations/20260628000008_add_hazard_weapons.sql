CREATE TABLE IF NOT EXISTS pf_weapon_hazard_association_table (
    hazard_id BIGINT NOT NULL,
    weapon_id   BIGINT NOT NULL,
    quantity    INTEGER NOT NULL,
    PRIMARY KEY (hazard_id, weapon_id, quantity),
    FOREIGN KEY (hazard_id) REFERENCES pf_hazard_table(id) ON DELETE CASCADE,
    FOREIGN KEY (weapon_id)   REFERENCES pf_weapon_table(id)   ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_weapon_hazard_association_table (
    hazard_id BIGINT NOT NULL,
    weapon_id   BIGINT NOT NULL,
    quantity    INTEGER NOT NULL,
    PRIMARY KEY (hazard_id, weapon_id, quantity),
    FOREIGN KEY (hazard_id) REFERENCES sf_hazard_table(id) ON DELETE CASCADE,
    FOREIGN KEY (weapon_id)   REFERENCES sf_weapon_table(id)   ON DELETE CASCADE
);