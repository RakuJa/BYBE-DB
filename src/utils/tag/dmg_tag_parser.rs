use crate::utils::tag::tag_parser::{get_dice_inside_string, remove_all_dices_from_description};
use evalexpr::eval;
use {once_cell::sync::Lazy, regex::Regex};

pub fn clean_description(description: &str, item_lvl: Option<i64>) -> String {
    let mut desc = String::from(description);
    // Matches @Damage[...] with one level of nested brackets (e.g. [bludgeoning]),
    // an optional extra trailing ] (some entries have a stray one), and optional {label}.
    // Group 1: full {label}, Group 2: label content without braces.
    static SPLIT_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"@Damage\[(?:[^\[\]]*(?:\[[^\[\]]*\])?)*\]\]?(\{([^}]*)\})?").unwrap()
    });
    for curr_match in SPLIT_REGEX.captures_iter(description) {
        let dirty_match = curr_match.get(0).unwrap().as_str();
        let cleaned_match = if let Some(curly_bracket_content) =
            curr_match.get(2).map(|x| x.as_str())
        {
            curly_bracket_content.to_string()
        } else {
            let inner = extract_dmg_inner_content(dirty_match);
            if has_top_level_comma(inner) {
                clean_multi_damage(inner)
            } else if inner.contains("@actor.") {
                clean_actor_level_damage(inner)
            } else {
                let item_cleaned = clean_item_rank_dmg_tag(dirty_match, item_lvl.unwrap_or(0));
                clean_generic_dmg_tag(&item_cleaned)
            }
        };
        desc = desc.replacen(dirty_match, &cleaned_match, 1);
    }
    desc
}

/// Extracts the content between `@Damage[` and its matching closing `]`.
fn extract_dmg_inner_content(dirty_match: &str) -> &str {
    const PREFIX: &str = "@Damage[";
    let rest = &dirty_match[PREFIX.len()..];
    let mut depth = 0i32;
    for (i, c) in rest.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => {
                if depth == 0 {
                    return &rest[..i];
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    rest
}

/// Returns true if `s` contains a `,` at bracket depth 0 (i.e. a top-level separator between
/// multiple damage entries such as `7d6[fire],4d12[persistent,poison]`).
fn has_top_level_comma(s: &str) -> bool {
    let mut depth = 0i32;
    for c in s.chars() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' if depth == 0 => return true,
            _ => {}
        }
    }
    false
}

/// Parses a multi-damage inner string like `7d6[fire],4d12[persistent,poison]|options:area-damage`
/// into a human-readable form like `7d6 fire, 4d12 persistent poison`.
fn clean_multi_damage(inner: &str) -> String {
    // Strip any top-level pipe suffix (|options:..., |traits:...).
    let content = {
        let mut depth = 0i32;
        let mut cut = inner.len();
        for (i, c) in inner.char_indices() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                '|' if depth == 0 => {
                    cut = i;
                    break;
                }
                _ => {}
            }
        }
        &inner[..cut]
    };

    // Split by ',' at depth 0.
    let mut parts: Vec<&str> = Vec::new();
    let mut start = 0;
    let mut depth = 0i32;
    for (i, c) in content.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(content[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(content[start..].trim());

    parts
        .iter()
        .filter(|p| !p.is_empty())
        .map(|p| parse_single_dmg_part(p))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Parses one damage segment like `7d6[fire]` or `4d12[persistent,poison]` into `7d6 fire` /
/// `4d12 persistent poison`.
fn parse_single_dmg_part(part: &str) -> String {
    if let Some(bracket_start) = part.find('[') {
        let dice = part[..bracket_start].trim();
        let type_raw = part[bracket_start + 1..].trim_end_matches(']');
        let dmg_type = type_raw.replace(',', " ");
        format!("{dice} {dmg_type}")
    } else {
        part.trim().to_string()
    }
}

/// Handles `@Damage[...]` inner content that references `@actor.level` or `@actor.rank`.
/// Substitutes those tokens with "Actor Level" and extracts the formula and damage type.
fn clean_actor_level_damage(inner: &str) -> String {
    // Strip top-level pipe options (e.g. |options:area-damage)
    let content = {
        let mut depth = 0i32;
        let mut cut = inner.len();
        for (i, c) in inner.char_indices() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                '|' if depth == 0 => {
                    cut = i;
                    break;
                }
                _ => {}
            }
        }
        &inner[..cut]
    };

    let content = content
        .replace("@actor.level", "Actor Level")
        .replace("@actor.rank", "Actor Level");

    // Find the last top-level '[type]' to separate formula from damage type
    let mut depth = 0i32;
    let mut last_open: Option<usize> = None;
    for (i, c) in content.char_indices() {
        match c {
            '[' => {
                if depth == 0 {
                    last_open = Some(i);
                }
                depth += 1;
            }
            ']' => depth -= 1,
            _ => {}
        }
    }

    if let Some(pos) = last_open {
        let formula = content[..pos].trim();
        let type_raw = content[pos + 1..].trim_end_matches(']');
        let dmg_type = type_raw.replace(',', " ");
        if dmg_type.is_empty() {
            formula.to_string()
        } else {
            format!("{formula} {dmg_type}")
        }
    } else {
        content
    }
}

