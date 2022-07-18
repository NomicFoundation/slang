<!-- markdownlint-configure-file { "first-line-heading": { "level": 2 } } -->

<!--
cSpell:ignore abicoder
cSpell:ignore structs
-->

## Version Pragma

This line declares which Solidity language version it was written for. This is to ensure that the contract is not compilable
with a new (breaking) compiler version, where it could behave differently. An error is produced if the running compiler version does not match these requirements.
Note that multiple version pragmas are supported, and the compiler

For example, this line specifies that the source code is written for Solidity version `0.4.16`, or a newer version of
the language up to, but not including version `0.9.0`:

```solidity
pragma solidity >=0.4.16 <0.9.0;
```

## ABI Coder Pragma

Used to instruct the compiler to choose a specific ABI encoder/decoder. The new ABI coder (v2) is able to encode and decode arbitrarily nested arrays and structs. It might produce less optimal code and has not received as much testing as the old encoder.

```solidity
pragma abicoder v1;
// OR
pragma abicoder v2;
```

!!! danger "Breaking Changes"

    Supported in `v0.7.5`:
    --8<-- "specification/notes/01-file-structure/03-pragmas/tests/abicoder/v1/generated/combined"
    ---
    --8<-- "specification/notes/01-file-structure/03-pragmas/tests/abicoder/v2/generated/combined"

## Experimental Pragma

It can be used to enable features of the compiler or language that are not yet enabled by default.
Compilers should produce an error on unrecognized pragmas (or earlier versions before they were released), and a warning before the stable version.
After the stable version, this should not have an effect.

### ABIEncoderV2

Please see the `abicoder` pragma defined above.

```solidity
pragma experimental ABIEncoderV2;
```

!!! danger "Breaking Changes"

    Experimental in `v0.4.16`, and enabled by default in `v0.6.0`:
    --8<-- "specification/notes/01-file-structure/03-pragmas/tests/experimental/abi-encoder-v2/generated/combined"

### SMTChecker

If you use `SMTChecker`, then you get additional safety warnings which are obtained by querying an
SMT solver. The component does not yet support all features of the Solidity language and
likely outputs many warnings. In case it reports unsupported features, the
analysis may not be fully sound.

```solidity
pragma experimental SMTChecker;
```

!!! danger "Breaking Changes"

    Experimental in `v0.4.16`, enabled by default in `v0.4.21`, and deprecated in `v0.8.4`:
    --8<-- "specification/notes/01-file-structure/03-pragmas/tests/experimental/smt-checker/generated/combined"

--8<-- "snippets/under-construction.md"
