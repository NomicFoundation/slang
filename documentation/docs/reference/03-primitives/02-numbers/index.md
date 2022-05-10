# Numbers

<!--
cSpell:ignore ufixed
-->

## Integers

Signed (`int8`..`int256`) and unsigned (`uint8`..`uint256`) integers of various sizes, from 8 to 256 bits,
moving up in steps of 8 bits. `uint` and `int` are aliases for `uint256` and `int256`, respectively.

Integers in Solidity are restricted to a certain range. For example, with `uint32`, this is `0` up to `2**32 - 1`.

Integer literals are formed from a sequence of digits in the range `0-9`. They are interpreted as decimals.
For example, `69` means sixty nine.

!!! note "Octal literals do not exist in Solidity and leading zeros are invalid."

## Decimals

Decimal fractional literals are formed by a `.` with at least one number on one side. Examples include `1.`, `.1` and `1.3`.

## Fixed Point Numbers

Signed `fixed` and unsigned fixed `ufixed` point number of various sizes. Keywords `ufixedMxN` and `fixedMxN`, where `M` represents the number of bits taken by the type and `N` represents how many decimal points are available. `M` must be divisible by 8 and goes from 8 to 256 bits. `N` must be between 0 and 80, inclusive. `ufixed` and `fixed` are aliases for `ufixed128x18` and `fixed128x18`, respectively.

!!! warning "Fixed point numbers are not fully supported by Solidity yet. They can be declared, but cannot be assigned to or from."

## Scientific Notation

Scientific notation in the form of `2e10` is also supported, where the mantissa can be fractional but the exponent has to be an integer.
The literal `MeE` is equivalent to `M * 10**E`. Examples include `2e10`, `-2e10`, `2e-10`, `2.5e1`.

## Using Underscores

Underscores can be used to separate the digits of a numeric literal to aid readability.
For example, decimal `123_000`, hexadecimal `0x2eff_abcde`, scientific decimal notation `1_2e345_678` are all valid.
Underscores are only allowed between two digits and only one consecutive underscore is allowed.
There is no additional semantic meaning added to a number literal containing underscores, the underscores are ignored.

## Ether Units

A literal number can take a suffix of `wei`, `gwei` or `ether` to specify a sub-denomination of Ether, where Ether numbers without a postfix are assumed to be Wei.

```solidity
assert(1 wei == 1);
assert(1 gwei == 1e9);
assert(1 szabo == 1e12);
assert(1 finney == 1e15);
assert(1 ether == 1e18);
```

!!! danger "Breaking Changes"

    Units `szabo` and `finney` have been removed in `v0.7.0`.
    --8<-- "reference/03-primitives/02-numbers/tests/units/finney/output/combined"
    ---
    --8<-- "reference/03-primitives/02-numbers/tests/units/szabo/output/combined"

## Time Units

Suffixes that can be used to specify units of time where seconds are the base unit and units are considered naively in the following way:

```solidity
assert(1 == 1 seconds);
assert(1 minutes == 60 seconds);
assert(1 hours == 60 minutes);
assert(1 days == 24 hours);
assert(1 weeks == 7 days);
```

!!! danger "Breaking Changes"

    Unit `years` have been deprecated in `v0.4.24` removed in `v0.5.0`.
    --8<-- "reference/03-primitives/02-numbers/tests/units/years/output/combined"
