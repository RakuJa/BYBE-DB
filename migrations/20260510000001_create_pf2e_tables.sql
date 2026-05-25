CREATE TABLE IF NOT EXISTS pf_trait_table (
    name TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE IF NOT EXISTS pf_action_table (
    id        BIGSERIAL PRIMARY KEY,
    name      TEXT NOT NULL,
    action_type TEXT NOT NULL,
    n_of_actions INTEGER,
    category  TEXT,
    description TEXT NOT NULL,
    license   TEXT NOT NULL,
    remaster  BOOLEAN NOT NULL,
    source    TEXT NOT NULL,
    slug      TEXT,
    rarity    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS pf_creature_table (
    id                 BIGSERIAL PRIMARY KEY,
    foundry_id         TEXT NOT NULL UNIQUE,
    name               TEXT NOT NULL,
    aon_id             INTEGER,
    charisma           INTEGER NOT NULL,
    constitution       INTEGER NOT NULL,
    dexterity          INTEGER NOT NULL,
    intelligence       INTEGER NOT NULL,
    strength           INTEGER NOT NULL,
    wisdom             INTEGER NOT NULL,
    ac                 INTEGER NOT NULL,
    hp                 INTEGER NOT NULL,
    hp_detail          TEXT NOT NULL,
    ac_detail          TEXT NOT NULL,
    language_detail    TEXT,
    level              INTEGER NOT NULL,
    license            TEXT NOT NULL,
    remaster           BOOLEAN NOT NULL,
    source             TEXT NOT NULL,
    initiative_ability TEXT NOT NULL,
    perception         INTEGER NOT NULL,
    perception_detail  TEXT NOT NULL,
    vision             BOOLEAN NOT NULL,
    fortitude          INTEGER NOT NULL,
    reflex             INTEGER NOT NULL,
    will               INTEGER NOT NULL,
    fortitude_detail   TEXT NOT NULL,
    reflex_detail      TEXT NOT NULL,
    will_detail        TEXT NOT NULL,
    rarity             TEXT NOT NULL,
    size               TEXT NOT NULL,
    cr_type            TEXT,
    family             TEXT,
    n_of_focus_points  INTEGER NOT NULL,
    status             TEXT NOT NULL DEFAULT 'valid',
    CONSTRAINT character_stats UNIQUE (
        name, charisma, constitution, dexterity, intelligence,
        strength, wisdom, ac, hp, level,
        license, remaster, source, rarity, size, status
    )
);

CREATE TABLE IF NOT EXISTS pf_immunity_table (
    name TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE IF NOT EXISTS pf_language_table (
    name TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE IF NOT EXISTS pf_sense_table (
    id     BIGSERIAL PRIMARY KEY,
    name   TEXT NOT NULL,
    range  INTEGER,
    acuity TEXT
);

CREATE TABLE IF NOT EXISTS pf_skill_table (
    id          BIGSERIAL PRIMARY KEY,
    name        TEXT NOT NULL,
    description TEXT,
    modifier    INTEGER NOT NULL,
    proficiency INTEGER NOT NULL,
    license     TEXT NOT NULL,
    remaster    BOOLEAN NOT NULL,
    source      TEXT NOT NULL,
    creature_id BIGINT NOT NULL,
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_spellcasting_entry_table (
    id                       BIGSERIAL PRIMARY KEY,
    spellcasting_name        TEXT NOT NULL,
    is_spellcasting_flexible BOOLEAN,
    type_of_spellcaster      TEXT NOT NULL,
    spellcasting_dc_mod      INTEGER NOT NULL,
    spellcasting_atk_mod     INTEGER NOT NULL,
    spellcasting_tradition   TEXT NOT NULL,
    heighten_level           INTEGER NOT NULL,
    creature_id              BIGINT NOT NULL,
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_spell_table (
    id                   BIGSERIAL PRIMARY KEY,
    name                 TEXT NOT NULL,
    area_type            TEXT,
    area_value           INTEGER,
    counteraction        BOOLEAN NOT NULL,
    saving_throw         TEXT,
    basic_saving_throw   BOOLEAN,
    sustained            BOOLEAN NOT NULL,
    duration             TEXT,
    level                INTEGER NOT NULL,
    range                TEXT,
    target               TEXT,
    actions              TEXT,
    license              TEXT NOT NULL,
    remaster             BOOLEAN NOT NULL,
    source               TEXT NOT NULL,
    rarity               TEXT NOT NULL,
    slot                 INTEGER NOT NULL,
    creature_id          BIGINT NOT NULL,
    spellcasting_entry_id BIGINT NOT NULL,
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id) ON DELETE CASCADE,
    FOREIGN KEY (spellcasting_entry_id) REFERENCES pf_spellcasting_entry_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_tradition_table (
    name TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE IF NOT EXISTS pf_rune_table (
    name TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE IF NOT EXISTS pf_item_table (
    id             BIGSERIAL PRIMARY KEY,
    foundry_id     TEXT NOT NULL UNIQUE,
    name           TEXT NOT NULL,
    bulk           DOUBLE PRECISION NOT NULL,
    base_item      TEXT,
    category       TEXT,
    description    TEXT NOT NULL,
    hardness       INTEGER NOT NULL,
    hp             INTEGER NOT NULL,
    level          INTEGER NOT NULL,
    price          INTEGER NOT NULL,
    usage          TEXT,
    item_group     TEXT,
    item_type      TEXT NOT NULL,
    is_derived     BOOLEAN NOT NULL,
    material_grade TEXT,
    material_type  TEXT,
    number_of_uses INTEGER,
    license        TEXT NOT NULL,
    remaster       BOOLEAN NOT NULL,
    source         TEXT NOT NULL,
    rarity         TEXT NOT NULL,
    size           TEXT NOT NULL,
    status         TEXT NOT NULL DEFAULT 'valid'
);

CREATE UNIQUE INDEX IF NOT EXISTS item_stats ON pf_item_table (
    name, bulk, hardness, hp, level, price,
    item_type, license, remaster, source, rarity, size, is_derived, status, md5(description)
);

CREATE TABLE IF NOT EXISTS pf_weapon_table (
    id                BIGSERIAL PRIMARY KEY,
    to_hit_bonus      INTEGER,
    splash_dmg        INTEGER,
    n_of_potency_runes INTEGER NOT NULL,
    n_of_striking_runes INTEGER NOT NULL,
    range             INTEGER,
    reload            TEXT,
    weapon_type       TEXT NOT NULL,
    base_item_id      BIGINT,
    FOREIGN KEY (base_item_id) REFERENCES pf_item_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_weapon_damage_table (
    id           BIGSERIAL PRIMARY KEY,
    bonus_dmg    INTEGER NOT NULL,
    dmg_type     TEXT,
    number_of_dice INTEGER,
    die_size     INTEGER,
    weapon_id    BIGINT NOT NULL,
    FOREIGN KEY (weapon_id) REFERENCES pf_weapon_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_armor_table (
    id                   BIGSERIAL PRIMARY KEY,
    ac_bonus             INTEGER NOT NULL,
    check_penalty        INTEGER NOT NULL,
    dex_cap              INTEGER NOT NULL,
    n_of_potency_runes   INTEGER NOT NULL,
    n_of_resilient_runes INTEGER NOT NULL,
    speed_penalty        INTEGER NOT NULL,
    strength_required    INTEGER,
    base_item_id         BIGINT,
    FOREIGN KEY (base_item_id) REFERENCES pf_item_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_shield_table (
    id                      BIGSERIAL PRIMARY KEY,
    ac_bonus                INTEGER NOT NULL,
    n_of_reinforcing_runes  INTEGER NOT NULL,
    speed_penalty           INTEGER NOT NULL,
    base_item_id            BIGINT,
    FOREIGN KEY (base_item_id) REFERENCES pf_item_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_resistance_table (
    id          BIGSERIAL PRIMARY KEY,
    creature_id BIGINT NOT NULL,
    name        TEXT NOT NULL,
    value       INTEGER NOT NULL,
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id) ON DELETE CASCADE
);

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

CREATE TABLE IF NOT EXISTS pf_weakness_table (
    creature_id BIGINT NOT NULL,
    name        TEXT NOT NULL,
    value       INTEGER NOT NULL,
    PRIMARY KEY (creature_id, name),
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_speed_table (
    creature_id BIGINT NOT NULL,
    name        TEXT NOT NULL,
    value       INTEGER NOT NULL,
    PRIMARY KEY (creature_id, name),
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_hazard_table (
    id                    BIGSERIAL PRIMARY KEY,
    foundry_id            TEXT NOT NULL UNIQUE,
    name                  TEXT NOT NULL,
    ac                    INTEGER NOT NULL,
    hardness              INTEGER NOT NULL,
    has_health            BOOLEAN NOT NULL,
    hp                    INTEGER NOT NULL,
    stealth               INTEGER NOT NULL,
    stealth_detail        TEXT NOT NULL,
    description           TEXT NOT NULL,
    disable_description   TEXT NOT NULL,
    reset_description     TEXT NOT NULL,
    routine_description   TEXT NOT NULL,
    is_complex            BOOLEAN NOT NULL,
    level                 INTEGER NOT NULL,
    license               TEXT NOT NULL,
    remaster              BOOLEAN NOT NULL,
    source                TEXT NOT NULL,
    fortitude             INTEGER,
    reflex                INTEGER,
    will                  INTEGER,
    fortitude_detail      TEXT NOT NULL,
    reflex_detail         TEXT NOT NULL,
    will_detail           TEXT NOT NULL,
    rarity                TEXT NOT NULL,
    size                  TEXT NOT NULL
);

-- Association tables
CREATE TABLE IF NOT EXISTS pf_weapon_creature_association_table (
    creature_id BIGINT NOT NULL,
    weapon_id   BIGINT NOT NULL,
    quantity    INTEGER NOT NULL,
    PRIMARY KEY (creature_id, weapon_id, quantity),
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id) ON DELETE CASCADE,
    FOREIGN KEY (weapon_id)   REFERENCES pf_weapon_table(id)   ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_armor_creature_association_table (
    creature_id BIGINT NOT NULL,
    armor_id    BIGINT NOT NULL,
    quantity    INTEGER NOT NULL,
    PRIMARY KEY (creature_id, armor_id, quantity),
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id) ON DELETE CASCADE,
    FOREIGN KEY (armor_id)    REFERENCES pf_armor_table(id)    ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_shield_creature_association_table (
    creature_id BIGINT NOT NULL,
    shield_id   BIGINT NOT NULL,
    quantity    INTEGER NOT NULL,
    PRIMARY KEY (creature_id, shield_id, quantity),
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id) ON DELETE CASCADE,
    FOREIGN KEY (shield_id)   REFERENCES pf_shield_table(id)   ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_item_creature_association_table (
    item_id     BIGINT NOT NULL,
    creature_id BIGINT NOT NULL,
    quantity    INTEGER NOT NULL,
    PRIMARY KEY (item_id, creature_id, quantity),
    FOREIGN KEY (item_id)     REFERENCES pf_item_table(id)     ON DELETE CASCADE,
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_creature_action_association_table (
    creature_id BIGINT NOT NULL,
    action_id   BIGINT NOT NULL,
    PRIMARY KEY (creature_id, action_id),
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id) ON DELETE CASCADE,
    FOREIGN KEY (action_id)   REFERENCES pf_action_table(id)   ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_action_hazard_association_table (
    action_id BIGINT NOT NULL,
    hazard_id BIGINT NOT NULL,
    PRIMARY KEY (action_id, hazard_id),
    FOREIGN KEY (action_id) REFERENCES pf_action_table(id)  ON DELETE CASCADE,
    FOREIGN KEY (hazard_id) REFERENCES pf_hazard_table(id)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_trait_creature_association_table (
    creature_id BIGINT NOT NULL,
    trait_id    TEXT NOT NULL,
    PRIMARY KEY (creature_id, trait_id),
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id) ON DELETE CASCADE,
    FOREIGN KEY (trait_id)    REFERENCES pf_trait_table(name)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_trait_action_association_table (
    action_id BIGINT NOT NULL,
    trait_id  TEXT NOT NULL,
    PRIMARY KEY (action_id, trait_id),
    FOREIGN KEY (action_id) REFERENCES pf_action_table(id) ON DELETE CASCADE,
    FOREIGN KEY (trait_id)  REFERENCES pf_trait_table(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_trait_item_association_table (
    item_id  BIGINT NOT NULL,
    trait_id TEXT NOT NULL,
    PRIMARY KEY (item_id, trait_id),
    FOREIGN KEY (item_id)  REFERENCES pf_item_table(id)    ON DELETE CASCADE,
    FOREIGN KEY (trait_id) REFERENCES pf_trait_table(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_trait_weapon_association_table (
    weapon_id BIGINT NOT NULL,
    trait_id  TEXT NOT NULL,
    PRIMARY KEY (weapon_id, trait_id),
    FOREIGN KEY (weapon_id) REFERENCES pf_weapon_table(id) ON DELETE CASCADE,
    FOREIGN KEY (trait_id)  REFERENCES pf_trait_table(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_trait_armor_association_table (
    armor_id BIGINT NOT NULL,
    trait_id TEXT NOT NULL,
    PRIMARY KEY (armor_id, trait_id),
    FOREIGN KEY (armor_id) REFERENCES pf_armor_table(id)   ON DELETE CASCADE,
    FOREIGN KEY (trait_id) REFERENCES pf_trait_table(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_trait_shield_association_table (
    shield_id BIGINT NOT NULL,
    trait_id  TEXT NOT NULL,
    PRIMARY KEY (shield_id, trait_id),
    FOREIGN KEY (shield_id) REFERENCES pf_shield_table(id) ON DELETE CASCADE,
    FOREIGN KEY (trait_id)  REFERENCES pf_trait_table(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_trait_hazard_association_table (
    hazard_id BIGINT NOT NULL,
    trait_id  TEXT NOT NULL,
    PRIMARY KEY (hazard_id, trait_id),
    FOREIGN KEY (hazard_id) REFERENCES pf_hazard_table(id) ON DELETE CASCADE,
    FOREIGN KEY (trait_id)  REFERENCES pf_trait_table(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_trait_spell_association_table (
    spell_id BIGINT NOT NULL,
    trait_id TEXT NOT NULL,
    PRIMARY KEY (spell_id, trait_id),
    FOREIGN KEY (spell_id) REFERENCES pf_spell_table(id)   ON DELETE CASCADE,
    FOREIGN KEY (trait_id) REFERENCES pf_trait_table(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_tradition_spell_association_table (
    spell_id     BIGINT NOT NULL,
    tradition_id TEXT NOT NULL,
    PRIMARY KEY (spell_id, tradition_id),
    FOREIGN KEY (spell_id)     REFERENCES pf_spell_table(id)      ON DELETE CASCADE,
    FOREIGN KEY (tradition_id) REFERENCES pf_tradition_table(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_rune_weapon_association_table (
    weapon_id BIGINT NOT NULL,
    rune_id   TEXT NOT NULL,
    PRIMARY KEY (weapon_id, rune_id),
    FOREIGN KEY (weapon_id) REFERENCES pf_weapon_table(id) ON DELETE CASCADE,
    FOREIGN KEY (rune_id)   REFERENCES pf_rune_table(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_rune_armor_association_table (
    armor_id BIGINT NOT NULL,
    rune_id  TEXT NOT NULL,
    PRIMARY KEY (armor_id, rune_id),
    FOREIGN KEY (armor_id) REFERENCES pf_armor_table(id)   ON DELETE CASCADE,
    FOREIGN KEY (rune_id)  REFERENCES pf_rune_table(name)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_language_creature_association_table (
    creature_id BIGINT NOT NULL,
    language_id TEXT NOT NULL,
    PRIMARY KEY (creature_id, language_id),
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id)  ON DELETE CASCADE,
    FOREIGN KEY (language_id) REFERENCES pf_language_table(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_immunity_creature_association_table (
    creature_id BIGINT NOT NULL,
    immunity_id TEXT NOT NULL,
    PRIMARY KEY (creature_id, immunity_id),
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id)  ON DELETE CASCADE,
    FOREIGN KEY (immunity_id) REFERENCES pf_immunity_table(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_sense_creature_association_table (
    creature_id BIGINT NOT NULL,
    sense_id    BIGINT NOT NULL,
    PRIMARY KEY (creature_id, sense_id),
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id) ON DELETE CASCADE,
    FOREIGN KEY (sense_id)    REFERENCES pf_sense_table(id)    ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_creature_skill_label_table (
    creature_id BIGINT NOT NULL,
    skill_id    BIGINT NOT NULL,
    skill_label TEXT NOT NULL,
    PRIMARY KEY (creature_id, skill_id, skill_label),
    FOREIGN KEY (creature_id) REFERENCES pf_creature_table(id) ON DELETE CASCADE,
    FOREIGN KEY (skill_id)    REFERENCES pf_skill_table(id)    ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pf_weapon_attack_effect_table (
    weapon_id   BIGINT NOT NULL,
    effect_name TEXT NOT NULL,
    PRIMARY KEY (weapon_id, effect_name),
    FOREIGN KEY (weapon_id) REFERENCES pf_weapon_table(id) ON DELETE CASCADE
);
