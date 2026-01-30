contract CreateX {

    address internal immutable _SELF = address(this);

    struct Values {
        uint256 constructorAmount;
        uint256 initCallAmount;
    }

    enum SenderBytes {
        MsgSender,
        ZeroAddress,
        Random
    }

    enum RedeployProtectionFlag {
        True,
        False,
        Unspecified
    }

    event ContractCreation(address indexed newContract, bytes32 indexed salt);

    event ContractCreation(address indexed newContract);

    event Create3ProxyContractCreation(address indexed newContract, bytes32 indexed salt);

    error FailedContractCreation(address emitter);

    error FailedContractInitialisation(address emitter, bytes revertData);

    error InvalidSalt(address emitter);

    error InvalidNonceValue(address emitter);

    error FailedEtherTransfer(address emitter, bytes revertData);

    function deployCreate(bytes memory initCode) public payable returns (address newContract) {

        _requireSuccessfulContractCreation({newContract: newContract});
        emit ContractCreation({newContract: newContract});
    }

    function deployCreateAndInit(
        bytes memory initCode,
        bytes memory data,
        Values memory values,
        address refundAddress
    ) public payable returns (address newContract) {

        _requireSuccessfulContractCreation({newContract: newContract});
        emit ContractCreation({newContract: newContract});

        (bool success, bytes memory returnData) = newContract.call{value: values.initCallAmount}(data);
        if (!success) {
            revert FailedContractInitialisation({emitter: _SELF, revertData: returnData});
        }

        if (_SELF.balance != 0) {

            (success, returnData) = refundAddress.call{value: _SELF.balance}("");
            if (!success) {
                revert FailedEtherTransfer({emitter: _SELF, revertData: returnData});
            }
        }
    }

    function deployCreateAndInit(
        bytes memory initCode,
        bytes memory data,
        Values memory values
    ) public payable returns (address newContract) {
        newContract = deployCreateAndInit({initCode: initCode, data: data, values: values, refundAddress: msg.sender});
    }

    function deployCreateClone(address implementation, bytes memory data) public payable returns (address proxy) {
        bytes20 implementationInBytes = bytes20(implementation);

        if (proxy == address(0)) {
            revert FailedContractCreation({emitter: _SELF});
        }
        emit ContractCreation({newContract: proxy});

        (bool success, bytes memory returnData) = proxy.call{value: msg.value}(data);
        _requireSuccessfulContractInitialisation({
            success: success,
            returnData: returnData,
            implementation: implementation
        });
    }

    function computeCreateAddress(address deployer, uint256 nonce) public view returns (address computedAddress) {
        bytes memory data;
        bytes1 len = bytes1(0x94);

        if (nonce > type(uint64).max - 1) {
            revert InvalidNonceValue({emitter: _SELF});
        }

        if (nonce == 0x00) {
            data = abi.encodePacked(bytes1(0xd6), len, deployer, bytes1(0x80));
        }

        else if (nonce <= 0x7f) {
            data = abi.encodePacked(bytes1(0xd6), len, deployer, uint8(nonce));
        }

        else if (nonce <= type(uint8).max) {
            data = abi.encodePacked(bytes1(0xd7), len, deployer, bytes1(0x81), uint8(nonce));
        } else if (nonce <= type(uint16).max) {
            data = abi.encodePacked(bytes1(0xd8), len, deployer, bytes1(0x82), uint16(nonce));
        } else if (nonce <= type(uint24).max) {
            data = abi.encodePacked(bytes1(0xd9), len, deployer, bytes1(0x83), uint24(nonce));
        } else if (nonce <= type(uint32).max) {
            data = abi.encodePacked(bytes1(0xda), len, deployer, bytes1(0x84), uint32(nonce));
        } else if (nonce <= type(uint40).max) {
            data = abi.encodePacked(bytes1(0xdb), len, deployer, bytes1(0x85), uint40(nonce));
        } else if (nonce <= type(uint48).max) {
            data = abi.encodePacked(bytes1(0xdc), len, deployer, bytes1(0x86), uint48(nonce));
        } else if (nonce <= type(uint56).max) {
            data = abi.encodePacked(bytes1(0xdd), len, deployer, bytes1(0x87), uint56(nonce));
        } else {
            data = abi.encodePacked(bytes1(0xde), len, deployer, bytes1(0x88), uint64(nonce));
        }

        computedAddress = address(uint160(uint256(keccak256(data))));
    }

    function computeCreateAddress(uint256 nonce) public view returns (address computedAddress) {
        computedAddress = computeCreateAddress({deployer: _SELF, nonce: nonce});
    }

    function deployCreate2(bytes32 salt, bytes memory initCode) public payable returns (address newContract) {
        bytes32 guardedSalt = _guard({salt: salt});

        _requireSuccessfulContractCreation({newContract: newContract});
        emit ContractCreation({newContract: newContract, salt: guardedSalt});
    }

    function deployCreate2(bytes memory initCode) public payable returns (address newContract) {

        newContract = deployCreate2({salt: _generateSalt(), initCode: initCode});
    }

    function deployCreate2AndInit(
        bytes32 salt,
        bytes memory initCode,
        bytes memory data,
        Values memory values,
        address refundAddress
    ) public payable returns (address newContract) {
        bytes32 guardedSalt = _guard({salt: salt});

        _requireSuccessfulContractCreation({newContract: newContract});
        emit ContractCreation({newContract: newContract, salt: guardedSalt});

        (bool success, bytes memory returnData) = newContract.call{value: values.initCallAmount}(data);
        if (!success) {
            revert FailedContractInitialisation({emitter: _SELF, revertData: returnData});
        }

        if (_SELF.balance != 0) {

            (success, returnData) = refundAddress.call{value: _SELF.balance}("");
            if (!success) {
                revert FailedEtherTransfer({emitter: _SELF, revertData: returnData});
            }
        }
    }

    function deployCreate2AndInit(
        bytes32 salt,
        bytes memory initCode,
        bytes memory data,
        Values memory values
    ) public payable returns (address newContract) {

        newContract = deployCreate2AndInit({
            salt: salt,
            initCode: initCode,
            data: data,
            values: values,
            refundAddress: msg.sender
        });
    }

    function deployCreate2AndInit(
        bytes memory initCode,
        bytes memory data,
        Values memory values,
        address refundAddress
    ) public payable returns (address newContract) {

        newContract = deployCreate2AndInit({
            salt: _generateSalt(),
            initCode: initCode,
            data: data,
            values: values,
            refundAddress: refundAddress
        });
    }

    function deployCreate2AndInit(
        bytes memory initCode,
        bytes memory data,
        Values memory values
    ) public payable returns (address newContract) {

        newContract = deployCreate2AndInit({
            salt: _generateSalt(),
            initCode: initCode,
            data: data,
            values: values,
            refundAddress: msg.sender
        });
    }

    function deployCreate2Clone(
        bytes32 salt,
        address implementation,
        bytes memory data
    ) public payable returns (address proxy) {
        bytes32 guardedSalt = _guard({salt: salt});
        bytes20 implementationInBytes = bytes20(implementation);

        if (proxy == address(0)) {
            revert FailedContractCreation({emitter: _SELF});
        }
        emit ContractCreation({newContract: proxy, salt: guardedSalt});

        (bool success, bytes memory returnData) = proxy.call{value: msg.value}(data);
        _requireSuccessfulContractInitialisation({
            success: success,
            returnData: returnData,
            implementation: implementation
        });
    }

    function deployCreate2Clone(address implementation, bytes memory data) public payable returns (address proxy) {

        proxy = deployCreate2Clone({salt: _generateSalt(), implementation: implementation, data: data});
    }

    function computeCreate2Address(
        bytes32 salt,
        bytes32 initCodeHash,
        address deployer
    ) public pure returns (address computedAddress) {

    }

    function computeCreate2Address(bytes32 salt, bytes32 initCodeHash) public view returns (address computedAddress) {
        computedAddress = computeCreate2Address({salt: salt, initCodeHash: initCodeHash, deployer: _SELF});
    }

    function deployCreate3(bytes32 salt, bytes memory initCode) public payable returns (address newContract) {
        bytes32 guardedSalt = _guard({salt: salt});
        bytes memory proxyChildBytecode = hex"67_36_3d_3d_37_36_3d_34_f0_3d_52_60_08_60_18_f3";
        address proxy;

        if (proxy == address(0)) {
            revert FailedContractCreation({emitter: _SELF});
        }
        emit Create3ProxyContractCreation({newContract: proxy, salt: guardedSalt});

        newContract = computeCreate3Address({salt: guardedSalt});
        (bool success, ) = proxy.call{value: msg.value}(initCode);
        _requireSuccessfulContractCreation({success: success, newContract: newContract});
        emit ContractCreation({newContract: newContract});
    }

    function deployCreate3(bytes memory initCode) public payable returns (address newContract) {

        newContract = deployCreate3({salt: _generateSalt(), initCode: initCode});
    }

    function deployCreate3AndInit(
        bytes32 salt,
        bytes memory initCode,
        bytes memory data,
        Values memory values,
        address refundAddress
    ) public payable returns (address newContract) {
        bytes32 guardedSalt = _guard({salt: salt});
        bytes memory proxyChildBytecode = hex"67_36_3d_3d_37_36_3d_34_f0_3d_52_60_08_60_18_f3";
        address proxy;

        if (proxy == address(0)) {
            revert FailedContractCreation({emitter: _SELF});
        }
        emit Create3ProxyContractCreation({newContract: proxy, salt: guardedSalt});

        newContract = computeCreate3Address({salt: guardedSalt});
        (bool success, ) = proxy.call{value: values.constructorAmount}(initCode);
        _requireSuccessfulContractCreation({success: success, newContract: newContract});
        emit ContractCreation({newContract: newContract});

        bytes memory returnData;
        (success, returnData) = newContract.call{value: values.initCallAmount}(data);
        if (!success) {
            revert FailedContractInitialisation({emitter: _SELF, revertData: returnData});
        }

        if (_SELF.balance != 0) {

            (success, returnData) = refundAddress.call{value: _SELF.balance}("");
            if (!success) {
                revert FailedEtherTransfer({emitter: _SELF, revertData: returnData});
            }
        }
    }

    function deployCreate3AndInit(
        bytes32 salt,
        bytes memory initCode,
        bytes memory data,
        Values memory values
    ) public payable returns (address newContract) {

        newContract = deployCreate3AndInit({
            salt: salt,
            initCode: initCode,
            data: data,
            values: values,
            refundAddress: msg.sender
        });
    }

    function deployCreate3AndInit(
        bytes memory initCode,
        bytes memory data,
        Values memory values,
        address refundAddress
    ) public payable returns (address newContract) {

        newContract = deployCreate3AndInit({
            salt: _generateSalt(),
            initCode: initCode,
            data: data,
            values: values,
            refundAddress: refundAddress
        });
    }

    function deployCreate3AndInit(
        bytes memory initCode,
        bytes memory data,
        Values memory values
    ) public payable returns (address newContract) {

        newContract = deployCreate3AndInit({
            salt: _generateSalt(),
            initCode: initCode,
            data: data,
            values: values,
            refundAddress: msg.sender
        });
    }

    function computeCreate3Address(bytes32 salt, address deployer) public pure returns (address computedAddress) {

    }

    function computeCreate3Address(bytes32 salt) public view returns (address computedAddress) {
        computedAddress = computeCreate3Address({salt: salt, deployer: _SELF});
    }

    function _guard(bytes32 salt) internal view returns (bytes32 guardedSalt) {
        (SenderBytes senderBytes, RedeployProtectionFlag redeployProtectionFlag) = _parseSalt({salt: salt});

        if (senderBytes == SenderBytes.MsgSender && redeployProtectionFlag == RedeployProtectionFlag.True) {

            guardedSalt = keccak256(abi.encode(msg.sender, block.chainid, salt));
        } else if (senderBytes == SenderBytes.MsgSender && redeployProtectionFlag == RedeployProtectionFlag.False) {

            guardedSalt = _efficientHash({a: bytes32(uint256(uint160(msg.sender))), b: salt});
        } else if (senderBytes == SenderBytes.MsgSender) {

            revert InvalidSalt({emitter: _SELF});
        } else if (senderBytes == SenderBytes.ZeroAddress && redeployProtectionFlag == RedeployProtectionFlag.True) {

            guardedSalt = _efficientHash({a: bytes32(block.chainid), b: salt});
        } else if (
            senderBytes == SenderBytes.ZeroAddress && redeployProtectionFlag == RedeployProtectionFlag.Unspecified
        ) {

            revert InvalidSalt({emitter: _SELF});
        } else {

            guardedSalt = (salt != _generateSalt()) ? keccak256(abi.encode(salt)) : salt;
        }
    }

    function _parseSalt(
        bytes32 salt
    ) internal view returns (SenderBytes senderBytes, RedeployProtectionFlag redeployProtectionFlag) {
        if (address(bytes20(salt)) == msg.sender && bytes1(salt[20]) == hex"01") {
            (senderBytes, redeployProtectionFlag) = (SenderBytes.MsgSender, RedeployProtectionFlag.True);
        } else if (address(bytes20(salt)) == msg.sender && bytes1(salt[20]) == hex"00") {
            (senderBytes, redeployProtectionFlag) = (SenderBytes.MsgSender, RedeployProtectionFlag.False);
        } else if (address(bytes20(salt)) == msg.sender) {
            (senderBytes, redeployProtectionFlag) = (SenderBytes.MsgSender, RedeployProtectionFlag.Unspecified);
        } else if (address(bytes20(salt)) == address(0) && bytes1(salt[20]) == hex"01") {
            (senderBytes, redeployProtectionFlag) = (SenderBytes.ZeroAddress, RedeployProtectionFlag.True);
        } else if (address(bytes20(salt)) == address(0) && bytes1(salt[20]) == hex"00") {
            (senderBytes, redeployProtectionFlag) = (SenderBytes.ZeroAddress, RedeployProtectionFlag.False);
        } else if (address(bytes20(salt)) == address(0)) {
            (senderBytes, redeployProtectionFlag) = (SenderBytes.ZeroAddress, RedeployProtectionFlag.Unspecified);
        } else if (bytes1(salt[20]) == hex"01") {
            (senderBytes, redeployProtectionFlag) = (SenderBytes.Random, RedeployProtectionFlag.True);
        } else if (bytes1(salt[20]) == hex"00") {
            (senderBytes, redeployProtectionFlag) = (SenderBytes.Random, RedeployProtectionFlag.False);
        } else {
            (senderBytes, redeployProtectionFlag) = (SenderBytes.Random, RedeployProtectionFlag.Unspecified);
        }
    }

    function _efficientHash(bytes32 a, bytes32 b) internal pure returns (bytes32 hash) {

    }

    function _generateSalt() internal view returns (bytes32 salt) {
        unchecked {
            salt = keccak256(
                abi.encode(

                    blockhash(block.number - 32),
                    block.coinbase,
                    block.number,
                    block.timestamp,
                    block.prevrandao,
                    block.chainid,
                    msg.sender
                )
            );
        }
    }

    function _requireSuccessfulContractCreation(bool success, address newContract) internal view {

        if (!success || newContract == address(0) || newContract.code.length == 0) {
            revert FailedContractCreation({emitter: _SELF});
        }
    }

    function _requireSuccessfulContractCreation(address newContract) internal view {
        if (newContract == address(0) || newContract.code.length == 0) {
            revert FailedContractCreation({emitter: _SELF});
        }
    }

    function _requireSuccessfulContractInitialisation(
        bool success,
        bytes memory returnData,
        address implementation
    ) internal view {
        if (!success || implementation.code.length == 0) {
            revert FailedContractInitialisation({emitter: _SELF, revertData: returnData});
        }
    }
}
