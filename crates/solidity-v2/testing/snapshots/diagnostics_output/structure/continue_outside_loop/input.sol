// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Invalid: `continue` at the function-body level, outside any loop.
    function directlyInBody() public pure {
        continue;
    }

    // Invalid: `continue` inside an `if` is still outside any loop.
    function insideIf(bool flag) public pure {
        if (flag) {
            continue;
        }
    }

    // Valid: `continue` inside each kind of loop must not be flagged.
    function insideLoops(uint256 n) public pure {
        for (uint256 i = 0; i < n; i++) {
            continue;
        }

        while (n > 0) {
            continue;
        }

        do {
            continue;
        } while (n > 0);
    }

    // Valid: a nested loop still provides an enclosing loop for `continue`.
    function nestedLoops(uint256 n) public pure {
        for (uint256 i = 0; i < n; i++) {
            while (n > 0) {
                continue;
            }
        }
    }
}
