// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public returns (uint256) {
        // Two `Panic` catch clauses: the second one must be flagged.
        try this.f() returns (uint256) {
        } catch Panic(uint256 a) {
            a;
        } catch Panic(uint256 b) {
            b;
        }
    }

    function g() public returns (uint256) {
        // A single `Panic` catch clause alongside an `Error` and a low-level
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
