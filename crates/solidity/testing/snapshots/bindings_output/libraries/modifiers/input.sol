library Test {
    modifier withinRange() {
        _;
    }
    function test() internal withinRange() {}
}
