// https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/parsing/inline_array_empty_cells_check_lvalue.sol

contract c {
    uint[] a;
    function f() returns (uint) {
        a = [,2,3];
        return (a[0]);
    }
}
