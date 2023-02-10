# Syntax

identifier := a..z\*

type :=
| Int
| Bool

if_statement := if expr { statement+; expr } else { statement+; expr}

statement :=
| if_statement

arg := identifier: type

function := fn identifier arg+ { statement+ }

Token::Fn Token::Identifier Token::Identifier Token::Type Token::LeftBrace

expr :=
| identifier
| function

var :=
let identifier: type = expr
