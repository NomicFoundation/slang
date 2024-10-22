contract Test {
    uint last_time;
    function foo() public {
        address origin = tx.origin;
        last_time = now;
        uint price = tx.gasprice;
    }
}
