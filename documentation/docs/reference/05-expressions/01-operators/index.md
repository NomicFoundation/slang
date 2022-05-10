# Operators

## Arithmetic Operators

By default, all arithmetic operations are checked for underflow or overflow, which mean that if the result of an operation falls outside the value range of the type, the call is reverted through a failing assertion. This can be disabled using the `unchecked` block, resulting in wrapping arithmetic.

- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`
- Modulo: `%`
- Exponentiation: `**`
- Unary Negation: `-`
- Increment (prefix and postfix): `++`
- Decrement (prefix and postfix): `--`

## Logical Operators

Logical operators apply the common short-circuiting rules. This means that in the expression `f(x) || g(y)`, if `f(x)` evaluates to `true`, `g(y)` will not be evaluated even if it may have side-effects.

- Negation (not): `!`
- Conjunction (and): `&&`
- Disjunction (or): `||`

## Comparison Operators

- Equality: `==`
- Inequality: `!=`
- Less than: `<`
- Greater than: `>`
- Less than or equal: `<=`
- Greater than or equal: `>=`

## Bitwise Operators

Bit operations are performed on the two's complement representation of the number. This means that, for example `~int256(0) == int256(-1)`.

- Bitwise And: `&`
- Bitwise Or: `|`
- Bitwise Xor: `^`
- Bitwise Negation: `~`

## Shift Operators

The result of a shift operation has the type of the left operand, truncating the result to match the type.
The right operand must be of unsigned type, trying to shift by a signed type will produce a compilation error.
Overflow checks are never performed for shift operations as they are done for arithmetic operations. Instead, the result is always truncated.

- Shift left: `<<`
- Shift right: `>>`

!!! danger "Breaking Changes"

    Before version `v0.5.0` a right shift `x >> y` for negative `x` was equivalent to the mathematical expression `x / 2**y` rounded towards zero.
    Right shifts used rounding up (towards zero), instead of rounding down (towards negative infinity).

### Compound Operators

If `a` is an LValue (i.e. a variable or something that can be assigned to), the following operators are available as shorthands:

- Compound Addition: `+=`
- Compound Subtraction: `-=`
- Compound Multiplication: `*=`
- Compound Division: `/=`
- Compound Modulo: `%=`
- Compound Bitwise Or: `|=`
- Compound Bitwise And: `&=`
- Compound Bitwise Xor: `^=`
- Compound Shift Left: `<<=`
- Compound Shift Right: `>>=`
