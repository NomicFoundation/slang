// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint16 x;
    uint16 public y;
    uint256 public z;

    // A Yul function `f` may shadow the Solidity function `f`, and still access
    // state variables by suffix from inside its body (state variables, unlike
    // local variables, are reachable from a Yul function). Mirrors ethereum/solidity:
    // test/libsolidity/semanticTests/inlineAssembly/inline_assembly_storage_access_inside_function.sol
    function f() public returns (bool) {
        uint256 off2;
        assembly {
            function f() -> o1 {
                sstore(z.slot, 7)
                o1 := y.offset
            }
            off2 := f()
        }
        assert(off2 == 2);
        return true;
    }
}
