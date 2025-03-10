// SPDX-License-Identifier: MIT

pragma solidity ^0.8.9;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";
import "@openzeppelin/contracts/finance/PaymentSplitter.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "erc721a/contracts/ERC721A.sol";
import "refer2earn/Referable.sol";
import {DefaultOperatorFilterer} from "./opensea/DefaultOperatorFilterer.sol";
import "./IMintPass.sol";

contract ERC721AContract is ERC721A, Ownable, PaymentSplitter, DefaultOperatorFilterer, ReentrancyGuard, Referable {

    using Strings for uint256;

    struct Token {
        uint16 maxSupply;
        PublicMintType pubMintType;
        uint16 pubMaxMint;
        uint72 preSalePrice;
        uint72 pubSalePrice;
        bool preSaleIsActive;
        bool pubSaleIsActive;
        bool claimIsActive;
        uint8 preSalePhase;
        bool transferrable;
        bool supplyLock;
    }

    enum PublicMintType { PerWallet, PerTransaction }

    mapping(address => uint16) public hasClaimed;
    mapping(address => uint16) public hasMinted;
    mapping(address => bool) public fiatMinters;
    Token public token;
    string private baseURI;
    IMintPass public mintpass;
    string public provenance;
    bytes32 public saleMerkleRoot;
    bytes32 public claimMerkleRoot;

    constructor(
        string memory _name,
        string memory _symbol,
        string memory _uri,
        address[] memory _payees,
        uint256[] memory _shares,
        address _owner,
        address _r2eAddress,
        address[] memory _fiatMinters,
        string memory _provenance,
        Token memory _token
    ) ERC721A(_name, _symbol)
      Referable(_r2eAddress)
      PaymentSplitter(_payees, _shares) {
        provenance = _provenance;
        baseURI = _uri;
        token = _token;
        for (uint256 i; i < _fiatMinters.length; i++) fiatMinters[_fiatMinters[i]] = true;
        transferOwnership(_owner);
    }

    function setApprovalForAll(address operator, bool approved) public override onlyAllowedOperatorApproval(operator) {
        require(token.transferrable, "Soulbound");
        super.setApprovalForAll(operator, approved);
    }

    function approve(address operator, uint256 tokenId) payable public override onlyAllowedOperatorApproval(operator) {
        require(token.transferrable, "Soulbound");
        super.approve(operator, tokenId);
    }

    function transferFrom(address from, address to, uint256 tokenId) payable public override onlyAllowedOperator(from) {
        require(token.transferrable, "Soulbound");
        super.transferFrom(from, to, tokenId);
    }

    function safeTransferFrom(address from, address to, uint256 tokenId) payable public override onlyAllowedOperator(from) {
        require(token.transferrable, "Soulbound");
        super.safeTransferFrom(from, to, tokenId);
    }

    function safeTransferFrom(address from, address to, uint256 tokenId, bytes memory data)
        public
        payable
        override
        onlyAllowedOperator(from)
    {
        require(token.transferrable, "Soulbound");
        super.safeTransferFrom(from, to, tokenId, data);
    }

    function lockSupply() external onlyOwner {
        token.supplyLock = true;
    }

    function setFiatMinter(address _address, bool _allowed) external onlyOwner {
        if (_allowed) {
            fiatMinters[_address] = true;
        } else {
            delete fiatMinters[_address];
        }
    }

    function setSaleRoot(bytes32 _root) external onlyOwner {
        saleMerkleRoot = _root;
    }

    function setClaimRoot(bytes32 _root) external onlyOwner {
        claimMerkleRoot = _root;
    }

    function _startTokenId() override internal pure returns (uint256) {
        return 1;
    }

    function tokenURI(uint256 _tokenId) override public view returns (string memory) {
        return string(abi.encodePacked(baseURI, _tokenId.toString()));
    }

    function setMintPass(address _address) external onlyOwner {
        mintpass = IMintPass(_address);
    }

    function setPrice(
        uint72 _preSalePrice,
        uint72 _pubSalePrice
    ) external onlyOwner {
        token.preSalePrice = _preSalePrice;
        token.pubSalePrice = _pubSalePrice;
    }

    function updateConfig(
        uint16 _maxSupply,
        uint16 _pubMaxMint,
        PublicMintType _pubMintType
    ) external onlyOwner {
        if (token.supplyLock) require(_maxSupply == token.maxSupply, "Locked");
        require(_pubMaxMint <= 50, "Too many");
        require(_maxSupply >= totalSupply(), "Invalid supply");
        token.maxSupply = _maxSupply;
        token.pubMaxMint = _pubMaxMint;
        token.pubMintType = _pubMintType;
    }

    function setBaseTokenURI(string memory _uri) external onlyOwner {
        baseURI = _uri;
    }

    function updateSaleState(
        bool _preSaleIsActive,
        bool _pubSaleIsActive,
        bool _claimIsActive,
        uint8 _preSalePhase
    ) external onlyOwner {
        require(_preSalePhase == 0 || _preSalePhase == 1 || _preSalePhase == 2, "Bad phase");
        if (_preSaleIsActive && _preSalePhase == 1) require(address(mintpass) != address(0), "MintPass undefined");
        if (_preSaleIsActive && _preSalePhase == 2) require(saleMerkleRoot != "", "Root undefined");
        if (_claimIsActive) require(claimMerkleRoot != "", "Root undefined");
        token.preSaleIsActive = _preSaleIsActive;
        token.pubSaleIsActive = _pubSaleIsActive;
        token.claimIsActive = _claimIsActive;
        token.preSalePhase = _preSalePhase;
    }

    function isEligible(
        address _address,
        uint16 _quantity,
        uint16 _maxMint,
        bytes32[] memory _proof,
        uint256 _value
    ) internal view returns (
        bool isEligibleClaim, 
        bool isEligiblePreSale,
        bool isEligiblePubSale
    ) {
        bool _isEligibleClaim;
        bool _isEligiblePreSale;
        bool _isEligiblePubSale;
        bool _hasSupply = uint16(totalSupply()) + _quantity <= token.maxSupply;
        if(token.claimIsActive && (_quantity <= (_maxMint - hasClaimed[_address])) && _value == 0) {
                _isEligibleClaim = _hasSupply && _maxMint <= 50 &&
                    MerkleProof.verify(_proof, claimMerkleRoot, keccak256(abi.encode(_address, _maxMint)));
        }
        if(!_isEligibleClaim && token.preSaleIsActive && (_value == token.preSalePrice * _quantity)) {
            if (token.preSalePhase == 1) {
                _isEligiblePreSale = _hasSupply && (mintpass.balanceOf(_address, 1) >= _quantity);
            }
            if (token.preSalePhase == 2 && (_quantity <= _maxMint - hasMinted[_address])) {
                _isEligiblePreSale = _hasSupply && _maxMint <= 50 &&
                    MerkleProof.verify(_proof, saleMerkleRoot, keccak256(abi.encode(_address, _maxMint)));
            }
        }
        if (!_isEligibleClaim && !_isEligiblePreSale && token.pubSaleIsActive) {
            if (token.pubMintType == PublicMintType.PerWallet) {
                _isEligiblePubSale = _hasSupply && _quantity <= token.pubMaxMint && (_quantity <= (token.pubMaxMint - hasMinted[_address]));
            } else {
                _isEligiblePubSale = _hasSupply && _quantity <= token.pubMaxMint;
            }
        }
        return (_isEligibleClaim, _isEligiblePreSale, _isEligiblePubSale);
    }

    function _mintTo(
        address _address,
        uint16 _quantity,
        uint16 _maxMint,
        bytes32[] memory _proof,
        address payable _referrer,
        uint256 _value
    ) internal {
        (bool _isEligibleClaim, bool _isEligiblePreSale, bool _isEligiblePubSale) = isEligible(_address, _quantity, _maxMint, _proof, _value);
        require(_isEligibleClaim || _isEligiblePreSale || _isEligiblePubSale, "Not eligible");
        if (_isEligibleClaim) {
            require(_value == 0, "ETH incorrect");
            hasClaimed[_address] += _quantity;
        }
        if (!_isEligibleClaim && _isEligiblePreSale) {
            require(token.preSalePrice * _quantity <= _value, "ETH incorrect");
            if (token.preSalePhase == 1) {
                mintpass.burnForAddress(1, _quantity, _address);
            }
            if (token.preSalePhase == 2) {
                hasMinted[_address] += _quantity;
            }
        }
        if (!_isEligibleClaim && !_isEligiblePreSale && _isEligiblePubSale) {
            require(token.pubSalePrice * _quantity <= _value, "ETH incorrect");
            if (token.pubMintType == PublicMintType.PerWallet) hasMinted[_address] += _quantity;
        }
        _safeMint(_address, _quantity);
        Referable.payReferral(_address, _referrer, _quantity, _value);
    }

    function mint(
        address _address,
        uint256 _quantity,
        uint256 _maxMint,
        bytes32[] memory _proof,
        address payable _referrer
    ) external payable nonReentrant {
        if (_address != msg.sender) require(fiatMinters[msg.sender], "Unauthorized");
        _mintTo(_address, uint16(_quantity), uint16(_maxMint), _proof, _referrer, msg.value);
    }

    function burn(uint256[] memory _ids) external {
        for (uint16 i; i < _ids.length; i++) {
            require(ownerOf(_ids[i]) == msg.sender, "Unauthorized");
            _burn(_ids[i]);
        }
    }

    function airdrop(address[] memory _addresses, uint16[] memory _quantities) external onlyOwner {
        require(_addresses.length > 0, "Invalid");
        require(_addresses.length == _quantities.length, "Invalid");
        uint16 _quantity;
        for (uint256 i; i < _quantities.length; i++) {
            require(_quantities[i] <= 50, "Too many");
            _quantity += _quantities[i];
        }
        require(totalSupply() + _quantity <= token.maxSupply, "No supply");
        for (uint256 i; i < _addresses.length; i++) _safeMint(_addresses[i], _quantities[i]);
    }
}