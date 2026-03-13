contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/StringLiteral/escape_arbitrary_unicode/input.sol
// Disallowed post 0.4.25
"\✅"

// <<<
;
    }
}
