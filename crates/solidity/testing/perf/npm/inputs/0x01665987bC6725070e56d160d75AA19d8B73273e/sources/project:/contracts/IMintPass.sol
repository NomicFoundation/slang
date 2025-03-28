// SPDX-License-Identifier: MIT

pragma solidity ^0.8.9;

abstract contract IMintPass {
    function balanceOf(address owner, uint256 id)
        public
        view
        virtual
        returns (uint256 balance);
    function burnForAddress(
        uint256 _id,
        uint256 _quantity,
        address _address
    ) public virtual;
}