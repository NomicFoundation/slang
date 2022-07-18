<!-- markdownlint-configure-file { "first-line-heading": { "level": 2 } } -->

## Unchecked Blocks

By default, all arithmetic operations are checked for underflow or overflow, which mean that if the result of an
operation falls outside the value range of the type, the call is reverted through a failing assertion. This can be
disabled using the `unchecked` block, resulting in wrapping arithmetic.

--8<-- "snippets/under-construction.md"
