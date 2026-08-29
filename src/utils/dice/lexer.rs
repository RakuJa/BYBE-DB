#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Comma,
    Dice,            // literal 'd' dice operator
    Ident(String),   // function name, e.g. "floor", "max"
    Dynamic(String), // "@..." reference, e.g. "@actor.level"
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = s.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' => i += 1,
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                i += 1;
            }
            '*' => {
                tokens.push(Token::Star);
                i += 1;
            }
            '/' => {
                tokens.push(Token::Slash);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                i += 1;
            }
            '0'..='9' => {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let n: i64 = chars[start..i]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .map_err(|_| "bad number".to_string())?;
                tokens.push(Token::Num(n));
            }
            '@' => {
                let start = i;
                i += 1;
                while i < chars.len()
                    && (chars[i].is_ascii_alphanumeric() || chars[i] == '.' || chars[i] == '_')
                {
                    i += 1;
                }
                tokens.push(Token::Dynamic(chars[start..i].iter().collect()));
            }
            c if c.is_ascii_alphabetic() => {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_alphabetic() {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                if word.eq_ignore_ascii_case("d") {
                    tokens.push(Token::Dice);
                } else {
                    tokens.push(Token::Ident(word));
                }
            }
            other => return Err(format!("unexpected character '{other}'")),
        }
    }
    Ok(tokens)
}
