// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public returns (uint256) {
        // Two low-level catch clauses: the second one must be flagged.
        try this.f() returns (uint256) {
        } catch {
        } catch (bytes memory data) {
            data;
        }
    }

    function g() public returns (uint256) {
        // A single low-level catch clause alongside an `Error` clause is valid
        // and must not be flagged.
        try this.g() returns (uint256) {
        } catch Error(string memory a) {
            a;
        } catch (bytes memory data) {
            data;
        }
    }
}
