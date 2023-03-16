# Lox-rs
## Yet another implementation of an interpreter of lox language in Rust
made by following the book [Crafting Interpreters by Robert Nystrom](https://craftinginterpreters.com/)
## What you will find this repo
this repo will contain the implementation of the interpreter, following each chapter of the book summerizing the algorithms described in the book a programming language agnostic way and how it translates to rust.

## Chapter 1 : Scanning
I will deviate a little from the book in this part as i prefer to test the code in a sandbox using unit tests rather than write a repl for now but we will get back to it when necessary.

First we must determine the tokens which will be seen by our interpreter as written source code is juste a string of characters we must restructure it in such a way that a machine can understand it.

**for example** : the line of code `var pi = 3.14159265359` cannot be used as is by the interpreter it must first be restructured into:

| LET  |  IDENTIFIER("pi") | NUMBER(3.14159265359) |
| ------------- |:-------------:| -----:|

We must the determine all the possible tokens in the lox language so we have:

LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

// One or two character tokens.
BANG, BANG_EQUAL,
EQUAL, EQUAL_EQUAL,
GREATER, GREATER_EQUAL,
LESS, LESS_EQUAL,

// Literals.
IDENTIFIER, STRING, NUMBER,

// Keywords.
AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

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

the tokens will be represented as an rust enum data type 