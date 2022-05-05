# W3C EBNF Grammar

This grammar describes W3C EBNF, using [W3C EBNF](https://www.w3.org/TR/REC-xml/#sec-notation)

The valid locations for whitespace and comments are explicitly specified, rather than being ambient.

```ebnf
grammar = ( S production )* S $ ;

production = Identifier S '=' S expression S ';' ;

expression = sequence ( S '|' S sequence )* ;

sequence = difference ( S difference )* ;

difference = item ( S '-' S item )? ;

item = primary ( '?' | '*' | '+' )? ;

primary = CharRange | CharSet | '$' | '.' | String | Identifier | '(' S expression S ')' ;

CharSet = '[' '^'? ( CharSetChar ( '-' CharSetChar )? )* ']' ;
CharSetChar = CharCode | [^#x09#x0A#x0D#x23#x5D] /* TAB or LF or CR or '#' or ']' */ ;
CharCode = '#x' HexDigit+ ;

String = '\'' StringChar* '\'' ;
StringChar = [^'\] | '\\' ( '\'' | '\\' | 'u{' HexDigit+ '}' ) ;

CharRange = SingleCharString '…' SingleCharString ;
SingleCharString = '\'' StringChar '\'' ;

HexDigit = '0'…'9' | 'a'…'f' | 'A'…'F' ;

Identifier = IdentifierStart IdentifierFollow* ;
IdentifierStart = ( '_' | 'a'…'z' | 'A'…'Z' ) ;
IdentifierFollow = IdentifierStart | '0'…'9' ;

S = ( Whitespace | Comment )* ;

Comment = '/*' ( [^*] | '*'+ [^*/] )* '*'+ '/' ;

Whitespace = ( '\u{09}' | '\u{0A}' | '\u{0D}' | '\u{20}' )+ ;
```
