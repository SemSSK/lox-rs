mod token;
use common::is_alphanumeric;
use token::*;

const UNIQUE_CHARACTER_SYMBOLE: [&str; 10] = ["(", ")", "{", "}", ",", ".", "-", "+", ";", "*"];
const EQUAL_ADJACENT_SYMBOLE: [&str; 4] = ["=", "!", ">", "<"];

/// Used for generating a vector of tokens with type
/// ```
///     Result<Token,String>
/// ```
/// As there might be syntax errors that need to be addressed
/// # Example:
///
/// ```
/// fn main(){
///     let code = <LOX CODE GOES HERE>;
///     let tokens = Lexer::new(&code)
///                         .process();
/// }
/// ```
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    line: u32,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let input = input.to_string();
        Self {
            input,
            position: 0,
            read_position: 0,
            line: 0,
        }
    }

    fn process(&mut self) -> Vec<Result<Token, String>> {
        let mut tokens = vec![];

        loop {
            self.position = self.read_position;
            self.read_position += 1;
            if self.read_position > self.input.len() {
                break;
            }
            let s = &self.input[self.position..self.read_position];
            match s {
                "\n" => self.line += 1,
                " " | "\t" | "\r" => {
                    continue;
                }
                s if UNIQUE_CHARACTER_SYMBOLE.contains(&s) => {}
                s if EQUAL_ADJACENT_SYMBOLE.contains(&s) => {
                    if let Some(c) = self.input.get(self.read_position..(self.read_position + 1)) {
                        match c {
                            "=" => {
                                self.read_position += 1;
                            }
                            _ => {}
                        }
                    }
                }
                "\"" => {
                    if let Err(s) = self.process_string() {
                        tokens.push(Err(s))
                    }
                }
                "/" => {
                    if let Err(s) = self.process_slash() {
                        tokens.push(Err(s))
                    }
                    if self.read_position - self.position > 1 {
                        continue;
                    }
                }
                _ => {
                    if let Err(s) = self.process_other() {
                        tokens.push(Err(s))
                    }
                }
            }
            tokens.push(Ok(Token::from_str(
                &self.input[self.position..self.read_position],
                self.line,
            )));
        }

        tokens.push(Ok(Token::eof(self.line)));
        tokens
    }

    fn process_string(&mut self) -> Result<(), String> {
        let s = self.input.get(self.read_position..(self.read_position + 1));
        match s {
            None => Err("Unclosed string error".to_string()),
            Some(s) => match s {
                "\"" => {
                    self.read_position += 1;
                    Ok(())
                }
                _ => {
                    self.read_position += 1;
                    self.process_string()
                }
            },
        }
    }

    fn process_other(&mut self) -> Result<(), String> {
        let s = self.input.get(self.read_position..(self.read_position + 1));
        match s {
            None => Ok(()),
            Some(s) => match s {
                s if UNIQUE_CHARACTER_SYMBOLE.contains(&s)
                    || EQUAL_ADJACENT_SYMBOLE.contains(&s)
                    || [" ", "\r", "\t", "\n"].contains(&s) =>
                {
                    Ok(())
                }
                _ => {
                    self.read_position += 1;
                    self.process_other()
                }
            },
        }
    }

    fn process_slash(&mut self) -> Result<(), String> {
        let s = self.input.get(self.read_position..(self.read_position + 1));
        match s {
            None => Ok(()),
            Some(s) => match s {
                "/" => {
                    self.read_position += 1;
                    self.process_comment()
                }
                _ => Ok(()),
            },
        }
    }

    fn process_comment(&mut self) -> Result<(), String> {
        let s = self.input.get(self.read_position..(self.read_position + 1));
        match s {
            None => Ok(()),
            Some(s) => match s {
                "\n" => {
                    self.read_position += 1;
                    self.line += 1;
                    Ok(())
                }
                _ => {
                    self.read_position += 1;
                    self.process_comment()
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::{token::Token, Lexer};

    #[test]
    fn tst_1() {
        let input = "var lexem == 1 + 2;";
        let tokens = Lexer::new(input)
            .process()
            .into_iter()
            .filter(|t| t.is_ok())
            .map(|t| t.unwrap())
            .collect::<Vec<_>>();
        let actual = vec![
            Token::from_str("var", 0),
            Token::from_str("lexem", 0),
            Token::from_str("==", 0),
            Token::from_str("1", 0),
            Token::from_str("+", 0),
            Token::from_str("2", 0),
            Token::from_str(";", 0),
            Token::eof(0),
        ];
        assert_eq!(actual, tokens);
    }

    #[test]
    fn tst_2() {
        let input = "// Your first Lox program!\nprint \"Hello, world!\";";
        let tokens = Lexer::new(input)
            .process()
            .into_iter()
            .filter(|t| t.is_ok())
            .map(|t| t.unwrap())
            .collect::<Vec<_>>();
        let actual = vec![
            Token::from_str("print", 1),
            Token::from_str("\"Hello, world!\"", 1),
            Token::from_str(";", 1),
            Token::eof(1),
        ];
        assert_eq!(actual, tokens);
    }

    #[test]
    fn tst_3() {
        let input = fs::read_to_string("test_samples/main.lox").unwrap();

        let tokens = Lexer::new(&input)
            .process()
            .into_iter()
            .filter(|t| t.is_ok())
            .map(|t| t.unwrap())
            .collect::<Vec<_>>();
        let actual = vec![
            Token::from_str("var", 0),
            Token::from_str("a", 0),
            Token::from_str("=", 0),
            Token::from_str("1", 0),
            Token::from_str(";", 0),
            Token::from_str("while", 1),
            Token::from_str("(", 1),
            Token::from_str("a", 1),
            Token::from_str("<", 1),
            Token::from_str("10", 1),
            Token::from_str(")", 1),
            Token::from_str("{", 1),
            Token::from_str("print", 2),
            Token::from_str("a", 2),
            Token::from_str(";", 2),
            Token::from_str("a", 3),
            Token::from_str("=", 3),
            Token::from_str("a", 3),
            Token::from_str("+", 3),
            Token::from_str("1", 3),
            Token::from_str(";", 3),
            Token::from_str("}", 4),
        ];
        assert!(true);
    }
}
