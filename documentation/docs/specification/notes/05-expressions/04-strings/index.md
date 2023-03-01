<!-- markdownlint-configure-file { "first-line-heading": { "level": 2 } } -->

## String Literals

String literals are written with either double or single-quotes (`"foo"` or `'bar'`), and they can also be split into multiple consecutive parts (`"foo" "bar"` is equivalent to `"foobar"`) which can be helpful when dealing with long strings. They do not imply trailing zeroes as in C; `"foo"` represents three bytes, not four. As with integer literals, their type can vary, but they are implicitly convertible to `bytes1`, ..., `bytes32` if they fit.

String literals can only contain printable ASCII characters, which means the characters between `0x20` and `0x7E` inclusively.

## Unicode Literals

While regular string literals can only contain ASCII, unicode literals (prefixed with the keyword `unicode`) can contain any valid UTF-8 sequence. They also support the very same escape sequences as regular string literals.

```solidity
string memory a = unicode"Hello 😃";
```

## Hexadecimal Literals

Hexadecimal literals are prefixed with the keyword `hex` and are enclosed in double or single-quotes (`hex"001122FF"`, `hex'0011_22_FF'`). Their content must be hexadecimal digits which can optionally use a single underscore as separator between byte boundaries. The value of the literal will be the binary representation of the hexadecimal sequence.

Hexadecimal literals behave like string literals and have the same convertibility restrictions. Additionally, multiple hexadecimal literals separated by whitespace are concatenated into a single literal: `hex"00112233" hex"44556677"` is equivalent to `hex"0011223344556677"`

## Escape Sequences

String literals also support the following escape characters:

- `\<newline>` (escapes an actual newline)
- `\\` (backslash)
- `\'` (single quote)
- `\"` (double quote)
- `\n` (newline)break
- `\r` (carriage return)
- `\t` (tab)
- `\xNN` (hex escape, takes a hex value and inserts the appropriate byte)
- `\uNNNN` (unicode escape, takes a Unicode code point and inserts an UTF-8 sequence)

Any Unicode line terminator which is not a newline (i.e. LF, VF, FF, CR, NEL, LS, PS) is considered to terminate the string literal. Newline only terminates the string literal if it is not preceded by a `\`.

!!! danger "Breaking Changes"

    Escape sequences  `\b`, `\f` and `\v` have been removed in `v0.8.0`. You can use `\x08`, `\x0c` and `\x0b` instead.
    --8<-- "specification/notes/05-expressions/04-strings/tests/escape-sequences/b/generated/combined"
    ---
    --8<-- "specification/notes/05-expressions/04-strings/tests/escape-sequences/f/generated/combined"
    ---
    --8<-- "specification/notes/05-expressions/04-strings/tests/escape-sequences/v/generated/combined"

--8<-- "snippets/under-construction.md"
