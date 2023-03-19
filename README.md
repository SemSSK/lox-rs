# Lox-rs
## Yet another implementation of an interpreter of lox language in Rust
made by following the book [Crafting Interpreters by Robert Nystrom](https://craftinginterpreters.com/)
## What you will find this repo
this repo will contain the implementation of the interpreter, following each chapter of the book summerizing the algorithms described in the book a programming language agnostic way and how it translates to rust.

### [1 - Scanning](#scanning)
### [2 - Context Free Grammar (CFG)](#context-free-grammar-cfg)


# Scanning
I will deviate a little from the book in this part as i prefer to test the code in a sandbox using unit tests rather than write a repl for now but we will get back to it when necessary.

First we must determine the tokens which will be seen by our interpreter as written source code is juste a string of characters we must restructure it in such a way that a machine can understand it.

**for example** : the line of code `var pi = 3.14159265359` cannot be used as is by the interpreter it must first be restructured into:

| LET  |  IDENTIFIER("pi") | NUMBER(3.14159265359) |
| ------------- |:-------------:| -----:|

We must the determine all the possible tokens in the lox language so we have:


| Token         | Code representation           |
| ------------- |:-------------:|
| LeftParen     | (             |
| RightParen    | )             |
| LeftBrace | {      |
| RightBrace| }      |
| Comma | ,      |
| Dot | .      |
| Minus | -      |
| Plus | +      |
| Semicolon | ;      |
| Slash | /      |
| Star | *      |
| Bang | !      |
| BangEqual | !=      |
| Greater | >      |
| GreaterEqual | >=      |
| Lesser | <      |
| LesserEqual | <=      |
| Identifier |  any name for a variable, function, class ...     |
| string | "a string"      |
| Number | 132      |
| And | and      |
| Class | class      |
| If | if      |
| Else | else      |
| False | false      |
| True | true      |
| Nil | nil      |
| Or | or      |
| Print | print      |
| Return | return      |
| Super | super      |
| This | this      |
| Var | var      |
| While | while      |
| Fun | fun      |
| Eof | special token represents end of file |

the tokens will be represented as an rust enum data type the lexer's return type will be (`Vec<Result<Token,String>>`) in case of syntax errors

the algorithm for parsing is similar to the one showcased in the book althought it was easier to parse through special symbols such as : if, else, while...etc using pattern matching on the strings to construct the token instead of explicitly creating them.

# Context Free Grammar (CFG)

Before we get to the parser we need to a representation of an abstract syntax tree that satisfies the following CFG:

```
expression     → literal
               | unary
               | binary
               | grouping ;

literal        → NUMBER | STRING | "true" | "false" | "nil" ;

grouping       → "(" expression ")" ;

unary          → ( "-" | "!" ) expression ;

binary         → expression operator expression ;

operator       → "==" | "!=" | "<" | "<=" | ">" |   ">=" | "+"  | "-"  | "*" | "/" ;

```

this part will differ alot from the book as we will represent the AST using an enums and pattern matching allowing us to have a one to one translation from the Grammar to the AST as follows:

```rust
#[derive(PartialEq)]
enum Literal {
    Nil,
    True,
    False,
    LiteralString(String),
    LiteralNumber(f64),
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
#[derive(Debug)]
enum Binary {
    Val(Box<Expr>, Operator, Box<Expr>),
}
#[derive(Debug)]
enum Unary {
    MinusExpr(Box<Expr>),
    BangExpr(Box<Expr>),
}
#[derive(Debug)]
enum Expr {
    Lit(Literal),
    Una(Unary),
    Bin(Binary),
    Grouping(Box<Expr>),
}

```

`
Box<Expr>
`
is used to allow recursive Type definitions as rust can't tell ahead of time the stack size to allocate so it needs to use the heap.

the interpretation of the AST will be implemented as such

```rust
trait Interpretable {
    fn interpret(self) -> Result<Literal, String>;
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
```
<div align="center">
🚧 WORK IN PROGRESS 🚧
</div>