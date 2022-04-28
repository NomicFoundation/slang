# Pragmas

<!--
cSpell:ignore abicoder
cSpell:ignore structs
-->

## Version Pragma

This line declares which Solidity language version it was written for. This is to ensure that the contract is not compilable with a new (breaking) compiler version, where it could behave differently. An error is produced if the running compiler version does not match these requirements.

For example, this line specifies that the source code is written for Solidity version `0.4.16`, or a newer version of
the language up to, but not including version `0.9.0`:

```solidity
pragma solidity >=0.4.16 <0.9.0;
```

!!! todo

    We need to document how are versions parsed, since they are a custom subset of the [official SemVer spec](https://semver.org/){target=\_blank}.
    In `solc`, the [checker pass](https://github.com/ethereum/solidity/blob/57e012da985569f6d20b0e7ac6a195e4d0aa8131/libsolidity/analysis/SyntaxChecker.cpp#L156-L171){target=\_blank} combined with a [custom version parser](https://github.com/ethereum/solidity/blob/d7a40622e4e21b05ff0d4abbde1dbe9dd519d94e/liblangutil/SemVerHandler.cpp#L121-L136){target=\_blank} only support a subset. We should also document all supported prefixes, like `>`, `<=`, and `^`.

## ABI Coder Pragma

Used to instruct the compiler to choose a specific ABI encoder/decoder. The new ABI coder (v2) is able to encode and decode arbitrarily nested arrays and structs. It might produce less optimal code and has not received as much testing as the old encoder.

```solidity
pragma abicoder v1;
// OR
pragma abicoder v2;
```

!!! danger "Breaking Changes"

    --8<-- "tests/pragmas/abicoder/v1/output/combined"
    ---
    --8<-- "tests/pragmas/abicoder/v2/output/combined"

## Experimental Pragma

It can be used to enable features of the compiler or language that are not yet enabled by default.
Compilers should produce an error on unrecognized pragmas (or earlier versions before they were released), and a warning before the stable version.
After the stable version, this should not have an effect.

### ABIEncoderV2

Please see `pragma abicoder v2` above.

```solidity
pragma experimental ABIEncoderV2;
```

!!! danger "Breaking Changes"

    --8<-- "tests/pragmas/experimental/abi-encoder-v2/output/combined"

### SMTChecker

If you use `SMTChecker`, then you get additional safety warnings which are obtained by querying an
SMT solver. The component does not yet support all features of the Solidity language and
likely outputs many warnings. In case it reports unsupported features, the
analysis may not be fully sound.

```solidity
pragma experimental SMTChecker;
```

!!! danger "Breaking Changes"

    --8<-- "tests/pragmas/experimental/smt-checker/output/combined"
