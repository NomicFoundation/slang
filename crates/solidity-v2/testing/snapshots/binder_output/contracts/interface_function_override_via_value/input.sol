pragma solidity *;

// The implementation takes `memory` where the interface declares `calldata`.
// Through a contract value the two collapse by selector, so calls through
// `this` and through the contract type reach `C.f`, and a call through the
// interface type reaches `I.f`.
interface I {
    function f(bytes calldata data) external returns (uint256);
}

contract C is I {
    function f(bytes memory data) public override returns (uint256) {
        return data.length;
    }

    function viaThis(bytes calldata data) external returns (uint256) {
        return this.f(data);
    }

    function viaContractType(bytes calldata data) external returns (uint256) {
        return C(address(this)).f(data);
    }

    function viaInterfaceType(bytes calldata data) external returns (uint256) {
        return I(address(this)).f(data);
    }
}
