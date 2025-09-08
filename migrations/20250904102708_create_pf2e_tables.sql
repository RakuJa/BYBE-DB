CREATE TABLE pf2e_trait_table (
    name TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE pf2e_action_table (
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
    rarity TEXT NOT NULL,
    creature_id INTEGER NOT NULL,
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id)
);

CREATE TABLE pf2e_creature_table (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    aon_id INTEGER,
    charisma INTEGER NOT NULL,
    constitution INTEGER NOT NULL,
    dexterity INTEGER NOT NULL,
    intelligence INTEGER NOT NULL,
    strength INTEGER NOT NULL,
    wisdom INTEGER NOT NULL,
    ac INTEGER NOT NULL,
    hp INTEGER NOT NULL,
    hp_detail TEXT NOT NULL,
    ac_detail TEXT NOT NULL,
    language_detail TEXT,
    level INTEGER NOT NULL,
    license TEXT NOT NULL,
    remaster BOOLEAN NOT NULL,
    source TEXT NOT NULL,
    initiative_ability TEXT NOT NULL,
    perception INTEGER NOT NULL,
    perception_detail TEXT NOT NULL,
    vision BOOLEAN NOT NULL,
    fortitude INTEGER NOT NULL,
    reflex INTEGER NOT NULL,
    will INTEGER NOT NULL,
    fortitude_detail TEXT NOT NULL,
    reflex_detail TEXT NOT NULL,
    will_detail TEXT NOT NULL,
    rarity TEXT NOT NULL,
    size TEXT NOT NULL,
    cr_type TEXT,
    family TEXT,
    n_of_focus_points INTEGER NOT NULL,
    UNIQUE(
        name, charisma, constitution, dexterity, intelligence,
        strength, wisdom, ac, hp, level,
        license, remaster, source, rarity, size
    )
);

CREATE TABLE pf2e_immunity_table (
    name TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE pf2e_language_table (
    name TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE pf2e_sense_table (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    range INTEGER,
    acuity TEXT
);

CREATE TABLE pf2e_skill_table (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    modifier INTEGER NOT NULL,
    proficiency INTEGER NOT NULL,
    license TEXT NOT NULL,
    remaster BOOLEAN NOT NULL,
    source TEXT NOT NULL,
    creature_id INTEGER NOT NULL,
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id)
);

CREATE TABLE pf2e_spellcasting_entry_table (
    id INTEGER PRIMARY KEY NOT NULL,
    spellcasting_name TEXT NOT NULL,
    is_spellcasting_flexible BOOLEAN,
    type_of_spellcaster TEXT NOT NULL,
    spellcasting_dc_mod INTEGER NOT NULL,
    spellcasting_atk_mod INTEGER NOT NULL,
    spellcasting_tradition TEXT NOT NULL,
    heighten_level INTEGER NOT NULL,
    creature_id INTEGER NOT NULL,
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id)
);

CREATE TABLE pf2e_tradition_table (
    name TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE pf2e_spell_table (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    area_type TEXT,
    area_value INTEGER,
    counteraction BOOLEAN NOT NULL,
    saving_throw_is_basic BOOLEAN,
    saving_throw_statistic TEXT,
    sustained BOOLEAN NOT NULL,
    duration TEXT,
    level INTEGER NOT NULL,
    range TEXT NOT NULL,
    target TEXT NOT NULL,
    action TEXT NOT NULL,
    license TEXT NOT NULL,
    remaster BOOLEAN NOT NULL,
    source TEXT NOT NULL,
    rarity TEXT NOT NULL,
    slot INTEGER NOT NULL,
    creature_id INTEGER NOT NULL,
    spellcasting_entry_id INTEGER NOT NULL,
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id),
    FOREIGN KEY (spellcasting_entry_id) REFERENCES pf2e_spellcasting_entry_table(id)
);

CREATE TABLE pf2e_armor_creature_association_table (
    creature_id INTEGER NOT NULL,
    armor_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    PRIMARY KEY (creature_id, armor_id, quantity),
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id),
    FOREIGN KEY (armor_id) REFERENCES pf2e_armor_table(id) ON UPDATE CASCADE
);

CREATE TABLE pf2e_immunity_creature_association_table (
    creature_id INTEGER NOT NULL,
    immunity_id TEXT NOT NULL,
    PRIMARY KEY (creature_id, immunity_id),
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id),
    FOREIGN KEY (immunity_id) REFERENCES pf2e_immunity_table(name)
);

CREATE TABLE pf2e_item_creature_association_table (
    creature_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    PRIMARY KEY (creature_id, item_id, quantity),
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id),
    FOREIGN KEY (item_id) REFERENCES pf2e_item_table(id) ON UPDATE CASCADE
);

