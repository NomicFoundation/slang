// SPDX-License-Identifier: MIT
pragma solidity *;

contract B {
    function bg() public virtual {}
}

contract C is B {
    function missing() internal pure returns (address) {
        // Called internally, a base function has no address.
        return super.bg.address;
    }

    function present() internal pure returns (bytes4, bytes4, bytes4) {
        return (super.bg.selector, B.bg.selector, bg.selector);
    }
}
