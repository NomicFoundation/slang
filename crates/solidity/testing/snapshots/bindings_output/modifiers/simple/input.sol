contract FunctionModifier {
    bool public locked;

    modifier noReentrancy() {
        require(!locked, "No reentrancy");

        locked = true;
        _;
        locked = false;
    }

    function decrement() public noReentrancy {
        // ...
    }
}
