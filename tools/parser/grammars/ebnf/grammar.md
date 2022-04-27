# W3C EBNF Grammar

This grammar describes W3C EBNF, using [W3C EBNF](https://www.w3.org/TR/REC-xml/#sec-notation)

The valid locations for whitespace and comments are explicitly specified, rather than being ambient.

```ebnf
grammar ::= ( S production )* S

production ::= Identifier S '::=' S choice

choice ::= sequence ( S '|' S sequence )*

sequence ::= difference ( S difference )*

difference ::= item ( S '-' S item )?

item ::= primary ( '?' | '*' | '+' )?

primary ::= set | '$' | '.' | CharCode | String | Identifier | '(' S choice S ')'

set ::= '[' '^'? ( SetChar ( '-' SetChar )? )* ']'

SetChar ::= CharCode | [^#x09#x0A#x0D#x23#x5D] /* TAB or LF or CR or '#' or ']' */

CharCode ::= '#x' [0-9a-fA-F]+

String ::= "'" [^']* "'" | '"' [^"]* '"'

Identifier ::= [_a-zA-Z] [_a-zA-Z0-9]*

S ::= ( Whitespace | Comment )*

Comment ::= '/*' ( [^*] | '*'+ [^*/] )* '*'+ '/'

Whitespace ::= #x09 | #x0A | #x0D | #x20
```

```yml
production:
  pattern:
    padded_by: 1
choice:
  pattern:
    padded_by: 1.0
    separated_by: 0
sequence:
  pattern:
    separated_by: 0
difference:
  pattern:
    padded_by: 1.0
primary:
  pattern:
    padded_by: 6.1
    delimited_by: 6.0
set:
  pattern:
    padded_by: 0
S:
  ignored:
Comment:
  ignored:
whitespace:
  ignored:
```
