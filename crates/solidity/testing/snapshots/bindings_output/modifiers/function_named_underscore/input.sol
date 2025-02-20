contract Foo {
    modifier aModifier() {
        _;
    }

    function _() public {}

    function bar() aModifier public {
        _;
    }
}
