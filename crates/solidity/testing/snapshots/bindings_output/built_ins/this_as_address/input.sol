contract Test {
    function test() public {
        // This was valid before 0.5.0
        this.balance;
    }
}
