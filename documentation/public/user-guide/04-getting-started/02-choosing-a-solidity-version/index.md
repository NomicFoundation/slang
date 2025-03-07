# 4.2. Choosing a Solidity Version

Slang aims to support all Solidity language versions, starting with `0.4.11`, and adding support for all future versions as they are released.

In order to use many of the Slang APIs, you will need to specify the Solidity version that you want to work with.
You can see a list of all supported Solidity versions [here](../../../solidity-specification/supported-versions.md).

You can also access this list programmatically, by using the `LanguageFacts` API:

```ts title="supported-versions.mts"
--8<-- "documentation/public/user-guide/04-getting-started/02-choosing-a-solidity-version/examples/01-supported-versions.test.mts"
```
