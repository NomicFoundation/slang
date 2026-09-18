// SPDX-License-Identifier: MIT
pragma solidity *;

interface IBase {
    function foo() external view;
}

contract Base is IBase {
    function foo() public view virtual {}
}

interface IExt is IBase {}

contract Ext is IExt, Base {}

// `Impl.foo` overrides `Base.foo`, a contract function, so the interface
// exemption doesn't apply and the `override` specifier is required.
contract Impl is Ext {
    function foo() public view {}
}
