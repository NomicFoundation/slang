# W3C EBNF Grammar

This grammar describes W3C EBNF, using [W3C EBNF](https://www.w3.org/TR/REC-xml/#sec-notation)

The valid locations for whitespace and comments are explicitly specified, rather than being ambient.

```ebnf
grammar ::= ( S production )* S

production ::= Identifier S '::=' S expression

expression ::= sequence ( S '|' S sequence )*

sequence ::= difference ( S difference )*

difference ::= item ( S '-' S item )?

item ::= primary ( '?' | '*' | '+' )?

primary ::= charSet | '$' | '.' | CharCode | String | Identifier | '(' S expression S ')'

charSet ::= '[' '^'? ( CharSetChar ( '-' CharSetChar )? )* ']'

CharSetChar ::= CharCode | [^#x09#x0A#x0D#x23#x5D] /* TAB or LF or CR or '#' or ']' */

CharCode ::= '#x' [0-9a-fA-F]+

String ::= "'" [^']* "'" | '"' [^"]* '"'

Identifier ::= [_a-zA-Z] [_a-zA-Z0-9]*

S ::= ( Whitespace | Comment )*

Comment ::= '/*' ( [^*] | '*'+ [^*/] )* '*'+ '/'

Whitespace ::= ( #x09 | #x0A | #x0D | #x20 )*
```

```yml
parsers: [grammar, expression]
productions:
  production: { ignore }
  expression: { ignore }
  sequence: { ignore }
  difference: { ignore }
  item: { ignore }
  charSet: { ignore }
  String: { ignore }
  grammar: { nomap, ignore }
  primary:
    ignore:
    nomap:
    "$": { map: eof_in_primary, ignore }
    2: { map: any_in_primary, ignore }
    CharCode: { map: char_code_in_primary, ignore }
    r#Identifier: { map: identifier_in_primary, lookahead: "S [^:]", ignore }
    6: { ignore }
  CharCode: { unwrap, ignore }
  CharSetChar:
    nomap:
    ignore:
    1: { ignore }
  Identifier: { nomap, ignore }
  S: { ignore }
  Comment:
    ignore:
    1:
      0:
        0: { ignore }
        1: { ignore }
  Whitespace: { ignore }
```
