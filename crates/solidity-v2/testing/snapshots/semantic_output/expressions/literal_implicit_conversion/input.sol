library Lib {
    function sig(uint, bytes4) internal returns (uint) {}
    function sig(uint, string memory) internal returns (uint) {}
}

contract Test {
    using Lib for uint;
    function test(uint x) internal {
        string memory s;
        x.sig(0x10203040).sig(s);
        x.sig(s).sig(0x10203040);
    }
}
