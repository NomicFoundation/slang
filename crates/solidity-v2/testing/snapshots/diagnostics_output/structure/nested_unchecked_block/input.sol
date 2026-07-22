// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint x) public pure {
        // A directly nested `unchecked` block is not allowed.
        unchecked {
            unchecked {
                uint a = 2 + 3;
            }
        }

        // Nesting through an intermediate regular block is also not allowed.
        unchecked {
            {
                unchecked {
                    uint b = 2 + 3;
                }
            }
        }

        // Nesting through a control-flow block is also not allowed.
        unchecked {
            if (x > 0) {
                unchecked {
                    uint c = 2 + 3;
                }
            }
        }

        // Two sibling (non-nested) `unchecked` blocks are fine.
        unchecked {
            uint d = 2 + 3;
        }
        unchecked {
            uint e = 2 + 3;
        }
    }
}
