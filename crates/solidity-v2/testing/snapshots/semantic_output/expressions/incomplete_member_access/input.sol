contract Test {
    struct Data {
        uint120 field;
    }

    Data public data;

    function foo() public {
        // complete:
        data.field = 0;

        // incomplete:
        data.
    }
}
