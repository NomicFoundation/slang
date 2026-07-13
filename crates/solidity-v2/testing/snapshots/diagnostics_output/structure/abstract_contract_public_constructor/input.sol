// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract Bad {
    // An abstract contract cannot have a public constructor.
    constructor() public {}
}

abstract contract OkInternal {
    // An internal constructor in an abstract contract is allowed.
    constructor() internal {}
}

abstract contract OkDefault {
    // A constructor with no visibility is always allowed.
    constructor() {}
}

contract PublicNonAbstract {
    // A public constructor in a non-abstract contract is fine.
    constructor() public {}
}
