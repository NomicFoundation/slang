abstract contract Utils {
    uint256 internal constant _FOO = 1;
}

contract Test is Utils {
    function test() public pure {
        assembly {
            let x := add(1, _FOO)
        }
    }
}
