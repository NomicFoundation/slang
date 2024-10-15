contract FunctionModifier {
    bool public locked;

    modifier noReentrancy() {
        assert(!locked);

        locked = true;
        _;
        locked = false;
    }

    function decrement() public noReentrancy {
        // ...
    }
}