fn clean_item_rank_dmg_tag(description: &str, item_lvl: i64) -> String {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"@Damage\[(.*[\[|(])?(@item\.(rank|level).*)([]|)])?\[([^]]*)](.*])").unwrap()
    });
    let mut clean_description = String::from(description);
    for curr_match in RE.captures_iter(description) {
        let raw_dmg_str = curr_match.get(0).unwrap().as_str();
        let curr_dmg_str = if let Some(second) = curr_match.get(2) {
            let mut dmg_str = String::new();
            // explanation of the logic with the given entry
            // @Damage[(2*(@item.level - 2))[fire,persistent]]
            // 0 => @Damage[
            // 1 => everything before @item => (2*(
            // 2 => @item.level - 2))
            // 3 => level (or rank if item.rank)
            // 5 => fire,persistent
            // 6 => ] (tailing garbage)
            // So we substitute item.level/rank with the level, and fill
            // the parenthesis if they are missing. Then we evaluate the result
            let first = curr_match.get(1).map(|x| x.as_str()).unwrap_or("");
            let match_str = format!("{}{}", first, second.as_str());
            let n_of_open_brackets = match_str.matches("(").count();
            let n_of_closed_brackets = match_str.matches(")").count();
            let filler = if n_of_closed_brackets > n_of_open_brackets {
                "(".repeat(n_of_closed_brackets - n_of_open_brackets)
            } else {
                ")".repeat(n_of_open_brackets - n_of_closed_brackets)
            };
            let to_replace = format!(
                "@item.{}",
                curr_match.get(3).map(|x| x.as_str()).unwrap_or("rank")
            );
            let to_evaluate = remove_all_dices_from_description(
                format!("{filler}{match_str}")
                    .replace(to_replace.as_str(), item_lvl.to_string().as_str())
                    .as_str(),
            );
            let dmg = eval(to_evaluate.as_str());
            dmg_str.push_str(dmg.unwrap().as_int().unwrap().to_string().as_str());
            if let Some(dice) = get_dice_inside_string(second.as_str()) {
                dmg_str.push_str(dice.to_string().as_str());
            };
            if let Some(dmg_type) = curr_match.get(5).map(|x| parse_dmg_type(x.as_str())) {
                if !dmg_type.is_empty() {
                    dmg_str.push(' ');
                }
                dmg_str.push_str(dmg_type.as_str());
            }
            dmg_str
        } else {
            String::new()
        };
        clean_description = clean_description.replace(raw_dmg_str, &curr_dmg_str);
    }
    clean_description
}

