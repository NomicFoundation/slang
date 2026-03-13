contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/Expression/new_expression_options/input.sol
new Foo{value: 10}{gas: 100}

// <<<
;
    }
}
