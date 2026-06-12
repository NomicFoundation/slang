uint constant k = 1;
bool constant k = true;

contract Test {
    uint x;
    bool x;

    function test() public view returns (uint) {
        k;
        x;
        return 0;
    }
}
