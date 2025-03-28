// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.7.6;
pragma abicoder v2;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";

import "./IBond.sol";

contract SeniorBond is IBond, ERC721 {

    address public override smartYield;

    constructor(
        address smartYield_,
        string memory name_,
        string memory symbol_
    ) ERC721(name_, symbol_) {
        smartYield = smartYield_;
    }

    function mint(address to_, uint256 tokenId_) public override {
        require(msg.sender == smartYield, "SB: mint not smartYield");
        _mint(to_, tokenId_);
    }

    function burn(uint256 tokenId_) public override {
        require(msg.sender == smartYield, "SB: burn not smartYield");
        _burn(tokenId_);
    }
}
