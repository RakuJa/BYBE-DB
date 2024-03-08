/*
use crate::schema::db::ability_scales::AbilityScales;
use crate::schema::db::ac_scales::AcScales;
use crate::schema::db::area_dmg_scales::AreaDmgScales;
use crate::schema::db::hp_scales::HpScales;
use crate::schema::db::item_scales::ItemScales;
use crate::schema::db::perception_scales::PerceptionScales;
use crate::schema::db::res_weak_scales::ResWeakScales;
use crate::schema::db::saving_throw_scales::SavingThrowScales;
use crate::schema::db::skill_scales::SkillScales;
use crate::schema::db::spell_dc_and_atk_scales::SpellDcAndAtkScales;
use crate::schema::db::strike_bonus_scales::StrikeBonusScales;
use crate::schema::db::strike_dmg_scales::StrikeDmgScales;
use sqlx::{Sqlite, Transaction};

async fn insert_ability_scales<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    ability_scales: &Vec<AbilityScales>,
    id: i64,
) -> anyhow::Result<bool> {
    for el in ability_scales {
        sqlx::query!(
            "INSERT INTO ABILITY_SCALES_TABLE \
            (level, extreme, high, moderate, low) VALUES ($1, $2, $3, $4, $5)",
            el.level,
            el.extreme,
            el.high,
            el.moderate,
            el.low
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_perception_scales<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    perception_scales: &Vec<PerceptionScales>,
    id: i64,
) -> anyhow::Result<bool> {
    for el in perception_scales {
        sqlx::query!(
            "INSERT INTO PERCEPTION_SCALES_TABLE \
            (level, extreme, high, moderate, low, terrible) VALUES ($1, $2, $3, $4, $5, $6)",
            el.level,
            el.extreme,
            el.high,
            el.moderate,
            el.low,
            el.terrible
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_skill_scales<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    skill_scales: &Vec<SkillScales>,
    id: i64,
) -> anyhow::Result<bool> {
    for el in skill_scales {
        sqlx::query!(
            "INSERT INTO SKILL_SCALES_TABLE \
            (level, extreme, high, moderate, low_ub, low_lb) VALUES ($1, $2, $3, $4, $5, $6)",
            el.level,
            el.extreme,
            el.high,
            el.moderate,
            el.low_ub,
            el.low_lb
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_item_scales<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    item_scales: &Vec<ItemScales>,
    id: i64,
) -> anyhow::Result<bool> {
    for el in item_scales {
        sqlx::query!(
            "INSERT INTO ITEM_SCALES_TABLE \
            (cr_level, safe_item_level) VALUES ($1, $2)",
            el.cr_level,
            el.safe_item_level
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_ac_scales<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    ac_scales: &Vec<AcScales>,
    id: i64,
) -> anyhow::Result<bool> {
    for el in ac_scales {
        sqlx::query!(
            "INSERT INTO AC_SCALES_TABLE \
            (level, extreme, high, moderate, low) VALUES ($1, $2, $3, $4, $5)",
            el.level,
            el.extreme,
            el.high,
            el.moderate,
            el.low
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_saving_throw_scales<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    saving_throw_scales: &Vec<SavingThrowScales>,
    id: i64,
) -> anyhow::Result<bool> {
    for el in saving_throw_scales {
        sqlx::query!(
            "INSERT INTO SAVING_THROW_SCALES_TABLE \
            (level, extreme, high, moderate, low, terrible) VALUES ($1, $2, $3, $4, $5, $6)",
            el.level,
            el.extreme,
            el.high,
            el.moderate,
            el.low,
            el.terrible
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_hp_scales<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    hp_scales: &Vec<HpScales>,
    id: i64,
) -> anyhow::Result<bool> {
    for el in hp_scales {
        sqlx::query!(
            "INSERT INTO HP_SCALES_TABLE \
            (level, high_ub, high_lb, moderate_ub, moderate_lb, low_ub, low_lb) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            el.level, el.high_ub, el.high_lb, el.moderate_ub, el.moderate_lb, el.low_ub, el.low_lb
        )
            .execute(&mut **conn)
            .await?;
    }
    Ok(true)
}

async fn insert_res_scales_scales<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    res_weak_scales: &Vec<ResWeakScales>,
    id: i64,
) -> anyhow::Result<bool> {
    for el in res_weak_scales {
        sqlx::query!(
            "INSERT INTO RES_WEAK_SCALES_TABLE \
            (level, max, min) VALUES ($1, $2, $3)",
            el.level,
            el.max,
            el.min
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_strike_bonus_scales<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    strike_bonus_scales: &Vec<StrikeBonusScales>,
    id: i64,
) -> anyhow::Result<bool> {
    for el in strike_bonus_scales {
        sqlx::query!(
            "INSERT INTO STRIKE_BONUS_SCALES_TABLE \
            (level, extreme, high, moderate, low) VALUES ($1, $2, $3, $4, $5)",
            el.level,
            el.extreme,
            el.high,
            el.moderate,
            el.low
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_strike_dmg_scales<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    strike_dmg_scales: &Vec<StrikeDmgScales>,
    id: i64,
) -> anyhow::Result<bool> {
    for el in strike_dmg_scales {
        sqlx::query!(
            "INSERT INTO STRIKE_DAMAGE_SCALES_TABLE \
            (level, extreme, high, moderate, low) VALUES ($1, $2, $3, $4, $5)",
            el.level,
            el.extreme,
            el.high,
            el.moderate,
            el.low
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

async fn insert_spell_dc_and_atk_scales<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    spell_dc_and_atk_scales: &Vec<SpellDcAndAtkScales>,
    id: i64,
) -> anyhow::Result<bool> {
    for el in spell_dc_and_atk_scales {
        sqlx::query!(
            "INSERT INTO SPELL_DC_AND_ATTACK_SCALES_TABLE \
            (level, extreme_dc, extreme_atk_bonus, high_dc, high_atk_bonus, moderate_dc, moderate_atk_bonus) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            el.level, el.extreme_dc, el.extreme_atk_bonus, el.high_dc, el.high_atk_bonus, el.moderate_dc, el.moderate_atk_bonus
        )
            .execute(&mut **conn)
            .await?;
    }
    Ok(true)
}

async fn insert_area_dmg_scales<'a>(
    conn: &mut Transaction<'a, Sqlite>,
    area_dmg_scales: &Vec<AreaDmgScales>,
    id: i64,
) -> anyhow::Result<bool> {
    for el in area_dmg_scales {
        sqlx::query!(
            "INSERT INTO AREA_DAMAGE_SCALES_TABLE \
            (level, unlimited_use, limited_use) VALUES ($1, $2, $3)",
            el.level,
            el.unlimited_use,
            el.limited_use,
        )
        .execute(&mut **conn)
        .await?;
    }
    Ok(true)
}

 */
