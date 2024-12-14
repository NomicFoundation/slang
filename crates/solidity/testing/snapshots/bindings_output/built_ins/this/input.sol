library Lib {
    function test() internal returns (uint) {
        return address(this).balance;
    }
}
