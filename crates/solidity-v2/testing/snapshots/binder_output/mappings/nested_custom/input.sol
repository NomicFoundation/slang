contract NestedCustom {
    enum Kind { Alpha, Beta }
    struct Values {
        uint balance;
    }
    mapping(address => mapping(Kind => Values)) vaults;

    function store(address _addr, Kind _kind, uint _amount) public {
        vaults[_addr][_kind].balance += _amount;
    }

    function balance(address _addr, Kind _kind) public returns (uint) {
        return vaults[_addr][_kind].balance;
    }
}
