DROP TABLE pf_weapon_attack_effect_table;
DROP TABLE sf_weapon_attack_effect_table;

CREATE TABLE IF NOT EXISTS pf_weapon_attack_effect_table (
    weapon_id   BIGINT NOT NULL,
    effect_name TEXT NOT NULL,
    PRIMARY KEY (weapon_id, effect_name),
    FOREIGN KEY (weapon_id) REFERENCES pf_weapon_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sf_weapon_attack_effect_table (
    weapon_id   BIGINT NOT NULL,
    effect_name TEXT NOT NULL,
    PRIMARY KEY (weapon_id, effect_name),
    FOREIGN KEY (weapon_id) REFERENCES sf_weapon_table(id) ON DELETE CASCADE
);