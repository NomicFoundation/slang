# changelog

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
