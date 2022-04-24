# W3C EBNF Grammar

This grammar describes W3C EBNF, using [W3C EBNF](https://www.w3.org/TR/REC-xml/#sec-notation)

The valid locations for whitespace and comments are explicitly specified, rather than being ambient.

```ebnf
grammar ::= ( S? production )* S?

production ::= Identifier S? '::=' S? choice

choice ::= sequenceOrDifference ( S? '|' S? sequenceOrDifference )*

sequenceOrDifference ::= item ( ( S? '-' S? item ) | ( S? item )* )?

item ::= primary ( '+' | '*' | '?')?

primary ::= set | '$' | '.' | CharCode | String | Identifier | '(' S? choice S? ')'

set ::= '[' '^'? ( SetChar ( '-' SetChar )? )* ']'

SetChar ::= CharCode | [^#x09#x0A#x0D#x23#x5D] /* TAB or LF or CR or '#' or ']' */

CharCode ::= '#x' [0-9a-fA-Z]+

String ::= "'" [^']* "'" | '"' [^"]* '"'

Identifier ::= [_a-zA-Z] [_a-zA-Z0-9]*

S
    ::= ( Whitespace | Comment )*

Comment
    ::= '/*' ( [^*] | '*'+ [^*/] )* '*'+ '/'

Whitespace
    ::= #x09 | #x0A | #x0D | #x20
```
