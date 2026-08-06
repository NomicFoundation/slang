// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

// 'lib.sol' re-exports 'Deep' under the name 'Alias', so importing 'Alias'
// resolves, but importing 'Deep' does not: only the alias is in scope there.
import {Alias} from "./lib.sol";
import {Deep} from "./lib.sol";

contract Test is Alias {}

// --- path: lib.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {Deep as Alias} from "./deep.sol";

// --- path: deep.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

contract Deep {}
