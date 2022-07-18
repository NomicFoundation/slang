<!-- markdownlint-configure-file { "first-line-heading": { "level": 2 } } -->

## License Specifier Comment

This comment line declares that the source code is licensed under the GPL version 3.0. Machine-readable license specifiers are important in a setting where publishing the source code is the default. The comment is recognized by the compiler anywhere in the file at the file level, but it is recommended to put it at the top of the file.

```solidity
// SPDX-License-Identifier: GPL-3.0
```

When omitted, the compiler produces a warning to add one. The compiler does not validate that the license is part of [the list allowed by SPDX](https://spdx.org/licenses/), but it does include the supplied string in the metadata.

If you do not want to specify a license or if the source code is not open-source, please use the special value `UNLICENSED`. Note that `UNLICENSED` (no usage allowed, not present in SPDX license list) is different from `UNLICENSE` (grants all rights to everyone).

!!! danger "Breaking Changes"

    Starting from `v0.8.8`, multiple license specifiers will result in a parse error:
    --8<-- "specification/notes/01-file-structure/01-license-specifier/tests/multiple/generated/combined"

--8<-- "snippets/under-construction.md"
