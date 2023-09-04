contract Example {
  function batchTransferERC721(address _to, IERC721 _collection, uint256[] calldata _ids) public isOwner {
    // snip
  }
}
