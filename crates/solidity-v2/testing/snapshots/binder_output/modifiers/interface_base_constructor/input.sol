pragma solidity *;

interface I {}

contract Derived is I {
    constructor() I() {}
}
