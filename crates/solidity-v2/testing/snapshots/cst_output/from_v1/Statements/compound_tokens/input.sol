contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/Statements/compound_tokens/input.sol
a && b;
a &= b;
a || b;
a |= b;
a / b;
a /= b;
a > b;
a >= b;
a >> b;
a >>= b;
a >>>= b;
a << b;
a <<= b;

// <<<
    }
}
