--8<-- "crates/solidity/inputs/schema/snippets/under-construction.md"

## Version Pragma

This line declares which Solidity language version it was written for. This is to ensure that the contract is not compilable
with a new (breaking) compiler version, where it could behave differently. An error is produced if the running compiler version does not match these requirements.

Note that multiple version pragmas are supported, and the compiler will verify each pragma separately.

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

## Experimental Pragma

It can be used to enable features of the compiler or language that are not yet enabled by default.
Compilers should produce an error on unrecognized pragmas (or earlier versions before they were released), and a warning before the stable version.
After the stable version, this should not have an effect.

### ABIEncoderV2

Please see the `abicoder` pragma defined above.

```solidity
pragma experimental ABIEncoderV2;
```

### SMTChecker

If you use `SMTChecker`, then you get additional safety warnings which are obtained by querying an
SMT solver. The component does not yet support all features of the Solidity language and
likely outputs many warnings. In case it reports unsupported features, the
analysis may not be fully sound.

```solidity
pragma experimental SMTChecker;
```
