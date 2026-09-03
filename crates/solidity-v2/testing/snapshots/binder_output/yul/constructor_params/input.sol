pragma solidity *;

contract Test {
    constructor (uint x) {
        assembly {
            let y := x
        }
    }
}