fn parse_dmg_type(raw_dmg_type: &str) -> String {
    // Remove parens and closing brackets; [ is kept as a fallback separator (see below)
    let cleaned = raw_dmg_type.replace(&['(', ')', ']'][..], "");
    // Types are separated by , (e.g. persistent,acid) or by [ (e.g. type1[type2 after ] removal)
    let sep = if cleaned.contains(',') { ',' } else { '[' };
    cleaned.split(sep).collect::<Vec<_>>().join(" ").replace('[', "").trim().to_string()
}
fn clean_generic_dmg_tag(description: &str) -> String {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"@Damage\[\(?([\w+\-\s]*)\)?\[?([^\s-]*])(\{.*})?(.*)").unwrap());
    let mut clean_description = String::from(description);
    for curr_match in RE.captures_iter(description) {
        let raw_dmg_str = curr_match.get(0).unwrap().as_str();
        // Curly bracket label takes priority: {hi osi} => hi osi
        let curr_dmg_str = if let Some(curly) = curr_match.get(3).map(|x| x.as_str()) {
            curly.replace(&['{', '}'][..], "")
        } else {
            let dmg_dice = curr_match.get(1).map(|x| x.as_str().trim()).unwrap_or("0");
            let mut dmg_str = dmg_dice.to_string();
            if let Some(dmg_type) = curr_match.get(2).map(|x| parse_dmg_type(x.as_str())) {
                if !dmg_type.is_empty() {
                    dmg_str.push(' ');
                }
                dmg_str.push_str(&dmg_type);
            }
            dmg_str
        };
        clean_description = clean_description.replace(raw_dmg_str, &curr_dmg_str);
    }
    clean_description
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("@Damage[1d6[bleed]]", "1d6 bleed")]
    #[case("@Damage[1d8[slashing]]", "1d8 slashing")]
    #[case("@Damage[3d6[bleed]]", "3d6 bleed")]
    #[case("@Damage[3d6[bludgeoning]]", "3d6 bludgeoning")]
    #[case("@Damage[1d6[bludgeoning]]", "1d6 bludgeoning")]
    #[case("@Damage[1d12[electricity]]", "1d12 electricity")]
    #[case(
        "take @Damage[1d12[electricity]] damage",
        "take 1d12 electricity damage"
    )]
    #[case("@Damage[(2d8+4)[electricity]]", "2d8+4 electricity")]
    #[case("@Damage[(1d10-1)[piercing]]", "1d10-1 piercing")]
    #[case("@Damage[(1d12 + 3)[bludgeoning]]", "1d12 + 3 bludgeoning")]
    fn clean_damage_string_with_one_type(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, None);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("@Damage[9d6[persistent,mental]]", "9d6 persistent mental")]
    #[case("@Damage[1d6[persistent,untyped]]", "1d6 persistent untyped")]
    #[case("@Damage[3d4[persistent,acid]]", "3d4 persistent acid")]
    fn clean_damage_string_with_one_type_and_one_trait(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input, None);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("@Damage[(4[splash])[force]]{4 splash force}", "4 splash force")]
    #[case("@Damage[(4[splash])[fire]]{4 splash fire}", "4 splash fire")]
    fn clean_damage_string_with_one_type_and_one_trait_and_bracket_descr(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input, None);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("@Damage[(@item.level)d6[slashing]]", Some(4), "4d6 slashing")]
    #[case("@Damage[(@item.level)d4[bludgeoning]]", Some(4), "4d4 bludgeoning")]
    #[case("@Damage[(@item.level)d6[mental]]", Some(4), "4d6 mental")]
    #[case("@Damage[(@item.level)d6[slashing]]", None, "0d6 slashing")]
    #[case("@Damage[(@item.level)d4[bludgeoning]]", None, "0d4 bludgeoning")]
    #[case("@Damage[(@item.level)d6[mental]]", None, "0d6 mental")]
    fn clean_damage_string_with_item_level_and_dmg_and_dmg_type(
        #[case] input: &str,
        #[case] input_lvl: Option<i64>,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input, input_lvl);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("@Damage[@item.level[fire]]", Some(4), "4 fire")]
    #[case("@Damage[@item.level[mental]]", Some(4), "4 mental")]
    #[case("@Damage[@item.level[vitality]]", Some(4), "4 vitality")]
    #[case("@Damage[@item.level[void]]", Some(4), "4 void")]
    #[case("@Damage[@item.level[fire]]", None, "0 fire")]
    #[case("@Damage[@item.level[mental]]", None, "0 mental")]
    #[case("@Damage[@item.level[vitality]]", None, "0 vitality")]
    #[case("@Damage[@item.level[void]]", None, "0 void")]
    fn clean_damage_string_with_item_level_and_no_dmg_and_dmg_type(
        #[case] input: &str,
        #[case] input_lvl: Option<i64>,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input, input_lvl);
        assert_eq!(expected, parsed_description);
    }

    //CARE: Item rank is usually reserved for spell ranks, so it's not really an item.
    // Foundry, WTH.
    #[rstest]
    #[case(
        "@Damage[@item.rank[vitality,healing]|shortLabel]]",
        Some(3),
        "3 vitality healing"
    )]
    #[case("@Damage[(@item.rank)[persistent,fire]]", None, "0 persistent fire")]
    #[case(
        "@Damage[(@item.rank)[persistent,vitality]]",
        Some(2),
        "2 persistent vitality"
    )]
    fn clean_damage_string_with_item_rank_and_no_dmg(
        #[case] input_str: &str,
        #[case] input_lvl: Option<i64>,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input_str, input_lvl);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("@Damage[(@item.rank -4)d6[fire]]", Some(4), "0d6 fire")]
    #[case("@Damage[(@item.rank*2)d6[electricity]]", Some(4), "8d6 electricity")]
    #[case("@Damage[(@item.rank -4)d6[fire]]", None, "-4d6 fire")]
    #[case("@Damage[(@item.rank*2)d6[electricity]]", None, "0d6 electricity")]
    fn clean_damage_string_with_item_rank_and_dmg_and_complex_op(
        #[case] input: &str,
        #[case] input_lvl: Option<i64>,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input, input_lvl);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "@Damage[(2*(@item.level - 2))[fire,persistent]]",
        Some(4),
        "4 fire persistent"
    )]
    #[case(
        "@Damage[(2*(@item.level - 2))[fire,persistent]]",
        None,
        "-4 fire persistent"
    )]
    fn clean_damage_string_with_item_level_and_dmg_to_calculate_and_dmg_type(
        #[case] input: &str,
        #[case] input_lvl: Option<i64>,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input, input_lvl);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "@Damage[2d6[bludgeoning],2d6[piercing],2d6[slashing]]{2d6 bludgeoning damage, 2d6 piercing damage, and 2d6 slashing damage}",
        "2d6 bludgeoning damage, 2d6 piercing damage, and 2d6 slashing damage"
    )]
    // pipe options (|options:...) before the curly label — seen in area-damage entries
    #[case(
        "@Damage[3d12[piercing],2d12[void]|options:area-damage]{3d12 piercing damage and 2d12 void damage}",
        "3d12 piercing damage and 2d12 void damage"
    )]
    fn clean_multi_damage_with_label(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, None);
        assert_eq!(expected, parsed_description);
    }
    
    #[rstest]
    #[case(
        "@Damage[7d6[fire],4d12[persistent,poison]|options:area-damage]",
        "7d6 fire, 4d12 persistent poison"
    )]
    #[case("@Damage[3d6[fire],3d6[cold]]", "3d6 fire, 3d6 cold")]
    #[case(
        "@Damage[2d8[slashing],1d6[persistent,bleed]]",
        "2d8 slashing, 1d6 persistent bleed"
    )]
    fn clean_multi_damage_without_label(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, None);
        assert_eq!(expected, parsed_description);
    }
    
    #[rstest]
    #[case("@Damage[1d6[fire]]{1d6 fire damage}", "1d6 fire damage")]
    fn clean_single_damage_curly_label_preferred(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, None);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "@Damage[2d6[persistent,fire]] or @Damage[2d6[persistent,acid]]",
        "2d6 persistent fire or 2d6 persistent acid"
    )]
    fn clean_double_damage_string_with_dmg_and_dmg_type(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input, None);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "@Damage[(@actor.abilities.str.mod)[bludgeoning]]{bludgeoning}",
        "bludgeoning"
    )]
    fn generic_curly_bracket_match(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, None);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "@Damage[6d6[fire]|traits:area-damage] damage and two @Damage[4d6[fire]|traits:area-damage]]",
        "6d6 fire damage and two 4d6 fire"
    )]
    fn multiple_damage_entry_in_one(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, None);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "@Damage[floor(1 + @actor.level/2)d6[void]|options:area-damage]",
        "floor(1 + Level/2)d6 void"
    )]
    #[case("@Damage[@actor.level[fire]]", "Level fire")]
    #[case("@Damage[@actor.rank[void,healing]]", "Level void healing")]
    fn clean_damage_string_with_actor_level(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, None);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "dealing @Damage[1d8[piercing]] damage plus @Damage[1d4[bleed]]{1d4 persistent bleed damage} to the wielder. If the weapon has a striking rune, this damage increases to @Damage[1d8[piercing]] damage per damage die and @Damage[1d4[bleed]]{1d4 persistent damage} per damage die;",
        "dealing 1d8 piercing damage plus 1d4 persistent bleed damage to the wielder. If the weapon has a striking rune, this damage increases to 1d8 piercing damage per damage die and 1d4 persistent damage per damage die;"
    )]
    fn multiple_damage_entry_in_one_and_one_curly_brackets(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input, None);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("take @Damage[10d6] damage", "take 10d6 damage")]
    #[case("@Damage[10d6]", "10d6")]
    fn simple_roll(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, None);
        assert_eq!(expected, parsed_description);
    }

    /* FEAT DAMAGE EDGE CASES, STILL NOT IMPLEMENTED
    #[rstest]
    #[case("@Damage[((@actor.flags.pf2e.swashbuckler.preciseStrike)d6[precision])[@item.system.damage.damageType]]{Precise Strike damage}", "ciao")]
    #[case("@Damage[((@actor.flags.pf2e.swashbuckler.preciseStrike)d6[precision])[@item.system.damage.damageType]]{full precise strike damage}", "ciao")]
    fn clean_damage_string_with_actor_flags_and_dmg_and_descr(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "@Damage[@item.level[@actor.flags.pf2e.sorcerer.elementalBloodline.damageType]]",
        "ciao"
    )]
    fn clean_damage_string_with_item_level_and_dmg_type(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input, None, None);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "@Damage[3d4[@actor.flags.pf2e.draconicExemplar.damageType,persistent]]",
        "ciao"
    )]
    fn clean_damage_string_with_actor_flags_and_dmg_and_dmg_type(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("@Damage[ceil(@actor.level/2)d8[@actor.flags.pf2e.draconicExemplar.damageType]|traits:area-damage]", "ciao")]
    fn clean_damage_string_with_actor_level_and_dmg_and_dmg_type_and_trait(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

        #[rstest]
    #[case("@Damage[(ternary(gte(@actor.level,19),6,ternary(gte(@actor.level,16),5,ternary(gte(@actor.level,12),4,ternary(gte(@actor.level,9),3,ternary(gte(@actor.level,5),2,1))))))d6[persistent,mental]]", "ciao")]
    #[case("@Damage[(ternary(gte(@actor.level,18),2,1))d4[electricity]]", "ciao")]
    fn clean_damage_string_with_ternary_actor_level(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("@Damage[(@actor.level)[void,healing]]{Hit Points}", "ciao")]
    #[case("@Damage[(@actor.level)[vitality,healing]]{Hit Points}", "ciao")]
    fn clean_damage_string_with_actor_level(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("@Damage[@actor.level[@actor.flags.pf2e.inventor.explode]|immutable|name:PF2E.SpecificRule.Inventor.Innovation.MalfunctionDamage]", "ciao")]
    fn clean_damage_string_with_actor_level_and_no_dmg(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

     */
}
