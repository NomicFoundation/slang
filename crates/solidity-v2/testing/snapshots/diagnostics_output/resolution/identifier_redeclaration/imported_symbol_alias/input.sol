// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {Foo as Bar} from "./other.sol";

contract Bar {}

// --- path: other.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

contract Foo {}
