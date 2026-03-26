interface IERC1155Receiver {
    function onERC1155Received(
                               address operator,
                               address from,
                               uint256 id,
                               uint256 value,
                               bytes calldata data
    ) external returns (bytes4);
}

abstract contract ERC1155Holder is IERC1155Receiver {
    function onERC1155Received(
                               address,
                               address,
                               uint256,
                               uint256,
                               bytes memory
    ) public virtual override returns (bytes4) {
        return this.onERC1155Received.selector;
    }
}