CREATE TABLE pf2e_language_creature_association_table (
    creature_id INTEGER NOT NULL,
    language_id TEXT NOT NULL,
    PRIMARY KEY (creature_id, language_id),
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id),
    FOREIGN KEY (language_id) REFERENCES pf2e_language_table(name)
);

CREATE TABLE pf2e_resistance_double_vs_table (
    resistance_id INTEGER NOT NULL,
    vs_name TEXT NOT NULL,
    PRIMARY KEY (resistance_id, vs_name),
    FOREIGN KEY (resistance_id) REFERENCES pf2e_resistance_table(id)
);

CREATE TABLE pf2e_resistance_exception_vs_table (
    resistance_id INTEGER NOT NULL,
    vs_name TEXT NOT NULL,
    PRIMARY KEY (resistance_id, vs_name),
    FOREIGN KEY (resistance_id) REFERENCES pf2e_resistance_table(id)
);

CREATE TABLE pf2e_resistance_table (
    id INTEGER PRIMARY KEY NOT NULL,
    creature_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    value INTEGER NOT NULL,
    UNIQUE (creature_id, name),
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id)
);

CREATE TABLE pf2e_sense_creature_association_table (
    creature_id INTEGER NOT NULL,
    sense_id INTEGER NOT NULL,
    PRIMARY KEY (creature_id, sense_id),
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id),
    FOREIGN KEY (sense_id) REFERENCES pf2e_sense_table(id)
);

CREATE TABLE pf2e_shield_creature_association_table (
    creature_id INTEGER NOT NULL,
    shield_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    PRIMARY KEY (creature_id, shield_id, quantity),
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id),
    FOREIGN KEY (shield_id) REFERENCES pf2e_shield_table(id) ON UPDATE CASCADE
);

CREATE TABLE pf2e_creature_skill_label_table (
    creature_id INTEGER NOT NULL,
    skill_id INTEGER NOT NULL,
    skill_label TEXT NOT NULL,
    PRIMARY KEY (creature_id, skill_id, skill_label),
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id),
    FOREIGN KEY (skill_id) REFERENCES pf2e_skill_table(id)
);

CREATE TABLE pf2e_speed_table (
    creature_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    value INTEGER NOT NULL,
    PRIMARY KEY (creature_id, name),
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id)
);

CREATE TABLE pf2e_tradition_spell_association_table (
    spell_id INTEGER NOT NULL,
    tradition_id TEXT NOT NULL,
    PRIMARY KEY (spell_id, tradition_id),
    FOREIGN KEY (spell_id) REFERENCES pf2e_spell_table(id),
    FOREIGN KEY (tradition_id) REFERENCES pf2e_tradition_table(name)
);

CREATE TABLE pf2e_trait_action_association_table (
    action_id INTEGER NOT NULL,
    trait_id TEXT NOT NULL,
    PRIMARY KEY (action_id, trait_id),
    FOREIGN KEY (action_id) REFERENCES pf2e_action_table(id),
    FOREIGN KEY (trait_id) REFERENCES pf2e_trait_table(name)
);

CREATE TABLE pf2e_trait_creature_association_table (
    creature_id INTEGER NOT NULL,
    trait_id TEXT NOT NULL,
    PRIMARY KEY (creature_id, trait_id),
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id),
    FOREIGN KEY (trait_id) REFERENCES pf2e_trait_table(name)
);

CREATE TABLE pf2e_trait_spell_association_table (
    spell_id INTEGER NOT NULL,
    trait_id TEXT NOT NULL,
    PRIMARY KEY (spell_id, trait_id),
    FOREIGN KEY (spell_id) REFERENCES pf2e_spell_table(id),
    FOREIGN KEY (trait_id) REFERENCES pf2e_trait_table(name)
);

CREATE TABLE pf2e_weakness_table (
    creature_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    value INTEGER NOT NULL,
    PRIMARY KEY (creature_id, name),
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id)
);

CREATE TABLE pf2e_weapon_creature_association_table (
    creature_id INTEGER NOT NULL,
    weapon_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    PRIMARY KEY (creature_id, weapon_id, quantity),
    FOREIGN KEY (creature_id) REFERENCES pf2e_creature_table(id),
    FOREIGN KEY (weapon_id) REFERENCES pf2e_weapon_table(id) ON UPDATE CASCADE
);

CREATE TABLE pf2e_item_table (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    bulk REAL NOT NULL,
    base_item TEXT,
    category TEXT,
    description TEXT NOT NULL COLLATE NOCASE,
    hardness INTEGER NOT NULL,
    hp INTEGER NOT NULL,
    level INTEGER NOT NULL,
    price INTEGER NOT NULL,
    usage TEXT,
    item_group TEXT,
    item_type TEXT NOT NULL,
    is_derived BOOL NOT NULL,
    material_grade TEXT,
    material_type TEXT,
    number_of_uses INTEGER,

    license TEXT NOT NULL,
    remaster BOOL NOT NULL,
    source TEXT NOT NULL,

    rarity TEXT NOT NULL,
    size TEXT NOT NULL,

    UNIQUE(
        name, bulk, description COLLATE NOCASE, hardness, hp, level, price,
        item_type, license, remaster, source, rarity, size, is_derived
    ) ON CONFLICT ABORT
);

