// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public returns (uint256) {
        // Named catch clauses whose selector is neither `Error` nor `Panic`
        // are invalid and must both be flagged.
        try this.f() returns (uint256) {
        } catch Error2(string memory a) {
            a;
        } catch abc() {
        }
    }

    function g() public returns (uint256) {
        // `Error` and low-level clauses are valid selectors at every supported
        // version and must not be flagged.
        try this.g() returns (uint256) {
        } catch Error(string memory a) {
            a;
        } catch (bytes memory data) {
            data;
        }
    }
}
