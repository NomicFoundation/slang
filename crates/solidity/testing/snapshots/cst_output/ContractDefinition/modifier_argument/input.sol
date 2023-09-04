//SPDX-License-Identifier: MIT
contract TYREAI {
  function freeMint(uint256 _mintAmount) public mintCompliance(_mintAmount) {
    if (!isFreeMint()) revert FreeMintOver();
    if (freeWallets[msg.sender] + _mintAmount > freeMaxMintPerWallet) revert ExceedsWalletLimit();

    unchecked {
      freeWallets[msg.sender] += _mintAmount;
    }
    _safeMint(msg.sender, _mintAmount);
  }
}
