// SPDX-License-Identifier: MIT

pragma solidity ^0.8.11;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

import "@openzeppelin/contracts/utils/Counters.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";

import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract DoodledBears is ERC721, ReentrancyGuard, Ownable {
  using Counters for Counters.Counter;

  constructor(string memory _name, string memory _symbol, string memory baseURI_) ERC721(_name, _symbol) {
    baseURI = baseURI_;
  }

  /**
   * Minting functionality
   */
  uint256 public constant MAX_PER_TXN = 20;
  uint256 public constant SALE_PRICE = 0.03 ether;
  uint256 public mintableSupply = 5555;
  uint256 public presaleSupply = 3000;
  Counters.Counter private supplyCounter;

  modifier withinQuantityLimits(uint256 _quantity) {
    require(_quantity > 0 && _quantity <= MAX_PER_TXN, "Invalid quantity");
    _;
  }

  modifier withinMaximumSupply(uint256 _quantity) {
    require(totalSupply() + _quantity <= mintableSupply, "Surpasses supply");
    _;
  }

  modifier withinPresaleSupply(uint256 _quantity) {
    require(totalSupply() + _quantity <= presaleSupply, "Surpasses presale supply");
    _;
  }

  modifier hasCorrectAmount(uint256 _wei, uint256 _quantity) {
    require(_wei >= SALE_PRICE * _quantity, "Insufficent funds");
    _;
  }

  function mintWhitelist(
    uint256 _quantity,
    bytes32[] calldata merkleProof
  )
    public
    payable
    whitelistSaleActive
    hasValidMerkleProof(merkleProof, whitelistMerkleRoot)
    hasCorrectAmount(msg.value, _quantity)
    withinPresaleSupply(_quantity)
  {
    _mintNft(_msgSender(), _quantity);
  }

  function mintPublic(uint256 _quantity) public payable publicSaleActive hasCorrectAmount(msg.value, _quantity) {
    _mintNft(_msgSender(), _quantity);
  }

  function _mintNft(
    address recipient,
    uint256 _quantity
  ) private withinQuantityLimits(_quantity) withinMaximumSupply(_quantity) {
    for (uint256 i = 0; i < _quantity; i++) {
      supplyCounter.increment();
      _mint(recipient, totalSupply());
    }
  }

  function totalSupply() public view returns (uint256) {
    return supplyCounter.current();
  }

  function decreaseMintableSupply(uint256 _total) public onlyOwner {
    require(_total < mintableSupply, "Can only decrease supply");
    require(_total >= totalSupply(), "Must be above total minted");
    mintableSupply = _total;
  }

  /**
   * Gift a mint
   */
  function mintAdmin(address recipient, uint256 _quantity) public payable onlyOwner {
    _mintNft(recipient, _quantity);
  }

  /**
   * Public and Whitelist Sale Toggle
   */
  bool public publicSale = false;
  bool public whitelistSale = false;

  modifier publicSaleActive() {
    require(publicSale, "Public sale has not started");
    _;
  }

  function setPublicSale(bool toggle) external onlyOwner {
    publicSale = toggle;
  }

  modifier whitelistSaleActive() {
    require(whitelistSale, "Whitelist has not started");
    require(!publicSale, "Public sale has started");
    _;
  }

  function setWhitelistSale(bool toggle) external onlyOwner {
    whitelistSale = toggle;
  }

  /**
   * Whitelist merkle root
   */
  bytes32 public whitelistMerkleRoot;

  modifier hasValidMerkleProof(bytes32[] calldata merkleProof, bytes32 root) {
    require(MerkleProof.verify(merkleProof, root, keccak256(abi.encodePacked(msg.sender))), "Address not whitelisted");
    _;
  }

  function setWhitelistMerkleRoot(bytes32 merkleRoot) external onlyOwner {
    whitelistMerkleRoot = merkleRoot;
  }

  /**
   * Base URI
   */
  string private baseURI;

  function setBaseURI(string memory baseURI_) external onlyOwner {
    baseURI = baseURI_;
  }

  function _baseURI() internal view virtual override returns (string memory) {
    return baseURI;
  }

  /**
   * Withdrawal
   */
  address private constant payoutAddress1 = 0xbaF153A8AfF8352cB6539CF9168255442Def0a02;
  address private constant payoutAddress2 = 0x79Bec191C629FfaE6e2fd5872f7D0033b10A5385;
  address private constant payoutAddress3 = 0x8Ba631c37CE91A2D303be09907F496220a153d6a;
  address private constant payoutAddress4 = 0x03D50521BFaB9D6442e73c1BF47e1795bAb52Df3;

  function withdraw() public nonReentrant {
    uint256 balance = address(this).balance;
    Address.sendValue(payable(payoutAddress1), (balance * 20) / 100);
    Address.sendValue(payable(payoutAddress2), (balance * 20) / 100);
    Address.sendValue(payable(payoutAddress3), (balance * 40) / 100);
    Address.sendValue(payable(payoutAddress4), (balance * 20) / 100);
  }
}
