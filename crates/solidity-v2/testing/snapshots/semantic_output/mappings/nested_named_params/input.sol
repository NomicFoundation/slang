contract Test {
    struct A {
        mapping(bytes32 value => uint256) positions;
    }

    mapping(address asset0 => mapping(address asset1 => address)) map;
}
