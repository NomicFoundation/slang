// --- path: s1.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {f as g, g as h} from "./s2.sol";

function f() pure returns (uint256) {
    return h() - g();
}

// --- path: s2.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {f as h} from "./s1.sol";

function f() pure returns (uint256) {
    return 2;
}

function g() pure returns (uint256) {
    return 4;
}

// --- path: s3.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

// Neither file above is ambiguous on its own: each name they declare or import
// holds one declaration. Importing both unqualified overlays the two aliasings,
// and every one of the three names then holds two declarations a call could not
// tell apart.
import "./s1.sol";
import "./s2.sol";
