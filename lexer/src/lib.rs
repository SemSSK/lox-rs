#[derive(Debug)]
pub enum Token {
    LeftParen { line: u32 },
    RightParen { line: u32 },
    LeftBrace { line: u32 },
    RightBrace { line: u32 },
    Comma { line: u32 },
    Dot { line: u32 },
    Minus { line: u32 },
    Plus { line: u32 },
    Semicolon { line: u32 },
    Slash { line: u32 },
    Star { line: u32 },
    Bang { line: u32 },
    BangEqual { line: u32 },
    Greater { line: u32 },
    GreaterEqual { line: u32 },
    Lesser { line: u32 },
    LesserEqual { line: u32 },
    Identifier { line: u32, literal: String },
    LoxString { line: u32, literal: String },
    Number { line: u32, literal: f64 },
    And { line: u32 },
    Class { line: u32 },
    If { line: u32 },
    Else { line: u32 },
    False { line: u32 },
    True { line: u32 },
    Nil { line: u32 },
    Or { line: u32 },
    Print { line: u32 },
    Return { line: u32 },
    Super { line: u32 },
    This { line: u32 },
    Var { line: u32 },
    While { line: u32 },
    Fun { line: u32 },
    Eof { line: u32 },
}

impl Token {
    pub fn from_str(lexem: &str, line: u32) -> Self {
        use Token::*;

        match lexem {
            "(" => LeftParen { line },
            ")" => RightParen { line },
            "{" => LeftBrace { line },
            "}" => RightBrace { line },
            "," => Comma { line },
            "." => Dot { line },
            "-" => Minus { line },
            "+" => Plus { line },
            ";" => Semicolon { line },
            "/" => Slash { line },
            "*" => Star { line },
            "!" => Bang { line },
            "!=" => BangEqual { line },
            ">" => GreaterEqual { line },
            ">=" => GreaterEqual { line },
            "<" => Lesser { line },
            "<=" => LesserEqual { line },
            "and" => And { line },
            "class" => Class { line },
            "if" => If { line },
            "else" => Else { line },
            "false" => False { line },
            "true" => True { line },
            "nil" => Nil { line },
            "or" => Or { line },
            "print" => Print { line },
            "return" => Return { line },
            "super" => Super { line },
            "var" => Var { line },
            "this" => This { line },
            "while" => While { line },
            "fun" => Fun { line },
            s if is_numeric(s) => Number {
                line,
                literal: s.parse().unwrap(),
            },
            s if is_string(s) => LoxString {
                line,
                literal: s[1..(s.len() - 1)].to_string(),
            },
            s => Identifier {
                line,
                literal: s.to_string(),
            },
        }
    }
}

fn is_numeric(s: &str) -> bool {
    s.chars()
        .map(|c| ('0'..='9').contains(&c) || c == '.')
        .fold(true, |acc, c| acc && c)
}

fn is_string(s: &str) -> bool {
    s.starts_with('"') && s.ends_with('"')
}
