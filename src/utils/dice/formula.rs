use crate::utils::dice::ast::{Expr, const_fold, parse_expr_str};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParsedDiceFormula {
    pub n_of_dice: Option<i64>,
    pub die_size: Option<i64>,
    pub bonus_dmg: i64,
    /// Couldn't be resolved statically, e.g. `"+@item.rank"` or `"+floor(1+@actor.level/2)d6"`.
    pub unresolved_terms: Vec<String>,
}

fn signed(sign: i64, e: &Expr) -> String {
    format!("{}{e}", if sign < 0 { "-" } else { "+" })
}

fn flatten(e: &Expr, sign: i64, out: &mut ParsedDiceFormula) {
    match e {
        Expr::Add(a, b) => {
            flatten(a, sign, out);
            flatten(b, sign, out);
        }
        Expr::Sub(a, b) => {
            flatten(a, sign, out);
            flatten(b, -sign, out);
        }
        Expr::Neg(a) => flatten(a, -sign, out),
        Expr::Dice(count, size) => match (const_fold(count), const_fold(size)) {
            // only the first resolvable dice term is promoted to n_of_dice/die_size
            (Some(n), Some(d)) if out.n_of_dice.is_none() => {
                out.n_of_dice = Some(sign * n);
                out.die_size = Some(d);
            }
            _ => out.unresolved_terms.push(signed(sign, e)),
        },
        other => match const_fold(other) {
            Some(v) => out.bonus_dmg += sign * v,
            None => out.unresolved_terms.push(signed(sign, other)),
        },
    }
}

pub fn parse_dice_formula(formula: &str) -> Option<ParsedDiceFormula> {
    let formula = formula.trim();
    if formula.is_empty() {
        return None;
    }
    let expr = parse_expr_str(formula).ok()?;
    let mut out = ParsedDiceFormula::default();
    flatten(&expr, 1, &mut out);
    Some(out)
}
