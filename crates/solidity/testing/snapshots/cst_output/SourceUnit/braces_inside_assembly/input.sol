// File: @openzeppelin/contracts/token/ERC20/IERC20.sol

// OpenZeppelin Contracts (last updated v5.1.0) (token/ERC20/IERC20.sol)

pragma solidity ^0.8.20;

library SafeERC20 {
  function _callOptionalReturn() private {
    assembly {
      if iszero(success) {
        revert(ptr)
      }
    }
  }
}
