contract Test {
    modifier validAddress(address _addr) {
        require(_addr != address(0), "Not valid address");
        _;
    }

    constructor (address _addr) validAddress(_addr) {}
}
