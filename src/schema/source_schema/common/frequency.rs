use itertools::Itertools;
use phf::{Map, phf_map};
use serde_json::Value;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Frequency {
    pub max: i64,
    pub per: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PeriodUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
}

impl Display for PeriodUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = String::from(match self {
            Self::Year => "year",
            Self::Month => "month",
            Self::Week => "week",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Minute => "minute",
            Self::Second => "second",
        });
        write!(f, "{}", str)
    }
}

const DATE_UNITS: Map<char, PeriodUnit> = phf_map! {
    'Y' => PeriodUnit::Year,
    'M' => PeriodUnit::Month,
    'W' => PeriodUnit::Week,
    'D' => PeriodUnit::Day,
};

const TIME_UNITS: Map<char, PeriodUnit> = phf_map! {
    'H' => PeriodUnit::Hour,
    'M' => PeriodUnit::Minute,
    'S' => PeriodUnit::Second,
};

impl TryFrom<&Value> for Frequency {
    type Error = ();
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let max = json.get("max").and_then(Value::as_i64).ok_or(())?;
        let per = json.get("per").and_then(Value::as_str).ok_or(())?;

        // Most `per` values are already plain enum-like words (`day`, `round`, `turn`) paired
        // with a meaningful `max` (how many times per period). A few Foundry entries (e.g.
        // "Objection", "Belcorra Haruvex") instead use a single-unit ISO 8601 duration
        // (`PT1M`, `PT10M`, `PT1H`, `P1W`, `P1M`, `P1Y`) where `max` is always 1;
        // the actually meaningful number is embedded in the duration itself.
        // For those, fold the embedded amount into `max` and reduce `per` to one canonical
        // singular unit name, so "minute"/"minutes" collapse to a single enum-like value
        // instead of depending on the amount.
        let (max, per) = match parse_iso_period(per) {
            Some((amount, unit)) => (amount, unit.to_string()),
            None => (max, per.to_string()),
        };

        Ok(Frequency { max, per })
    }
}

/// Parses a single-unit ISO 8601 duration (e.g. `PT10M`, `P1W`) into (amount, canonical
/// singular unit). Returns `None` for non-ISO strings.
fn parse_iso_period(raw: &str) -> Option<(i64, PeriodUnit)> {
    let rest = raw.strip_prefix('P')?;
    let (date_part, time_part) = match rest.split_once('T') {
        Some((d, t)) => (d, Some(t)),
        None => (rest, None),
    };

    let date_components = parse_duration_components(date_part, &DATE_UNITS);
    let time_components = time_part
        .map(|t| parse_duration_components(t, &TIME_UNITS))
        .unwrap_or_default();

    date_components
        .into_iter()
        .chain(time_components)
        .at_most_one()
        .ok()
        .flatten()
}

fn parse_duration_components(s: &str, units: &Map<char, PeriodUnit>) -> Vec<(i64, PeriodUnit)> {
    // logic of the split: 1Y2M => 1Y, 2M; splits at every non-digit char
    s.split_inclusive(|c: char| !c.is_ascii_digit())
        .filter_map(|chunk| {
            let mut chars = chunk.chars();
            let last = chars.next_back()?;
            if last.is_ascii_digit() {
                return None; // trailing digits == no unit char
            }
            let name = units.get(&last)?;
            let n: i64 = chars.as_str().parse().ok()?;
            Some((n, *name))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn try_from_parses_valid_json() {
        let json = serde_json::json!({"max": 3, "per": "day", "value": 1});
        let frequency = Frequency::try_from(&json).unwrap();
        assert_eq!(3, frequency.max);
        assert_eq!("day", frequency.per);
    }

    #[test]
    fn try_from_rejects_missing_fields() {
        assert!(Frequency::try_from(&serde_json::json!({})).is_err());
        assert!(Frequency::try_from(&Value::String("".to_string())).is_err());
    }

    #[rstest]
    #[case("round", 1, "round")]
    #[case("turn", 1, "turn")]
    #[case("PT1M", 1, "minute")]
    #[case("PT10M", 10, "minute")]
    #[case("PT1H", 1, "hour")]
    #[case("P1W", 1, "week")]
    #[case("P1M", 1, "month")]
    #[case("P1Y", 1, "year")]
    fn try_from_folds_iso8601_amount_into_max(
        #[case] per: &str,
        #[case] expected_max: i64,
        #[case] expected_per: &str,
    ) {
        // json max is always 1 for these in practice; use 1 here so the test can't pass
        // by accident of json max happening to equal expected_max.
        let json = serde_json::json!({"max": 1, "per": per});
        let frequency = Frequency::try_from(&json).unwrap();
        assert_eq!(expected_max, frequency.max);
        assert_eq!(expected_per, frequency.per);
    }

    #[test]
    fn try_from_ignores_json_max_for_iso8601_durations() {
        // The embedded amount (10) wins over the json max (5): json max is meaningless for
        // these entries (always 1 in practice), so it must not leak through when it isn't.
        let json = serde_json::json!({"max": 5, "per": "PT10M"});
        let frequency = Frequency::try_from(&json).unwrap();
        assert_eq!(10, frequency.max);
        assert_eq!("minute", frequency.per);
    }

    #[test]
    fn try_from_falls_back_to_raw_per_for_multi_unit_durations() {
        let json = serde_json::json!({"max": 1, "per": "P1Y2M3D"});
        let frequency = Frequency::try_from(&json).unwrap();
        assert_eq!(1, frequency.max);
        assert_eq!("P1Y2M3D", frequency.per);
    }
}
