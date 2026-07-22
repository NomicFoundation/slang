// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public returns (uint256) {
        // Two `Error` catch clauses: the second one must be flagged.
        try this.f() returns (uint256) {
        } catch Error(string memory a) {
            a;
        } catch Error(string memory b) {
            b;
        }
    }

    function g() public returns (uint256) {
        // A single `Error` catch clause alongside a `Panic` and a low-level
        // catch clause is valid and must not be flagged.
        try this.g() returns (uint256) {
        } catch Error(string memory a) {
            a;
        } catch Panic(uint256 code) {
            code;
        } catch (bytes memory data) {
            data;
        }
    }
}
