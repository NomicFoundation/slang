contract C {
    function g() external {}

    function f() external returns (address) {
        return this.g.address;
    }
}
