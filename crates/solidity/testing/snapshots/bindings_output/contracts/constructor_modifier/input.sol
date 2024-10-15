contract Test {
    modifier validAddress(address _addr) {
        assert(_addr != address(0));
        _;
    }

    constructor (address _addr) validAddress(_addr) {}
}
