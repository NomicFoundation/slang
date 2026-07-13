// SPDX-License-Identifier: MIT
pragma solidity *;

contract Bad {
    // A non-abstract contract cannot have an internal constructor.
    constructor() internal {}
}

abstract contract OkInternal {
    // An internal constructor in an abstract contract is allowed.
    constructor() internal {}
}

contract OkPublic {
    // A public constructor in a non-abstract contract is fine.
    constructor() public {}
}

contract OkDefault {
    // A constructor with no visibility is always allowed.
    constructor() {}
}
