// --- path: a.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

// Each file imports the symbol from the other, and neither declares it, so
// the alias chain closes on itself without ever reaching a declaration.
import {foo} from "./b.sol";

// --- path: b.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {foo} from "./a.sol";
