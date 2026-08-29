use super::lexer::{Token, tokenize};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i64),
    Dynamic(String),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Dice(Box<Expr>, Box<Expr>), // count d size
    Call(String, Vec<Expr>),
}

/// Wraps an operand in parens when displaying it as a sub-expression of a
/// higher-precedence node, unless it's already atomic. Prevents ambiguous
/// output like "2+@actor.leveld6" for a Dice node whose count is `2+@actor.level`.
struct Paren<'a>(&'a Expr);
impl std::fmt::Display for Paren<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Expr::Num(_) | Expr::Dynamic(_) | Expr::Call(..) => write!(f, "{}", self.0),
            other => write!(f, "({other})"),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{n}"),
            Expr::Dynamic(s) => write!(f, "{s}"),
            Expr::Neg(e) => write!(f, "-{}", Paren(e)),
            Expr::Add(a, b) => write!(f, "{a}+{b}"),
            Expr::Sub(a, b) => write!(f, "{a}-{}", Paren(b)),
            Expr::Mul(a, b) => write!(f, "{}*{}", Paren(a), Paren(b)),
            Expr::Div(a, b) => write!(f, "{}/{}", Paren(a), Paren(b)),
            Expr::Dice(c, s) => write!(f, "{}d{}", Paren(c), Paren(s)),
            Expr::Call(name, args) => {
                write!(f, "{name}(")?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{a}")?;
                }
                write!(f, ")")
            }
        }
    }
}

/// Tries to reduce an expression to a single known integer. Returns `None`
/// as soon as any `Dynamic` reference or unrecognized function is reached;
/// a dice roll (`Dice`) is also never folded, since it isn't a fixed value
/// even when both operands are constant (it's still a roll, not a number).
pub fn const_fold(e: &Expr) -> Option<i64> {
    match e {
        Expr::Num(n) => Some(*n),
        Expr::Dynamic(_) => None,
        Expr::Neg(a) => const_fold(a).map(|v| -v),
        Expr::Add(a, b) => Some(const_fold(a)? + const_fold(b)?),
        Expr::Sub(a, b) => Some(const_fold(a)? - const_fold(b)?),
        Expr::Mul(a, b) => Some(const_fold(a)? * const_fold(b)?),
        Expr::Div(a, b) => {
            let bv = const_fold(b)?;
            (bv != 0).then(|| const_fold(a)).flatten().map(|x| x / bv)
        }
        Expr::Dice(_, _) => None,
        Expr::Call(name, args) => {
            let vals = args.iter().map(const_fold).collect::<Option<Vec<_>>>()?;
            match (name.to_lowercase().as_str(), vals.as_slice()) {
                // integer division above already truncates; floor/ceil/round
                // are treated as identity on our i64 model
                ("floor" | "ceil" | "round", [a]) => Some(*a),
                ("abs", [a]) => Some(a.abs()),
                ("min", vs) if !vs.is_empty() => vs.iter().copied().min(),
                ("max", vs) if !vs.is_empty() => vs.iter().copied().max(),
                _ => None,
            }
        }
    }
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn bump(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        t
    }

    // expr := term (('+'|'-') term)*
    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.pos += 1;
                    left = Expr::Add(Box::new(left), Box::new(self.parse_term()?));
                }
                Some(Token::Minus) => {
                    self.pos += 1;
                    left = Expr::Sub(Box::new(left), Box::new(self.parse_term()?));
                }
                _ => return Ok(left),
            }
        }
    }

    // term := dice (('*'|'/') dice)*
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_dice()?;
        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.pos += 1;
                    left = Expr::Mul(Box::new(left), Box::new(self.parse_dice()?));
                }
                Some(Token::Slash) => {
                    self.pos += 1;
                    left = Expr::Div(Box::new(left), Box::new(self.parse_dice()?));
                }
                _ => return Ok(left),
            }
        }
    }

    // dice := unary ('d' unary)?
    fn parse_dice(&mut self) -> Result<Expr, String> {
        let left = self.parse_unary()?;
        if let Some(Token::Dice) = self.peek() {
            self.pos += 1;
            Ok(Expr::Dice(Box::new(left), Box::new(self.parse_unary()?)))
        } else {
            Ok(left)
        }
    }

    // unary := '-' unary | atom
    fn parse_unary(&mut self) -> Result<Expr, String> {
        if let Some(Token::Minus) = self.peek() {
            self.pos += 1;
            return Ok(Expr::Neg(Box::new(self.parse_unary()?)));
        }
        self.parse_atom()
    }

    // atom := number | dynamic | ident '(' expr (',' expr)* ')' | '(' expr ')'
    fn parse_atom(&mut self) -> Result<Expr, String> {
        match self.bump() {
            Some(Token::Num(n)) => Ok(Expr::Num(n)),
            Some(Token::Dynamic(s)) => Ok(Expr::Dynamic(s)),
            Some(Token::LParen) => {
                let e = self.parse_expr()?;
                match self.bump() {
                    Some(Token::RParen) => Ok(e),
                    other => Err(format!("expected ')', got {other:?}")),
                }
            }
            Some(Token::Ident(name)) => {
                if self.peek() != Some(&Token::LParen) {
                    // bare word with no call syntax — treat as opaque/dynamic
                    return Ok(Expr::Dynamic(name));
                }
                self.pos += 1;
                let mut args = Vec::new();
                if self.peek() != Some(&Token::RParen) {
                    args.push(self.parse_expr()?);
                    while self.peek() == Some(&Token::Comma) {
                        self.pos += 1;
                        args.push(self.parse_expr()?);
                    }
                }
                match self.bump() {
                    Some(Token::RParen) => Ok(Expr::Call(name, args)),
                    other => Err(format!("expected ')', got {other:?}")),
                }
            }
            other => Err(format!("unexpected token {other:?}")),
        }
    }
}

pub fn parse_expr_str(s: &str) -> Result<Expr, String> {
    let tokens = tokenize(s)?;
    let mut parser = Parser {
        tokens: &tokens,
        pos: 0,
    };
    let expr = parser.parse_expr()?;
    if parser.pos != tokens.len() {
        let x = parser.pos;
        return Err(format!("trailing input after {x} tokens"));
    }
    Ok(expr)
}
