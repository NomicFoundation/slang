// ---- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./other.sol" as M;

// f compiles the file level constant's value in, so A's deployed code
// embeds A's own creation code.

contract A {
    function f() public pure returns (bytes memory) {
        return M.CODE;
    }
}

// ---- path: other.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./main.sol";

bytes constant CODE = type(A).creationCode;
