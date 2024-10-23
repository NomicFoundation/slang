# changelog

## 0.18.3

### Patch Changes

-   [#1130](https://github.com/NomicFoundation/slang/pull/1130) [`a97b27d`](https://github.com/NomicFoundation/slang/commit/a97b27d918c9719edddbb2816db3486f43f7ec24) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - fix a bug where CST nodes are invalidated after using AST types

## 0.18.2

### Patch Changes

-   [#1126](https://github.com/NomicFoundation/slang/pull/1126) [`e1d9748`](https://github.com/NomicFoundation/slang/commit/e1d974860d5ea7d8e6a700ba126ef7d3e08254f0) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - fix wasm type exports for the npm package

-   [#1122](https://github.com/NomicFoundation/slang/pull/1122) [`bbb5323`](https://github.com/NomicFoundation/slang/commit/bbb53231efac2b46e3add2c007aaba81e4699b8c) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - support Solidity `0.8.28` release

## 0.18.1

### Patch Changes

-   [#1123](https://github.com/NomicFoundation/slang/pull/1123) [`bdb0ef3`](https://github.com/NomicFoundation/slang/commit/bdb0ef3167eb6d4496b33147437d3adc9714b92c) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - fix a minor issue with npm package ESM imports

## 0.18.0

### Minor Changes

-   [#1120](https://github.com/NomicFoundation/slang/pull/1120) [`25eef3e`](https://github.com/NomicFoundation/slang/commit/25eef3e8c64aa03e195aefbba0867bfa7646b821) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - migrate NAPI front-end to WASM and ESM

-   [#1120](https://github.com/NomicFoundation/slang/pull/1120) [`25eef3e`](https://github.com/NomicFoundation/slang/commit/25eef3e8c64aa03e195aefbba0867bfa7646b821) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - add `TerminalKindExtensions.is_valid()` API to distinguish correctly-parsed and erroneous nodes

-   [#1117](https://github.com/NomicFoundation/slang/pull/1117) [`be7bb79`](https://github.com/NomicFoundation/slang/commit/be7bb79c8f497e5283674878dacaa0fd1ec6e68a) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - rename `Language` API to `Parser`, in preparation for introducing a multi-file compilation API.

-   [#1116](https://github.com/NomicFoundation/slang/pull/1116) [`c88f9b5`](https://github.com/NomicFoundation/slang/commit/c88f9b5d50dabe111b9d1b6cb4e6b6b9e276f1da) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - merge `language`, `parse_error`, `parse_output` namespaces into the `parser` namespace.

-   [#1115](https://github.com/NomicFoundation/slang/pull/1115) [`96df645`](https://github.com/NomicFoundation/slang/commit/96df64514ffec22ac41af38cc9f91e7b1e260a25) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - merge `cursor`, `kinds`, `query`, and `text_index` namespaces into the `cst` namespace.

-   [#1120](https://github.com/NomicFoundation/slang/pull/1120) [`25eef3e`](https://github.com/NomicFoundation/slang/commit/25eef3e8c64aa03e195aefbba0867bfa7646b821) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - unify API methods on `TerminalNode` and `NonTerminalNode`, and add type assertions and guards to both types

-   [#1120](https://github.com/NomicFoundation/slang/pull/1120) [`25eef3e`](https://github.com/NomicFoundation/slang/commit/25eef3e8c64aa03e195aefbba0867bfa7646b821) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - expose edges and edge labels on CST nodes via `Node.children()` method. This allows distinguishing between children of the same node based on their label/role in the parent, even if they have the same kind.

-   [#1120](https://github.com/NomicFoundation/slang/pull/1120) [`25eef3e`](https://github.com/NomicFoundation/slang/commit/25eef3e8c64aa03e195aefbba0867bfa7646b821) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - add `TerminalNode.id` and `Nonterminal.id` properties to get a numeric ID that can be used in indexing/comparison at runtime.

-   [#1120](https://github.com/NomicFoundation/slang/pull/1120) [`25eef3e`](https://github.com/NomicFoundation/slang/commit/25eef3e8c64aa03e195aefbba0867bfa7646b821) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - add `TerminalKindExtensions.is_trivia()` API to distinguish between trivia nodes and other contentful nodes

### Patch Changes

-   [#1096](https://github.com/NomicFoundation/slang/pull/1096) [`15c437c`](https://github.com/NomicFoundation/slang/commit/15c437c6c4902cd43e0027f750ba59e8f22f47f9) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - add support for Solidity `0.8.27`.

-   [#1120](https://github.com/NomicFoundation/slang/pull/1120) [`25eef3e`](https://github.com/NomicFoundation/slang/commit/25eef3e8c64aa03e195aefbba0867bfa7646b821) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - change `Parser::new()` constructor to `Parser::create()` static method.

-   [#1097](https://github.com/NomicFoundation/slang/pull/1097) [`e17af22`](https://github.com/NomicFoundation/slang/commit/e17af22cc4c1d373be751e525963f45ddf4dd3c3) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - Fix the grammar of keywords (`blobbasefee`, `blobhash`, `mcopy`, `tload`, `tstore`) added in `0.8.24`, as they were actually reserved in `0.8.25`.

## 0.17.0

### Minor Changes

-   [#1079](https://github.com/NomicFoundation/slang/pull/1079) [`43b389e`](https://github.com/NomicFoundation/slang/commit/43b389e9db59054774e6346dca7e71c307192ebb) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - Move the Rust CLI into a separate `slang_solidity_cli` crate.

### Patch Changes

-   [#1086](https://github.com/NomicFoundation/slang/pull/1086) [`f749e53`](https://github.com/NomicFoundation/slang/commit/f749e536a377b1612b623eb8277abb2b59019026) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - The grammar for `VersionExpressionSet` has changed to more accurately model the allowed structure.

## 0.16.0

### Minor Changes

-   [#1030](https://github.com/NomicFoundation/slang/pull/1030) [`7e467ce`](https://github.com/NomicFoundation/slang/commit/7e467ce199cb07acb443da9f542fbcc74f2a5321) Thanks [@ggiraldez](https://github.com/ggiraldez)! - Tree Query Language: queries now ignore trivia nodes.

-   [#1030](https://github.com/NomicFoundation/slang/pull/1030) [`7e467ce`](https://github.com/NomicFoundation/slang/commit/7e467ce199cb07acb443da9f542fbcc74f2a5321) Thanks [@ggiraldez](https://github.com/ggiraldez)! - Tree Query Language: remove the ellipsis query `...` operator making it implicit, add an adjacency operator `.`.

### Patch Changes

-   [#1015](https://github.com/NomicFoundation/slang/pull/1015) [`369ee30`](https://github.com/NomicFoundation/slang/commit/369ee309325ef57c7cd6f29e2f7adc4f9ec09c88) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - upgrade to rust `1.79.0`.

-   [#1052](https://github.com/NomicFoundation/slang/pull/1052) [`54c9067`](https://github.com/NomicFoundation/slang/commit/54c9067aa6c587ff93cac6575726a9068a9bb758) Thanks [@ggiraldez](https://github.com/ggiraldez)! - Tree Query Language: Compute row and column information for query parser errors.

-   [#1048](https://github.com/NomicFoundation/slang/pull/1048) [`c408caa`](https://github.com/NomicFoundation/slang/commit/c408caae1826095cc2f2c01caf9be58ab5ff8eee) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - support Yul multiple variables declaration

-   [#1047](https://github.com/NomicFoundation/slang/pull/1047) [`2b32045`](https://github.com/NomicFoundation/slang/commit/2b3204549af27ea3782da2a9a2de470db13a7402) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - inline `MemberAccess` enum into the parent `MemberAccessExpression`

-   [#1062](https://github.com/NomicFoundation/slang/pull/1062) [`6b05496`](https://github.com/NomicFoundation/slang/commit/6b05496cbd19b5a7f65033fb223c1bcd3d448738) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - fix stack assignments operator `=:`.

-   [#1052](https://github.com/NomicFoundation/slang/pull/1052) [`54c9067`](https://github.com/NomicFoundation/slang/commit/54c9067aa6c587ff93cac6575726a9068a9bb758) Thanks [@ggiraldez](https://github.com/ggiraldez)! - Tree Query Language: Improve reporting when an error occurs attempting to parse edge labels or node kinds.

-   [#1037](https://github.com/NomicFoundation/slang/pull/1037) [`2a74f91`](https://github.com/NomicFoundation/slang/commit/2a74f91ed8e67fc3d315afd49f593dfef52f0e4d) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - fix `ExponentiationExpression` associativity before `0.8.0`

## 0.15.1

### Patch Changes

-   [#1012](https://github.com/NomicFoundation/slang/pull/1012) [`9ca51b4`](https://github.com/NomicFoundation/slang/commit/9ca51b431e6d9e52b537683bf618d8852103936b) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - expose `QueryMatch` to public API

## 0.15.0

### Minor Changes

-   [#975](https://github.com/NomicFoundation/slang/pull/975) [`46b1dde`](https://github.com/NomicFoundation/slang/commit/46b1dde2e39903cff6398d5da3a4d1a1820f0095) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - rename `QueryResult` to `QueryMatch`, and its `bindings` to `captures`.

-   [#971](https://github.com/NomicFoundation/slang/pull/971) [`be943b7`](https://github.com/NomicFoundation/slang/commit/be943b7349e4c4b7589d93cf670bc8453125b229) Thanks [@Xanewok](https://github.com/Xanewok)! - Rename `RuleKind` to `NonterminalKind`, `TokenKind` to `TerminalKind`, and `NodeLabel` to `EdgeLabel`.

-   [#963](https://github.com/NomicFoundation/slang/pull/963) [`a5593f9`](https://github.com/NomicFoundation/slang/commit/a5593f981b1df133449264c995c91ac738981474) Thanks [@Xanewok](https://github.com/Xanewok)! - Introduce a `Diagnostic` API for compiler errors, warnings etc.

### Patch Changes

-   [#996](https://github.com/NomicFoundation/slang/pull/996) [`cdc153d`](https://github.com/NomicFoundation/slang/commit/cdc153dbb149c277f6f0d00ed95bbac1e5bec8f1) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - add support for version `0.8.26`.

-   [#983](https://github.com/NomicFoundation/slang/pull/983) [`ea31417`](https://github.com/NomicFoundation/slang/commit/ea3141741ef3e491b2125f0d24d5db58c2f5d600) Thanks [@ggiraldez](https://github.com/ggiraldez)! - Expose the language root non-terminal kind at `Language.rootKind()`.

-   [#965](https://github.com/NomicFoundation/slang/pull/965) [`61b6b06`](https://github.com/NomicFoundation/slang/commit/61b6b06deaa1db86a21c5fed675cd665ed2c42a4) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - refactor CST building and querying utilities into a separate `metaslang_cst` crate.

-   [#997](https://github.com/NomicFoundation/slang/pull/997) [`84ad856`](https://github.com/NomicFoundation/slang/commit/84ad856b344e7c17376b38c420a7952556dc4ff5) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - Support stack assignments in Yul before `0.5.0`.

## 0.14.2

### Patch Changes

-   [#948](https://github.com/NomicFoundation/slang/pull/948) [`ce88cb7`](https://github.com/NomicFoundation/slang/commit/ce88cb7a6fd945b59ccc967cfd20f423dadc36fc) Thanks [@Xanewok](https://github.com/Xanewok)! - Restrict the grammar to correctly only allow an identifier in Yul variable declaration

-   [#945](https://github.com/NomicFoundation/slang/pull/945) [`e8f80d8`](https://github.com/NomicFoundation/slang/commit/e8f80d867b4b9d02413f42a8ece2630a43bc7494) Thanks [@Xanewok](https://github.com/Xanewok)! - Support `.address` built-in access in Yul paths

## 0.14.1

### Patch Changes

-   [#943](https://github.com/NomicFoundation/slang/pull/943) [`a561fb1`](https://github.com/NomicFoundation/slang/commit/a561fb161eb7c18c838c85f71d132764d1d04050) Thanks [@Xanewok](https://github.com/Xanewok)! - Support Solidity 0.8.25

## 0.14.0

### Minor Changes

-   [#753](https://github.com/NomicFoundation/slang/pull/753) [`b35c763`](https://github.com/NomicFoundation/slang/commit/b35c7630ab7240304e67a43734700cf359acde0b) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Add tree query implementation as `Query::parse` and `Cursor::query`

-   [#755](https://github.com/NomicFoundation/slang/pull/755) [`8c260fc`](https://github.com/NomicFoundation/slang/commit/8c260fcb7e3111191cd33dd527817fb51119eac4) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - support parsing NatSpec comments

-   [#908](https://github.com/NomicFoundation/slang/pull/908) [`ab3688b`](https://github.com/NomicFoundation/slang/commit/ab3688bb99a60862c506566ac6122cd9c1155c57) Thanks [@Xanewok](https://github.com/Xanewok)! - Changed the cst.NodeType in TS to use more descriptive string values rather than 0/1 integers

-   [#886](https://github.com/NomicFoundation/slang/pull/886) [`0125717`](https://github.com/NomicFoundation/slang/commit/0125717fb0b48a5342a8452f18080db13e68fb6b) Thanks [@Xanewok](https://github.com/Xanewok)! - Add `TokenKind::is_trivia`

-   [#887](https://github.com/NomicFoundation/slang/pull/887) [`dff1201`](https://github.com/NomicFoundation/slang/commit/dff12011c549d68b20ecd54251af764643fb72db) Thanks [@Xanewok](https://github.com/Xanewok)! - Add support for constant function modifier removed in 0.5.0

-   [#885](https://github.com/NomicFoundation/slang/pull/885) [`a9bd8da`](https://github.com/NomicFoundation/slang/commit/a9bd8da018469739832f71e38437caa83087baf0) Thanks [@Xanewok](https://github.com/Xanewok)! - Flatten the trivia syntax nodes into sibling tokens

-   [#908](https://github.com/NomicFoundation/slang/pull/908) [`ab3688b`](https://github.com/NomicFoundation/slang/commit/ab3688bb99a60862c506566ac6122cd9c1155c57) Thanks [@Xanewok](https://github.com/Xanewok)! - Add RuleNode/TokenNode::toJSON() in the TypeScript API

### Patch Changes

-   [#801](https://github.com/NomicFoundation/slang/pull/801) [`ecbba49`](https://github.com/NomicFoundation/slang/commit/ecbba49c7ac25e37b8d317fb60fab7340c0628a5) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - unreserve pragma keywords in all versions

-   [#869](https://github.com/NomicFoundation/slang/pull/869) [`951b58d`](https://github.com/NomicFoundation/slang/commit/951b58ddb3eaea600ddf44427a82649761c6b651) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - support dots in yul identifiers from `0.5.8` till `0.7.0`

-   [#890](https://github.com/NomicFoundation/slang/pull/890) [`1ff8599`](https://github.com/NomicFoundation/slang/commit/1ff85993f25d92b38d0a500baa6ee48669a1b62a) Thanks [@Xanewok](https://github.com/Xanewok)! - Mark `override` as being a valid attribute only after 0.6.0

-   [#800](https://github.com/NomicFoundation/slang/pull/800) [`d1827ff`](https://github.com/NomicFoundation/slang/commit/d1827ff7e1010493ff5487532a5ee0c77d355aa2) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - support unicode characters in string literals up to `0.7.0`

-   [#797](https://github.com/NomicFoundation/slang/pull/797) [`86f36d7`](https://github.com/NomicFoundation/slang/commit/86f36d71e60a44261ec114339e931dd3d24cd4a4) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - fix source locations for unicode characters in error reports

-   [#854](https://github.com/NomicFoundation/slang/pull/854) [`4b8970b`](https://github.com/NomicFoundation/slang/commit/4b8970b47ef7a2d1d51339cf5020a3e0f168b9aa) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - parse line breaks without newlines

-   [#844](https://github.com/NomicFoundation/slang/pull/844) [`f62de9e`](https://github.com/NomicFoundation/slang/commit/f62de9ea3fc2049ee11e5dbeff3dc51eb1ca984e) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - fix parsing empty `/**/` comments

-   [#799](https://github.com/NomicFoundation/slang/pull/799) [`303dda9`](https://github.com/NomicFoundation/slang/commit/303dda95c08b20450d03116765c210ece64a0864) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - prevent parsing multiple literals under `StringExpression` before `0.5.14`

-   [#847](https://github.com/NomicFoundation/slang/pull/847) [`6b6f260`](https://github.com/NomicFoundation/slang/commit/6b6f2603e3ba07c0a7dede0f96082369dc1df940) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - prioritize parsing `MultiLineComment` over `MultiLineNatSpecComment`

-   [#796](https://github.com/NomicFoundation/slang/pull/796) [`59e1e53`](https://github.com/NomicFoundation/slang/commit/59e1e53e7efa52355c273d7cef1a3974de13d88d) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - add `public` and `internal` to `UnnamedFunctionAttribute` till `0.5.0`

-   [#756](https://github.com/NomicFoundation/slang/pull/756) [`e839817`](https://github.com/NomicFoundation/slang/commit/e8398173f62d48596669628afc7c8b3572a15291) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - fix parsing `payable` primary expressions

-   [#851](https://github.com/NomicFoundation/slang/pull/851) [`67dfde8`](https://github.com/NomicFoundation/slang/commit/67dfde81a6d00101a9ed133104f15da5d46662b6) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - fix selection order of prefix/postfix AST fields

-   [#857](https://github.com/NomicFoundation/slang/pull/857) [`f677d5e`](https://github.com/NomicFoundation/slang/commit/f677d5eff40c4bfcf1db2fc4e63cdf37457fe467) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - rename `FieldName` to `NodeLabel`

-   [#852](https://github.com/NomicFoundation/slang/pull/852) [`ca79eca`](https://github.com/NomicFoundation/slang/commit/ca79ecaa522e531420b42ffba67da192c1e5fdb2) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - allow parsing `ColonEqual` as two separate tokens before `0.5.5`

-   [#889](https://github.com/NomicFoundation/slang/pull/889) [`ce5050f`](https://github.com/NomicFoundation/slang/commit/ce5050f95195fdd018a38a0351d8525f7d62073a) Thanks [@Xanewok](https://github.com/Xanewok)! - Support `delete` as an expression rather than a statement

-   [#923](https://github.com/NomicFoundation/slang/pull/923) [`bb30fc1`](https://github.com/NomicFoundation/slang/commit/bb30fc1e28a0fe806f8954a0d2779d903f3f4da7) Thanks [@Xanewok](https://github.com/Xanewok)! - Support arbitrary ASCII escape sequences in string literals until 0.4.25

-   [#887](https://github.com/NomicFoundation/slang/pull/887) [`dff1201`](https://github.com/NomicFoundation/slang/commit/dff12011c549d68b20ecd54251af764643fb72db) Thanks [@Xanewok](https://github.com/Xanewok)! - Support view and pure function modifiers only from 0.4.16

-   [#800](https://github.com/NomicFoundation/slang/pull/800) [`d1827ff`](https://github.com/NomicFoundation/slang/commit/d1827ff7e1010493ff5487532a5ee0c77d355aa2) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - rename `AsciiStringLiteral` to `StringLiteral`

-   [#838](https://github.com/NomicFoundation/slang/pull/838) [`ad98d1c`](https://github.com/NomicFoundation/slang/commit/ad98d1c7d9f9f7cb12b4b6184c04c9b680e6d70a) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - upgrade to rust `1.76.0`

-   [#849](https://github.com/NomicFoundation/slang/pull/849) [`5c42e0e`](https://github.com/NomicFoundation/slang/commit/5c42e0ef5f3afe0355614967cb6d2daa31518ccf) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - add `override` and `virtual` to `ConstructorAttribute`

-   [#862](https://github.com/NomicFoundation/slang/pull/862) [`5e37ea0`](https://github.com/NomicFoundation/slang/commit/5e37ea0c40e929e0888b6297fa6dd92952d9cd73) Thanks [@Xanewok](https://github.com/Xanewok)! - allow call options as a post-fix expression

-   [#786](https://github.com/NomicFoundation/slang/pull/786) [`0bfa6b7`](https://github.com/NomicFoundation/slang/commit/0bfa6b7397cd25aca713b30628c6d06e761b416a) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - support Yul label statements before `0.5.0`

-   [#839](https://github.com/NomicFoundation/slang/pull/839) [`2d698eb`](https://github.com/NomicFoundation/slang/commit/2d698ebe469110b85f539d6e0c75b503cd4ce57e) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - support string literals in version pragmas

-   [#891](https://github.com/NomicFoundation/slang/pull/891) [`70c9d7d`](https://github.com/NomicFoundation/slang/commit/70c9d7deebddb0f22114b7b05ddc85da6dcceaaf) Thanks [@Xanewok](https://github.com/Xanewok)! - Fix parsing `<NUMBER>.member` member access expression

-   [#842](https://github.com/NomicFoundation/slang/pull/842) [`2069126`](https://github.com/NomicFoundation/slang/commit/20691263fb6967195bee30fba92abdfb06daa6fa) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - add `private` to `UnnamedFunctionAttribute` till `0.5.0`

-   [#840](https://github.com/NomicFoundation/slang/pull/840) [`7fb0d20`](https://github.com/NomicFoundation/slang/commit/7fb0d20655024daf71c872a6ef95aa30277a1366) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - allow `var` in `TupleDeconstructionStatement` before `0.5.0`

## 0.13.1

### Patch Changes

-   [#748](https://github.com/NomicFoundation/slang/pull/748) [`c289cbf7`](https://github.com/NomicFoundation/slang/commit/c289cbf7e22118881818b82d0ffc5933a424a7aa) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - Properly parse EVM built-ins up till Paris/Solidity 0.8.18

## 0.13.0

### Minor Changes

-   [#710](https://github.com/NomicFoundation/slang/pull/710) [`2025b6cb`](https://github.com/NomicFoundation/slang/commit/2025b6cb23dc320b413b482ed1fe8455229b7d84) Thanks [@Xanewok](https://github.com/Xanewok)! - CST children nodes are now named

-   [#723](https://github.com/NomicFoundation/slang/pull/723) [`b3dc6bcd`](https://github.com/NomicFoundation/slang/commit/b3dc6bcdc1834d266a87d483927894617bf8e817) Thanks [@Xanewok](https://github.com/Xanewok)! - Properly parse unreserved keywords in an identifier position, i.e. `from`, `emit`, `global` etc.

-   [#728](https://github.com/NomicFoundation/slang/pull/728) [`662a672c`](https://github.com/NomicFoundation/slang/commit/662a672cd661b9f1bf4c18587acf68111fd1f2e8) Thanks [@Xanewok](https://github.com/Xanewok)! - Remove Language#scan API; use the parser API instead

-   [#719](https://github.com/NomicFoundation/slang/pull/719) [`1ad6bb37`](https://github.com/NomicFoundation/slang/commit/1ad6bb37337aa28d9344380c5c9eb1945e908271) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - introduce strong types for all Solidity non terminals in the TypeScript API.

### Patch Changes

-   [#719](https://github.com/NomicFoundation/slang/pull/719) [`1ad6bb37`](https://github.com/NomicFoundation/slang/commit/1ad6bb37337aa28d9344380c5c9eb1945e908271) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - unify Rust/TypeScript node helpers: `*_with_kind()`, `*_with_kinds()`, `*_is_kind()`), ...

-   [#731](https://github.com/NomicFoundation/slang/pull/731) [`3deaea2e`](https://github.com/NomicFoundation/slang/commit/3deaea2eb82ce33dbccc54d1a79b9cf5657385ac) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - add `RuleNode.unparse()` to the TypeScript API

## 0.12.0

### Minor Changes

-   [#699](https://github.com/NomicFoundation/slang/pull/699) [`ddfebfe9`](https://github.com/NomicFoundation/slang/commit/ddfebfe988345136007431f8ea2efac19fd7e887) Thanks [@Xanewok](https://github.com/Xanewok)! - Remove `ProductionKind` in favor of `RuleKind`

-   [#699](https://github.com/NomicFoundation/slang/pull/699) [`ddfebfe9`](https://github.com/NomicFoundation/slang/commit/ddfebfe988345136007431f8ea2efac19fd7e887) Thanks [@Xanewok](https://github.com/Xanewok)! - Allow parsing individual precedence expressions, like `ShiftExpression`

-   [#665](https://github.com/NomicFoundation/slang/pull/665) [`4b5f8b46`](https://github.com/NomicFoundation/slang/commit/4b5f8b467d4cbab72cf27a539bb5ca8c71090dd6) Thanks [@Xanewok](https://github.com/Xanewok)! - Remove the CST Visitor API in favor of the Cursor API

-   [#666](https://github.com/NomicFoundation/slang/pull/666) [`0434b68c`](https://github.com/NomicFoundation/slang/commit/0434b68c9ef9cd1d1dcc07d7ed50e6d63645319b) Thanks [@Xanewok](https://github.com/Xanewok)! - Add `Node::unparse()` that allows to reconstruct the source code from the CST node

-   [#675](https://github.com/NomicFoundation/slang/pull/675) [`daea4b7f`](https://github.com/NomicFoundation/slang/commit/daea4b7f954ff1e918b9191aff40ee95c10a4db2) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - rename `Cursor`'s `pathRuleNodes()` to `ancestors()` in the NodeJS API.

-   [#676](https://github.com/NomicFoundation/slang/pull/676) [`b496d361`](https://github.com/NomicFoundation/slang/commit/b496d36120700347bcbcc25b948eb46814fd5412) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - Fix NAPI `cursor` types and expose `cursor.depth`.

### Patch Changes

-   [#685](https://github.com/NomicFoundation/slang/pull/685) [`b5fca94a`](https://github.com/NomicFoundation/slang/commit/b5fca94af917a2f0418c224b3101885c02e5cb9c) Thanks [@Xanewok](https://github.com/Xanewok)! - `bytes` is now properly recognized as a reserved word

-   [#660](https://github.com/NomicFoundation/slang/pull/660) [`97028991`](https://github.com/NomicFoundation/slang/commit/9702899164f0540a49f2e0f7f19d82fbd04b1d1b) Thanks [@Xanewok](https://github.com/Xanewok)! - Drop List suffix from collection grammar rule names

## 0.11.0

### Minor Changes

-   [#625](https://github.com/NomicFoundation/slang/pull/625) [`7bb650b`](https://github.com/NomicFoundation/slang/commit/7bb650b12ae793a318dc5b7839fb93915c88828e) Thanks [@Xanewok](https://github.com/Xanewok)! - The CST Cursor now implements the Iterator trait as part of the Rust API

-   [#647](https://github.com/NomicFoundation/slang/pull/647) [`b1dced3`](https://github.com/NomicFoundation/slang/commit/b1dced355ce83f3bd858c02837d67665f7ef281d) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - Require specifying an initial offset when creating a CST cursor.

### Patch Changes

-   [#648](https://github.com/NomicFoundation/slang/pull/648) [`2327bf5`](https://github.com/NomicFoundation/slang/commit/2327bf5d8c40d85edd0cc80fe9e36d367a1a3336) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - Support Solidity v0.8.22.

-   [#623](https://github.com/NomicFoundation/slang/pull/623) [`80114a8`](https://github.com/NomicFoundation/slang/commit/80114a833dc8249447c382bf457215b1a4d9e5ae) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Correct the types in the TS api by adding the correct namespaces to type references

## 0.10.1

### Patch Changes

-   [#615](https://github.com/NomicFoundation/slang/pull/615) [`06cbbe8`](https://github.com/NomicFoundation/slang/commit/06cbbe88bc68928ad44046a96c31ad6e53fbf76c) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - `cursor` method is now exposed in Typescript API

## 0.10.0

### Minor Changes

-   [#595](https://github.com/NomicFoundation/slang/pull/595) [`1a258c4`](https://github.com/NomicFoundation/slang/commit/1a258c49432eac06dac7055bc427e68af1fa3875) Thanks [@Xanewok](https://github.com/Xanewok)! - Attempt error recovery when parsing incomplete lists

-   [#564](https://github.com/NomicFoundation/slang/pull/564) [`e326a06`](https://github.com/NomicFoundation/slang/commit/e326a064da559c974fbb7a199090e9e5a313abb8) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Parsing operators with missing operands should no longer panic

-   [#564](https://github.com/NomicFoundation/slang/pull/564) [`e326a06`](https://github.com/NomicFoundation/slang/commit/e326a064da559c974fbb7a199090e9e5a313abb8) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Inline parse rules are no longer exposed to the API.

-   [#564](https://github.com/NomicFoundation/slang/pull/564) [`e326a06`](https://github.com/NomicFoundation/slang/commit/e326a064da559c974fbb7a199090e9e5a313abb8) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Scanners are no longer available as methods - use next_token instead

-   [#564](https://github.com/NomicFoundation/slang/pull/564) [`e326a06`](https://github.com/NomicFoundation/slang/commit/e326a064da559c974fbb7a199090e9e5a313abb8) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Scanners are now grouped into contexts to deal with contextual scanning

### Patch Changes

-   [#601](https://github.com/NomicFoundation/slang/pull/601) [`cbd2a79`](https://github.com/NomicFoundation/slang/commit/cbd2a79658849c0029bb6a5ccc0b086564c28fe0) Thanks [@Xanewok](https://github.com/Xanewok)! - Attempt parser error recovery between bracket delimiters

-   [#599](https://github.com/NomicFoundation/slang/pull/599) [`4bbad48`](https://github.com/NomicFoundation/slang/commit/4bbad48d45ae7bde8a22198b33f790b7c792b319) Thanks [@Xanewok](https://github.com/Xanewok)! - Use correct versions for the `revert` and `global` keywords

-   [#561](https://github.com/NomicFoundation/slang/pull/561) [`cb6a138`](https://github.com/NomicFoundation/slang/commit/cb6a1384cb6f04926def3e4c1fe7a0b12a67143c) Thanks [@Xanewok](https://github.com/Xanewok)! - Add preliminary documentation for the `solidity_language` Rust package

-   [#603](https://github.com/NomicFoundation/slang/pull/603) [`be59a10`](https://github.com/NomicFoundation/slang/commit/be59a10c937542f0413a34fd84d84ec4d4400f6d) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - upgrade to rust 1.72.0

## 0.9.0

### Minor Changes

-   [#540](https://github.com/NomicFoundation/slang/pull/540) [`0d04f95`](https://github.com/NomicFoundation/slang/commit/0d04f959bf1f5831c912d5109de3d933cfaa6266) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Add a Rust Cursor API and refactor the Rust Visitor API to run on top of it.

-   [#540](https://github.com/NomicFoundation/slang/pull/540) [`0d04f95`](https://github.com/NomicFoundation/slang/commit/0d04f959bf1f5831c912d5109de3d933cfaa6266) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Move Visitor et al to node:: namespace, which is where Cursor is.

-   [#540](https://github.com/NomicFoundation/slang/pull/540) [`0d04f95`](https://github.com/NomicFoundation/slang/commit/0d04f959bf1f5831c912d5109de3d933cfaa6266) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Rename `range` functions that return a TextRange to `text_range`

### Patch Changes

-   [#543](https://github.com/NomicFoundation/slang/pull/543) [`7a34599`](https://github.com/NomicFoundation/slang/commit/7a34599f6b237b03a0f8ba92755cae6107589e37) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - Move `syntax::parser::ProductionKind` to `syntax::nodes` namespace.

-   [#540](https://github.com/NomicFoundation/slang/pull/540) [`0d04f95`](https://github.com/NomicFoundation/slang/commit/0d04f959bf1f5831c912d5109de3d933cfaa6266) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Add TokenNode.text to the TS API.

-   [#540](https://github.com/NomicFoundation/slang/pull/540) [`0d04f95`](https://github.com/NomicFoundation/slang/commit/0d04f959bf1f5831c912d5109de3d933cfaa6266) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Add first pass of Typescript binding to the Cursor API, but no TS Visitor yet.

-   [#545](https://github.com/NomicFoundation/slang/pull/545) [`e73658a`](https://github.com/NomicFoundation/slang/commit/e73658ae4e777e78a01e213f213e2a5dc13e5cba) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - render EBNF grammar on top of each `ProductionKind`, `RuleKind`, and `TokenKind`.

-   [#558](https://github.com/NomicFoundation/slang/pull/558) [`95bbc50`](https://github.com/NomicFoundation/slang/commit/95bbc5025fbf63b8d4e07f7652a70a7f66363db6) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - Correct versioning for `SourceUnitMember` and `ContractMember` children.

## 0.8.0

### Minor Changes

-   [#513](https://github.com/NomicFoundation/slang/pull/513) [`7e01250`](https://github.com/NomicFoundation/slang/commit/7e012501c04e639b54cd150e3736683ee2c2606f) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Typescript API now has TextIndex and TextRange types that are returned from the appropriate methods rather than tuples.

### Patch Changes

-   [#527](https://github.com/NomicFoundation/slang/pull/527) [`7ccca87`](https://github.com/NomicFoundation/slang/commit/7ccca87beaa9cb96ad294d1af8a02f115481b71a) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Fix pratt parser behavior in the face of error correction
-   [#531](https://github.com/NomicFoundation/slang/pull/531) [`e3450be4`](https://github.com/NomicFoundation/slang/commit/e3450be4722845bcfce7a9ec3b3046ba6eb6961d) Thanks [@alcuadrado](https://github.com/alcuadrado)! - Make ESM named imports work in Node.js.

## 0.7.0

### Minor Changes

-   [#502](https://github.com/NomicFoundation/slang/pull/502) [`c383238`](https://github.com/NomicFoundation/slang/commit/c383238c1f51157b37ec63bc99e63fb85c1bc224) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Added error recovery i.e. a CST is _always_ produced, even if there are errors. The erroneous/skipped text is in the CST as a `TokenKind::SKIPPED` token.

-   [#501](https://github.com/NomicFoundation/slang/pull/501) [`cb221fe`](https://github.com/NomicFoundation/slang/commit/cb221fed784e8a2eb59f17907412149c7b415ed8) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - generate typescript string enums for CST kinds

-   [#517](https://github.com/NomicFoundation/slang/pull/517) [`8bd5446`](https://github.com/NomicFoundation/slang/commit/8bd544695a6dd4880a00d0cecf8d13ad79b238d3) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - extract inlined and sub-expressions in language grammar

-   [#518](https://github.com/NomicFoundation/slang/pull/518) [`b3b562b`](https://github.com/NomicFoundation/slang/commit/b3b562be6365fab25b97e54746a7500b9e7bd595) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - fill in missing CST node names

-   [#515](https://github.com/NomicFoundation/slang/pull/515) [`f24e873`](https://github.com/NomicFoundation/slang/commit/f24e873a93cbcef53aad1fa5eed1ea9ab1af1c04) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - switch over the NPM package to use CommonJS modules instead of ES modules.

-   [#498](https://github.com/NomicFoundation/slang/pull/498) [`44f1ff7`](https://github.com/NomicFoundation/slang/commit/44f1ff70100d6e2f8afe54c7ff87e24a8479e4b9) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - flatten unnamed CST nodes into parent nodes

-   [#502](https://github.com/NomicFoundation/slang/pull/502) [`c383238`](https://github.com/NomicFoundation/slang/commit/c383238c1f51157b37ec63bc99e63fb85c1bc224) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Use the Rowan model for the CST i.e. TokenNodes contain the string content, and RuleNodes contain only the combined _length_ of their children's text.

-   [#499](https://github.com/NomicFoundation/slang/pull/499) [`1582d60`](https://github.com/NomicFoundation/slang/commit/1582d60c7ef81a785db0b9e3cb4d074d9cb6d442) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - preserve correct ranges on empty rule nodes

-   [#500](https://github.com/NomicFoundation/slang/pull/500) [`73ddac9`](https://github.com/NomicFoundation/slang/commit/73ddac9ca972f80aa9a0321de7f94c47b505d7a6) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - inlining CST nodes that offer no additional syntactic information

-   [#512](https://github.com/NomicFoundation/slang/pull/512) [`72dc3d3`](https://github.com/NomicFoundation/slang/commit/72dc3d3d90bc6a02d36836cc1fed17f5be5de2fb) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Expression productions now correctly wrap the recursive 'calls' in a rule node

## 0.6.0

### Minor Changes

-   [#490](https://github.com/NomicFoundation/slang/pull/490) [`ea8e7e7`](https://github.com/NomicFoundation/slang/commit/ea8e7e771fef7fd9195bcc3004b08fc132c8990d) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - provide API to list supported language versions

-   [#489](https://github.com/NomicFoundation/slang/pull/489) [`15c34a7`](https://github.com/NomicFoundation/slang/commit/15c34a7bb0268bf26eaa6535dd637f73349596c8) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - replace panics with JS exceptions in npm package

### Patch Changes

-   [#488](https://github.com/NomicFoundation/slang/pull/488) [`d7f171c`](https://github.com/NomicFoundation/slang/commit/d7f171cf1e2da375a7ededd034a62fc6b723d44d) Thanks [@DaniPopes](https://github.com/DaniPopes)! - introduce a `cli` Cargo feature to compile the CLI binary

## 0.5.0

### Minor Changes

-   [#475](https://github.com/NomicFoundation/slang/pull/475) [`0cdfe86`](https://github.com/NomicFoundation/slang/commit/0cdfe86037bfe2f1f8be66a69e8e7d7bdbf06364) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - match TypeScript and Rust API namespaces

-   [#477](https://github.com/NomicFoundation/slang/pull/477) [`13c85a2`](https://github.com/NomicFoundation/slang/commit/13c85a2a3e4e97894d9f24a3e2886a08ffe6e569) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - move expression operators into separate nodes

-   [#481](https://github.com/NomicFoundation/slang/pull/481) [`0269f2b`](https://github.com/NomicFoundation/slang/commit/0269f2b9de3f6711055119e1f40c3f036fe3a81f) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - fix grammar versions of individual keywords

-   [#473](https://github.com/NomicFoundation/slang/pull/473) [`11d8cb0`](https://github.com/NomicFoundation/slang/commit/11d8cb0658e01f16b7afd808f31d1da88e967679) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - upgrade to rust 1.69.0

## 0.4.0

### Minor Changes

-   [#458](https://github.com/NomicFoundation/slang/pull/458) [`c0fc7e9`](https://github.com/NomicFoundation/slang/commit/c0fc7e95b87eb1ddca4f9e0003136fcbe74f5798) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Record both character and byte offsets for input positions

-   [#463](https://github.com/NomicFoundation/slang/pull/463) [`0958d6b`](https://github.com/NomicFoundation/slang/commit/0958d6baadba3295df9307e421ddd0a41ef3aaa0) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - use `number` and getters in npm public API

## 0.3.0

### Minor Changes

-   [#457](https://github.com/NomicFoundation/slang/pull/457) [`b7aae2a`](https://github.com/NomicFoundation/slang/commit/b7aae2ad891f940ee764365ac12c75fd1cb47687) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - minor grammar fixes

-   [#453](https://github.com/NomicFoundation/slang/pull/453) [`0f2f9ab`](https://github.com/NomicFoundation/slang/commit/0f2f9abec3f2525640d25bf1f288b769917fbc61) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - move Rust's `syntax::Parser::Language` API to root module

-   [#454](https://github.com/NomicFoundation/slang/pull/454) [`85dec01`](https://github.com/NomicFoundation/slang/commit/85dec0196eafa337065233de03c30d36211b03cf) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - moving to Rust version 1.65.0

-   [#456](https://github.com/NomicFoundation/slang/pull/456) [`c6d1041`](https://github.com/NomicFoundation/slang/commit/c6d10417da440f15e1c074b7d8b5d13d38e95519) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - expose `ParseError` API

-   [#451](https://github.com/NomicFoundation/slang/pull/451) [`78f633c`](https://github.com/NomicFoundation/slang/commit/78f633cb5977d358b4bcc468151359a4301089b2) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - rename `VisitorExitResponse::StepIn` to `VisitorExitResponse::Continue`

## 0.2.1

### Patch Changes

-   [#444](https://github.com/NomicFoundation/slang/pull/444) [`a858e2c`](https://github.com/NomicFoundation/slang/commit/a858e2c842db3b2183821fb92ed26af6b433e823) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - Fix HexLiteral cannot have NumberUnit

## 0.2.0

### Minor Changes

-   [#435](https://github.com/NomicFoundation/slang/pull/435) [`2a5b193`](https://github.com/NomicFoundation/slang/commit/2a5b1930b20024359fbaf06b6e9748585d7423ff) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - support user defined operators

### Patch Changes

-   [#416](https://github.com/NomicFoundation/slang/pull/416) [`fb977a5`](https://github.com/NomicFoundation/slang/commit/fb977a52b152a1ce8d8ce92db4bb00fcfb5881c1) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - fix primary expressions parser order

-   [#434](https://github.com/NomicFoundation/slang/pull/434) [`beb3708`](https://github.com/NomicFoundation/slang/commit/beb3708218ec797614ba283a13f1854d5f3c7239) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - fix UnicodeStringLiteral versioning

-   [#430](https://github.com/NomicFoundation/slang/pull/430) [`8b7492e`](https://github.com/NomicFoundation/slang/commit/8b7492e65ec7261176e444dca2563a82603b43b0) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - update READMEs with links to packages and user guides.

-   [#425](https://github.com/NomicFoundation/slang/pull/425) [`9b49b3d`](https://github.com/NomicFoundation/slang/commit/9b49b3d827536e707d78a6bc349fc82698237b75) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - add user guides to rust crate and npm packages.

-   [#432](https://github.com/NomicFoundation/slang/pull/432) [`1d1a8bb`](https://github.com/NomicFoundation/slang/commit/1d1a8bb5503c510a470bb99a18632c3e51a587ec) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - fix FunctionCallOptions versioning

-   [#427](https://github.com/NomicFoundation/slang/pull/427) [`1103916`](https://github.com/NomicFoundation/slang/commit/11039163ac3a3b66a74fa85683bde1c380a519f4) Thanks [@AntonyBlakey](https://github.com/AntonyBlakey)! - fix VariableDeclarationStatement versioning

## 0.1.1

### Patch Changes

-   [#412](https://github.com/NomicFoundation/slang/pull/412) [`9cac1a04`](https://github.com/NomicFoundation/slang/commit/9cac1a04670fa870c15cee1bd20e0e78c1d213db) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - publish npm packages

## 0.1.0

### Minor Changes

-   [#396](https://github.com/NomicFoundation/slang/pull/396) [`621b338`](https://github.com/NomicFoundation/slang/commit/621b33838c74415c46ab157205068008e05c5b9b) Thanks [@OmarTawfik](https://github.com/OmarTawfik)! - Initial release.
