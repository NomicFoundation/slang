// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Invalid: `break` at the function-body level, outside any loop.
    function directlyInBody() public pure {
        break;
    }

    // Invalid: `break` inside an `if` is still outside any loop.
    function insideIf(bool flag) public pure {
        if (flag) {
            break;
        }
    }

    // Valid: `break` inside each kind of loop must not be flagged.
    function insideLoops(uint256 n) public pure {
        for (uint256 i = 0; i < n; i++) {
            break;
        }

        while (n > 0) {
            break;
        }

        do {
            break;
        } while (n > 0);
    }

    // Valid: a nested loop still provides an enclosing loop for `break`.
    function nestedLoops(uint256 n) public pure {
        for (uint256 i = 0; i < n; i++) {
            while (n > 0) {
                break;
            }
        }
    }
}
