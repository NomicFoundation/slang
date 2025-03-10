// SPDX-License-Identifier: MIT

pragma solidity ^0.8.9;

abstract contract IReferable {
    function getCommission(
        address _contract,
        address _recipient,
        address _referrer,
        uint256 _value
    )
        public
        view
        virtual
        returns (uint256, uint256);
    function payReferral(
        address _recipient, 
        address payable _referrer,
        uint256 _quantity, 
        uint256 _value
    ) public virtual payable;
}

contract Referable {

    address public r2eAddress;

    constructor(address _r2eAddress) {
        r2eAddress = _r2eAddress;
    }

    function payReferral(address _recipient, address payable _referrer, uint256 _quantity, uint256 _value) internal {
        (uint256 _commission, uint256 _fee) = IReferable(r2eAddress).getCommission(address(this), _recipient, _referrer, _value);
        if (_commission > 0 && _fee > 0) IReferable(r2eAddress).payReferral{value: _commission + _fee}(_recipient, _referrer, _quantity, _value);
    }
}
