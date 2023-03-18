use std::fmt::Debug;

use common::*;
use lexer::*;

trait Interpretable {
    fn interpret(self) -> Result<Literal, String>;
}

#[derive(PartialEq)]
enum Literal {
    Nil,
    True,
    False,
    LiteralString(String),
    LiteralNumber(f64),
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "Nil"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::LiteralString(arg0) => f.debug_tuple("").field(arg0).finish(),
            Self::LiteralNumber(arg0) => f.debug_tuple("").field(arg0).finish(),
        }
    }
}

impl Literal {
    fn boolean(b: bool) -> Literal {
        if b {
            Literal::True
        } else {
            Literal::False
        }
    }
    fn number(x: f64) -> Literal {
        Literal::LiteralNumber(x)
    }
    fn string(s: String) -> Literal {
        Literal::LiteralString(s)
    }
    fn nil() -> Literal {
        Literal::Nil
    }
}

enum Operator {
    EqualEqual,
    BangEqual,
    Lesser,
    LesserEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

impl Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EqualEqual => write!(f, "=="),
            Self::BangEqual => write!(f, "!="),
            Self::Lesser => write!(f, "<"),
            Self::LesserEqual => write!(f, "<="),
            Self::Greater => write!(f, ">"),
            Self::GreaterEqual => write!(f, ">="),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
        }
    }
}

#[derive(Debug)]
enum Binary {
    Val(Box<Expr>, Operator, Box<Expr>),
}
impl Interpretable for Binary {
    fn interpret(self) -> Result<Literal, String> {
        let Binary::Val(left, op, right) = self;
        let (left, right) = (left.interpret()?, right.interpret()?);
        match (left, right) {
            (Literal::Nil, _) | (_, Literal::Nil) => {
                Err("Type error cannot operate on Nil".to_string())
            }
            (Literal::True, Literal::True) | (Literal::False, Literal::False) => match op {
                Operator::EqualEqual => Ok(Literal::True),
                Operator::BangEqual => Ok(Literal::False),
                _ => Err("Type Error operator not implemented for Booleans".to_string()),
            },
            (Literal::True, Literal::False) | (Literal::False, Literal::True) => match op {
                Operator::EqualEqual => Ok(Literal::False),
                Operator::BangEqual => Ok(Literal::True),
                _ => Err("Type Error operator not implemented for Booleans".to_string()),
            },
            (Literal::LiteralNumber(x), Literal::LiteralNumber(y)) => match op {
                Operator::EqualEqual => Ok(Literal::boolean(x == y)),
                Operator::Greater => Ok(Literal::boolean(x > y)),
                Operator::GreaterEqual => Ok(Literal::boolean(x >= y)),
                Operator::Lesser => Ok(Literal::boolean(x < y)),
                Operator::LesserEqual => Ok(Literal::boolean(x <= y)),
                Operator::Plus => Ok(Literal::number(x + y)),
                Operator::Minus => Ok(Literal::number(x - y)),
                Operator::Star => Ok(Literal::number(x * y)),
                Operator::Slash => Ok(Literal::number(x / y)),
                _ => Err("Type Error operator not implemented for number".to_string()),
            },
            (Literal::LiteralString(x), Literal::LiteralString(y)) => match op {
                Operator::EqualEqual => Ok(Literal::boolean(x == y)),
                Operator::BangEqual => Ok(Literal::boolean(x != y)),
                Operator::Plus => Ok(Literal::string(x + &y)),
                _ => Err("Type Error operator not implemented for strings".to_string()),
            },
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
enum Unary {
    MinusExpr(Box<Expr>),
    BangExpr(Box<Expr>),
}

impl Interpretable for Unary {
    fn interpret(self) -> Result<Literal, String> {
        match self {
            Unary::MinusExpr(e) => {
                let l = e.interpret()?;
                match l {
                    Literal::LiteralNumber(n) => Ok(Literal::LiteralNumber(-n)),
                    _ => Err("Type error Number expected".to_string()),
                }
            }
            Unary::BangExpr(e) => {
                let l = e.interpret()?;
                match l {
                    Literal::False => Ok(Literal::True),
                    Literal::True => Ok(Literal::False),
                    _ => Err("Type error Number expected".to_string()),
                }
            }
        }
    }
}

#[derive(Debug)]
enum Expr {
    Lit(Literal),
    Una(Unary),
    Bin(Binary),
    Grouping(Box<Expr>),
}

impl Interpretable for Expr {
    fn interpret(self) -> Result<Literal, String> {
        match self {
            Expr::Lit(l) => Ok(l),
            Expr::Una(u) => u.interpret(),
            Expr::Bin(b) => b.interpret(),
            Expr::Grouping(e) => e.interpret(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_tree_1() {
        let tree = Expr::Bin(Binary::Val(
            Box::new(Expr::Una(Unary::MinusExpr(Box::new(Expr::Lit(
                Literal::number(5.),
            ))))),
            Operator::Star,
            Box::new(Expr::Grouping(Box::new(Expr::Lit(Literal::LiteralNumber(
                12.,
            ))))),
        ));
        let result = tree.interpret().unwrap();
        let actual = Literal::LiteralNumber(-60.);
        assert_eq!(actual, result)
    }
}
