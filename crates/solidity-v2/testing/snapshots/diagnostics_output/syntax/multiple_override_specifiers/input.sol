pragma solidity ^0.8.0;

contract A {
    uint public x;
    function foo() public virtual {}
    fallback() external virtual payable {}
    receive() external virtual payable {}
    modifier onlyOwner() virtual {}
}

contract B is A {
    function foo() public override override {}
    fallback() external override override payable {}
    receive() external override override payable {}
    uint public override override x;
    modifier onlyOwner() override override {}
}
contract C is A {
    function foo() public override {}
    fallback() external override payable {}
    receive() external override payable {}
    uint public override x;
    modifier onlyOwner() override {}
}
// (removed duplicate B and C)
