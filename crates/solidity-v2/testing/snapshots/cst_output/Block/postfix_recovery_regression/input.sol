contract C {
    function f()
// >>> Copied from crates/solidity/testing/snapshots/cst_output/Block/postfix_recovery_regression/input.sol
// Make sure we don't panic when encountering an unwinding close brace in a precedence parser

{
    a.b('
        }');
}

// <<<
}
