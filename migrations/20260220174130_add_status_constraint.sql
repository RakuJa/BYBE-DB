DROP TABLE IF EXISTS pf_creature_table_new;
CREATE TABLE pf_creature_table_new (
    id INTEGER PRIMARY KEY NOT NULL,
    foundry_id TEXT NOT NULL UNIQUE,
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
    status TEXT NOT NULL DEFAULT 'valid',
    CONSTRAINT character_stats UNIQUE(
        name, charisma, constitution, dexterity, intelligence,
        strength, wisdom, ac, hp, level,
        license, remaster, source, rarity, size, status
    )
);
INSERT INTO pf_creature_table_new (
    id, foundry_id, name, aon_id, charisma,
    constitution, dexterity, intelligence, strength,
    wisdom, ac, hp, hp_detail, ac_detail, language_detail,
    level, license, remaster, source, initiative_ability,
    perception, perception_detail, vision, fortitude, reflex,
    will, fortitude_detail, reflex_detail, will_detail, rarity,
    size, cr_type, family, n_of_focus_points
)
SELECT
    id, foundry_id, name, aon_id, charisma, constitution, dexterity, intelligence, strength,
    wisdom, ac, hp, hp_detail, ac_detail, language_detail, level, license, remaster, source,
    initiative_ability, perception, perception_detail, vision, fortitude, reflex, will,
    fortitude_detail, reflex_detail, will_detail, rarity, size, cr_type, family, n_of_focus_points
FROM pf_creature_table;
DROP TABLE pf_creature_table;
ALTER TABLE pf_creature_table_new RENAME TO pf_creature_table;

DROP TABLE IF EXISTS sf_creature_table_new;
CREATE TABLE sf_creature_table_new (
    id INTEGER PRIMARY KEY NOT NULL,
    foundry_id TEXT NOT NULL UNIQUE,
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
    status TEXT NOT NULL DEFAULT 'valid',
    CONSTRAINT character_stats UNIQUE(
        name, charisma, constitution, dexterity, intelligence,
        strength, wisdom, ac, hp, level,
        license, remaster, source, rarity, size, status
    )
);
INSERT INTO sf_creature_table_new (
    id, foundry_id, name, aon_id, charisma,
    constitution, dexterity, intelligence, strength,
    wisdom, ac, hp, hp_detail, ac_detail, language_detail,
    level, license, remaster, source, initiative_ability,
    perception, perception_detail, vision, fortitude, reflex,
    will, fortitude_detail, reflex_detail, will_detail, rarity,
    size, cr_type, family, n_of_focus_points
)
SELECT
    id, foundry_id, name, aon_id, charisma, constitution, dexterity, intelligence, strength,
    wisdom, ac, hp, hp_detail, ac_detail, language_detail, level, license, remaster, source,
    initiative_ability, perception, perception_detail, vision, fortitude, reflex, will,
    fortitude_detail, reflex_detail, will_detail, rarity, size, cr_type, family, n_of_focus_points
FROM sf_creature_table;
DROP TABLE sf_creature_table;
ALTER TABLE sf_creature_table_new RENAME TO sf_creature_table;

DROP TABLE IF EXISTS pf_item_table_new;
CREATE TABLE pf_item_table_new (
    id INTEGER PRIMARY KEY NOT NULL,
    foundry_id TEXT NOT NULL UNIQUE,
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
    status TEXT NOT NULL DEFAULT 'valid',
    CONSTRAINT item_stats UNIQUE(
        name, bulk, description COLLATE NOCASE, hardness, hp, level, price,
        item_type, license, remaster, source, rarity, size, is_derived, status
    )
);
INSERT INTO pf_item_table_new (
    id, foundry_id, name, bulk, base_item, category, description,
    hardness, hp, level, price, usage, item_group, item_type,
    is_derived, material_grade, material_type, number_of_uses,
    license, remaster, source, rarity, size
)
SELECT
    id, foundry_id, name, bulk, base_item, category, description,
    hardness, hp, level, price, usage, item_group, item_type,
    is_derived, material_grade, material_type, number_of_uses,
    license, remaster, source, rarity, size
FROM pf_item_table;
DROP TABLE pf_item_table;
ALTER TABLE pf_item_table_new RENAME TO pf_item_table;

DROP TABLE IF EXISTS sf_item_table_new;
CREATE TABLE sf_item_table_new (
    id INTEGER PRIMARY KEY NOT NULL,
    foundry_id TEXT NOT NULL UNIQUE,
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
    status TEXT NOT NULL DEFAULT 'valid',
    CONSTRAINT item_stats UNIQUE(
        name, bulk, description COLLATE NOCASE, hardness, hp, level, price,
        item_type, license, remaster, source, rarity, size, is_derived, status
   )
);
INSERT INTO sf_item_table_new (
    id, foundry_id, name, bulk, base_item, category, description,
    hardness, hp, level, price, usage, item_group, item_type,
    is_derived, material_grade, material_type, number_of_uses,
    license, remaster, source, rarity, size
)
SELECT
    id, foundry_id, name, bulk, base_item, category, description,
    hardness, hp, level, price, usage, item_group, item_type,
    is_derived, material_grade, material_type, number_of_uses,
    license, remaster, source, rarity, size
FROM sf_item_table;
DROP TABLE sf_item_table;
ALTER TABLE sf_item_table_new RENAME TO sf_item_table;