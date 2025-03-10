# 4.2. Choosing a Solidity Version

## Supported Versions

Slang aims to support all Solidity language versions, starting with `0.4.11`, and adding support for all future versions as they are released.

In order to use many of the Slang APIs, you will need to specify the Solidity version that you want to work with.
You can see a list of all supported Solidity versions [here](../../../solidity-grammar/supported-versions.md).

You can also access this list programmatically, by using the `LanguageFacts` API:

```ts title="supported-versions.mts"
--8<-- "documentation/public/user-guide/04-getting-started/02-choosing-a-solidity-version/examples/01-supported-versions.test.mts"
```

## Inferring Compatible Solidity Versions

For cases where you don't know in advance which version of Solidity to use, the `LanguageFacts` API provides a utility to generate a list of compatible versions. It uses the [version pragmas](https://docs.soliditylang.org/en/develop/layout-of-source-files.html#version-pragma) defined in your Solidity source file to filter the list of versions supported by Slang, returning only the compatible ones.

The list is sorted from earliest to latest, so the last item in the list will be the most up-to-date version of Solidity supported by your source.

```ts title="infer-versions.mts"
--8<-- "documentation/public/user-guide/04-getting-started/02-choosing-a-solidity-version/examples/02-infer-versions.test.mts"
```
