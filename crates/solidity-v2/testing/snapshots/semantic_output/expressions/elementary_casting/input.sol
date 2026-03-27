contract Test {
    using Lib for uint;

    function test(address a) public {
        address(this).balance;
        payable(a).call("");
        uint(10).noop();
    }
}

library Lib {
    function noop(uint x) public returns (uint) {}
}
