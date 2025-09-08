CREATE TABLE ability_scales_table (
    id INTEGER PRIMARY KEY NOT NULL,
    level INTEGER UNIQUE NOT NULL,
    extreme INTEGER,
    high INTEGER NOT NULL,
    moderate INTEGER NOT NULL,
    low INTEGER NOT NULL
);

CREATE TABLE perception_scales_table (
    id INTEGER PRIMARY KEY NOT NULL,
    level INTEGER UNIQUE NOT NULL,
    extreme INTEGER NOT NULL,
    high INTEGER NOT NULL,
    moderate INTEGER NOT NULL,
    low INTEGER NOT NULL,
    terrible INTEGER NOT NULL
);

CREATE TABLE skill_scales_table (
    id INTEGER PRIMARY KEY NOT NULL,
    level INTEGER UNIQUE NOT NULL,
    extreme INTEGER NOT NULL,
    high INTEGER NOT NULL,
    moderate INTEGER NOT NULL,
    low_ub INTEGER NOT NULL,
    low_lb INTEGER NOT NULL
);

CREATE TABLE item_scales_table (
    id INTEGER PRIMARY KEY NOT NULL,
    cr_level TEXT UNIQUE NOT NULL,
    safe_item_level TEXT NOT NULL
);

CREATE TABLE ac_scales_table (
    id INTEGER PRIMARY KEY NOT NULL,
    level INTEGER UNIQUE NOT NULL,
    extreme INTEGER NOT NULL,
    high INTEGER NOT NULL,
    moderate INTEGER NOT NULL,
    low INTEGER NOT NULL
);

CREATE TABLE saving_throw_scales_table (
    id INTEGER PRIMARY KEY NOT NULL,
    level INTEGER UNIQUE NOT NULL,
    extreme INTEGER NOT NULL,
    high INTEGER NOT NULL,
    moderate INTEGER NOT NULL,
    low INTEGER NOT NULL,
    terrible INTEGER NOT NULL
);

CREATE TABLE hp_scales_table (
    id INTEGER PRIMARY KEY NOT NULL,
    level INTEGER UNIQUE NOT NULL,
    high_ub INTEGER NOT NULL,
    high_lb INTEGER NOT NULL,
    moderate_ub INTEGER NOT NULL,
    moderate_lb INTEGER NOT NULL,
    low_ub INTEGER NOT NULL,
    low_lb INTEGER NOT NULL
);

CREATE TABLE res_weak_scales_table (
    id INTEGER PRIMARY KEY NOT NULL,
    level INTEGER UNIQUE NOT NULL,
    max INTEGER NOT NULL,
    min INTEGER NOT NULL
);

CREATE TABLE strike_bonus_scales_table (
    id INTEGER PRIMARY KEY NOT NULL,
    level INTEGER UNIQUE NOT NULL,
    extreme INTEGER NOT NULL,
    high INTEGER NOT NULL,
    moderate INTEGER NOT NULL,
    low INTEGER NOT NULL
);

CREATE TABLE strike_damage_scales_table (
    id INTEGER PRIMARY KEY NOT NULL,
    level INTEGER UNIQUE NOT NULL,
    extreme TEXT NOT NULL,
    high TEXT NOT NULL,
    moderate TEXT NOT NULL,
    low TEXT NOT NULL
);

CREATE TABLE spell_dc_and_attack_scales_table (
    id INTEGER PRIMARY KEY NOT NULL,
    level INTEGER UNIQUE  NOT NULL,
    extreme_dc INTEGER NOT NULL,
    extreme_atk_bonus INTEGER NOT NULL,
    high_dc INTEGER NOT NULL,
    high_atk_bonus INTEGER NOT NULL,
    moderate_dc INTEGER NOT NULL,
    moderate_atk_bonus INTEGER NOT NULL
);

CREATE TABLE area_damage_scales_table (
    id INTEGER PRIMARY KEY NOT NULL,
    level INTEGER UNIQUE NOT NULL,
    unlimited_use TEXT NOT NULL,
    limited_use TEXT NOT NULL
);
