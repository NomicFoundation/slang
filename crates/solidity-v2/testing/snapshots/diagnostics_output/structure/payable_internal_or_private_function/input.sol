// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // `internal` and `private` functions cannot be payable.
    function internalPayable() internal payable {}
    function privatePayable() private payable {}

    // `public` and `external` payable functions are allowed.
    function publicPayable() public payable {}
    function externalPayable() external payable {}

    // Non-payable `internal`/`private` functions are fine.
    function internalPlain() internal {}
    function privatePlain() private {}

    // A payable function with no explicit visibility is only reported as a
    // missing-visibility error, not as this one.
    function noVisibilityPayable() payable {}
}
