contract Test {
    struct Foo {
        bytes value;
    }

    function test(bytes32 slot) internal returns (Foo storage r) {
        assembly {
            r.slot := slot
        }
    }
}
