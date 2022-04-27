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

primary ::= charSet | '$' | '.' | CharCode | String | Identifier | '(' S choice S ')'

charSet ::= '[' '^'? ( CharSetChar ( '-' CharSetChar )? )* ']'

CharSetChar ::= CharCode | [^#x09#x0A#x0D#x23#x5D] /* TAB or LF or CR or '#' or ']' */

CharCode ::= '#x' [0-9a-fA-F]+

String ::= "'" [^']* "'" | '"' [^"]* '"'

Identifier ::= [_a-zA-Z] [_a-zA-Z0-9]*

S ::= ( Whitespace | Comment )*

Comment ::= '/*' ( [^*] | '*'+ [^*/] )* '*'+ '/'

Whitespace ::= #x09 | #x0A | #x0D | #x20
```

```yml
mapping:
  use: super::tree_builder
productions:
  grammar:
    map:
  primary:
    map:
  primary/1:
    map: to_eof
  primary/2:
    map: to_any
  primary/3:
    map: map_char_code_in_primary
  primary/5:
    lookahead: S [^:]
    map: map_identifier_in_primary
  CharSetChar:
    map:
  CharCode:
    unwrap:
  Identifier:
    map:
  S:
    ignore: true
  Comment:
    ignore: true
  Whitespace:
    ignore: true
```
