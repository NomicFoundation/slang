---
"@nomicfoundation/slang": minor
---

Add `LanguageUtils::infer_language_versions(source_code) -> Version[]` API, which will analyze version pragmas inside a source file, and return a list of supported language versions that they allow. This can be used to select a valid language version to use with the rest of Slang APIs. Please see the [Choosing a Solidity Version](https://nomicfoundation.github.io/slang/1.1.0/user-guide/04-getting-started/02-choosing-a-solidity-version/) guide for more information.
