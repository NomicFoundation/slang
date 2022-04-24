# W3C EBNF Grammar

This grammar describes W3C EBNF, using [W3C EBNF](https://www.w3.org/TR/REC-xml/#sec-notation)

The valid locations for whitespace and comments are explicitly specified, rather than being ambient.

```ebnf
grammar ::= ( rule | S )*

rule ::= identifier S? '::=' S? expression

expression ::= optional

optional ::= sequence '?'?

sequence ::= alternate (S? alternate)*

alternate ::= exclusion ( S? '|' S? exclusion )*

exclusion ::= repeat ( S? '-' S? repeat )?

repeat ::= term ( '+' | '*' )?

term ::= hex_character | string | identifier | set | '(' S? expression S? ')'

hex_character ::= '#x' [0-9a-fA-Z]+

string ::= ' [^']* ' | " [^"]* "

identifier ::= [a-zA-Z$_] [a-zA-Z$_0-9]*

set ::= '[' '^'? ( setchar ( '-' setchar )? )* ']'

setchar := hex_character | [^#x23#x5D] /* # or ] */

COMMENT
    ::= '/*' ( [^*] | '*' [^/] )* '*/'

S
    ::= ( ' ' | 0x09 | 0x0A | 0x0D | COMMENT )+
```
