# The BASIC programming language for the Sharp PC-1500/TRS-80 PC-2

## EBNF

Checked with [BNF Visualizer](https://bnfplayground.pauliankline.com/).

```ebnf
/* --- Tokens --- */
<digit> ::= [0-9]
<number> ::= <digit>+
<letter> ::= [A-Z]
<identifier> ::= <letter> (<letter> | <digit>)*
<comparison_op> ::= "=" | "<>" | "<" | ">" | "<=" | ">="
<add_sub_op> ::= "+" | "-"
<mul_div_op> ::= "*" | "/"
<char> ::= [A-Z] | [a-z] | [0-9] | " " | "!" | "\"" | "#" | "$" | "%" | "&" | "'" | "(" | ")" | "*" | "+" | "," | "-" | "." | "/" | ":" | ";" | "<" | "=" | ">" | "?" | "@" | "[" | "\\" | "]" | "^" | "_" | "`" | "{" | "|" | "}" | "~"
<string> ::= "\"" <char>* "\"" | "\"" <char>*
<newline> ::= "\n"

/* --- Grammar --- */
<program> ::= <line>+
<line> ::= <number> <statement> <newline>

/* Statements */
<statement> ::= <atomic_statement> (":" <atomic_statement>)*
<atomic_statement> ::= <assignment> 
    | <print>
    | <pause>
    | <input>
    | <wait>
    | <if>
    | <for>
    | <next>
    | <goto>
    | <gosub>
    | <return>
    | <end>
    | <comment>
    | <data>
    | <read>
    | <restore>
    | <poke>
    | <call>
    | <dim>

/* Comments */
<comment> ::= "REM" <char>*

/* Variables */
<variable> ::= <identifier> "$"?
<array_subscript> ::= <variable> "(" <expression> ")"
<lvalue> ::= <variable> | <array_subscript>
<assignment> ::= "LET"? <lvalue> "=" <expression>

/* I/O */
<print> ::= "PRINT" <expression> (";" <expression>)*
<pause> ::= "PAUSE" <expression> (";" <expression>)*
<input> ::= "INPUT" (<expression> ";")? <lvalue>
<wait> ::= "WAIT" <expression>?

/* Data */
<data_item> ::= <number> | <string>
<data> ::= "DATA" <data_item> ("," <data_item>)*
<read> ::= "READ" <lvalue> ("," <lvalue>)*
<restore> ::= "RESTORE" (<number>)?

/* Control flow */
<if> ::= "IF" <expression> "THEN" <statement> ("ELSE" <statement>)?
<for> ::= "FOR" <variable> "=" <expression> "TO" <expression> ("STEP" <expression>)?
<next> ::= "NEXT" <variable>
<goto> ::= "GOTO" <number>
<gosub> ::= "GOSUB" <number>
<return> ::= "RETURN"
<end> ::= "END"

/* Assembly */
<poke> ::= "POKE" <number>, (<number>)+
<call> ::= "CALL" <number>

/* Arrays */
<dim> ::= "DIM" <variable> "(" <number> ")" ("*" <number>)?

/* Expressions */
<expression> ::= <or_expr>
<or_expr> ::= <and_expr> ("OR" <and_expr>)*
<and_expr> ::= <not_expr> ("AND" <not_expr>)*
<not_expr> ::= <comparison> | "NOT" <not_expr>
<comparison> ::= <add_sub> (<comparison_op> <add_sub>)*
<add_sub> ::= <mul_div> (<add_sub_op> <mul_div>)*
<mul_div> ::= <factor> (<mul_div_op> <factor>)*
<factor> ::= "-" <factor> | "+" <factor> | <term>
<term> ::= <number> | <lvalue> | <string> | "(" <expression> ")"
```
