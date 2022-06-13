// SPDX-License-Identifier: MIT
// BEE Team
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";

contract BEE is ERC20 {
  using SafeERC20 for IERC20;
  using Address for address;
  using SafeMath for uint256;

  address public governance;
  mapping(address => bool) public minters;

  constructor() public ERC20("BEE Team", "BEE") {
    governance = msg.sender;
    _mint(msg.sender, 50000000 * 10**decimals());
  }

  function mint(address account, uint256 amount) public {
    require(minters[msg.sender], "!minter");
    _mint(account, amount);
  }

  function setGovernance(address _governance) public {
    require(msg.sender == governance, "!governance");
    governance = _governance;
  }

  function addMinter(address _minter) public {
    require(msg.sender == governance, "!governance");
    minters[_minter] = true;
  }

  function removeMinter(address _minter) public {
    require(msg.sender == governance, "!governance");
    minters[_minter] = false;
  }
}
