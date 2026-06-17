// ---- path: a.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {B} from "./b.sol";

contract A is B {}

// ---- path: b.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {C} from "./c.sol";

contract B is C {}

// ---- path: c.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {A} from "./a.sol";

contract C is A {}
