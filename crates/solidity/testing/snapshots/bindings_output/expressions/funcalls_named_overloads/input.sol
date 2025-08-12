contract Test {
    function test() internal {
        process({ rcpt: tx.origin, qty: 1 });
        process({ symbol: "foo", qty: 2 });
    }

    function process(uint256 qty, address rcpt) internal {}
    function process(string memory symbol, uint256 qty) internal {}
}
