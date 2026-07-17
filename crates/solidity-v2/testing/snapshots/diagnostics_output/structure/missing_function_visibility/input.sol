// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A contract function must specify a visibility (suggests `public`).
    function noVis() {}

    // A function with an explicit visibility is allowed.
    function withVis() public {}

    // Constructors are exempt from the visibility requirement.
    constructor() {}
}

library L {
    // A library function must specify a visibility (suggests `public`).
    function noVis() {}
}

interface I {
    // An interface function without visibility suggests `external`.
    function noVis();

    // An explicit `external` visibility is allowed.
    function withVis() external;
}

// A free function without visibility is allowed.
function free() {}
