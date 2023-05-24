# changelog

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