CREATE TABLE pf2e_trait_item_association_table (
    item_id INTEGER NOT NULL,
    trait_id TEXT NOT NULL,
    PRIMARY KEY (item_id, trait_id),
    FOREIGN KEY (item_id) REFERENCES pf2e_item_table(id),
    FOREIGN KEY (trait_id) REFERENCES pf2e_trait_table(name)
);

CREATE TABLE pf2e_trait_weapon_association_table (
    weapon_id INTEGER NOT NULL,
    trait_id TEXT NOT NULL,
    PRIMARY KEY (weapon_id, trait_id),
    FOREIGN KEY (weapon_id) REFERENCES pf2e_weapon_table(id),
    FOREIGN KEY (trait_id) REFERENCES pf2e_trait_table(name)
);

CREATE TABLE pf2e_trait_shield_association_table (
    shield_id INTEGER NOT NULL,
    trait_id TEXT NOT NULL,
    PRIMARY KEY (shield_id, trait_id),
    FOREIGN KEY (shield_id) REFERENCES pf2e_shield_table(id),
    FOREIGN KEY (trait_id) REFERENCES pf2e_trait_table(name)
);

CREATE TABLE pf2e_trait_armor_association_table (
    armor_id INTEGER NOT NULL,
    trait_id TEXT NOT NULL,
    PRIMARY KEY (armor_id, trait_id),
    FOREIGN KEY (armor_id) REFERENCES pf2e_armor_table(id),
    FOREIGN KEY (trait_id) REFERENCES pf2e_trait_table(name)
);

CREATE TABLE pf2e_weapon_table (
    id INTEGER PRIMARY KEY NOT NULL,

    to_hit_bonus INTEGER,
    splash_dmg INTEGER,

    n_of_potency_runes INTEGER NOT NULL,
    n_of_striking_runes INTEGER NOT NULL,
    range INTEGER,
    reload TEXT,
    weapon_type TEXT NOT NULL,

    base_item_id INTEGER,
    FOREIGN KEY (base_item_id) REFERENCES pf2e_item_table(id) ON UPDATE CASCADE
);

CREATE TABLE pf2e_weapon_damage_table (
    id INTEGER PRIMARY KEY NOT NULL,

    bonus_dmg INTEGER NOT NULL,
    dmg_type TEXT,
    number_of_dice INTEGER,
    die_size INTEGER,

    weapon_id INTEGER NOT NULL,
    FOREIGN KEY (weapon_id) REFERENCES pf2e_weapon_table(id) ON UPDATE CASCADE
);

CREATE TABLE pf2e_armor_table (
    id INTEGER PRIMARY KEY NOT NULL,

    bonus_ac INTEGER NOT NULL,
    check_penalty INTEGER NOT NULL,
    dex_cap INTEGER NOT NULL,
    n_of_potency_runes INTEGER NOT NULL,
    n_of_resilient_runes INTEGER NOT NULL,
    speed_penalty INTEGER NOT NULL,
    strength_required INTEGER,

    base_item_id INTEGER,
    FOREIGN KEY (base_item_id) REFERENCES pf2e_item_table(id) ON UPDATE CASCADE
);

CREATE TABLE pf2e_shield_table (
    id INTEGER PRIMARY KEY NOT NULL,

    bonus_ac INTEGER NOT NULL,

    n_of_reinforcing_runes INTEGER NOT NULL,

    speed_penalty INTEGER NOT NULL,

    base_item_id INTEGER,
    FOREIGN KEY (base_item_id) REFERENCES pf2e_item_table(id) ON UPDATE CASCADE
);

CREATE TABLE pf2e_rune_table (
    name TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE pf2e_rune_weapon_association_table (
    weapon_id INTEGER NOT NULL,
    rune_id TEXT NOT NULL,
    PRIMARY KEY (weapon_id, rune_id),
    FOREIGN KEY (weapon_id) REFERENCES pf2e_weapon_table(id),
    FOREIGN KEY (rune_id) REFERENCES pf2e_rune_table(name)
);

CREATE TABLE pf2e_rune_armor_association_table (
    armor_id INTEGER NOT NULL,
    rune_id TEXT NOT NULL,
    PRIMARY KEY (armor_id, rune_id),
    FOREIGN KEY (armor_id) REFERENCES pf2e_armor_table(id),
    FOREIGN KEY (rune_id) REFERENCES pf2e_rune_table(name)
);
