pragma solidity *;

contract Base {}

contract Test is Base {
    constructor() Base() {
    }
}
