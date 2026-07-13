// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {
    // An `external` function is the only allowed form.
    function ok() external;

    // Explicit non-external visibilities are rejected.
    function withPublic() public;
    function withInternal() internal;
    function withPrivate() private;

    // No visibility defaults to non-external, so this is also rejected
    // (alongside the missing-visibility diagnostic).
    function withDefault();
}

contract C {
    // A non-external function in a contract is fine.
    function ok() public {}
}
