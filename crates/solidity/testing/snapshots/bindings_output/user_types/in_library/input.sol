contract Test {
    function test() public returns (int32) {
        return MyLib.LibType.unwrap(MyLib.create());
    }
}

library MyLib {
    type LibType is int32;

    function create() public returns (LibType) {
        return LibType.wrap(30);
    }
}
