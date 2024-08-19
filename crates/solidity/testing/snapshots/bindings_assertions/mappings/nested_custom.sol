contract NestedCustom {
    enum Kind { Alpha, Beta }
    //   ^def:1
    struct Values {
        // ^def:2
        uint balance;
        //   ^def:3
    }
    mapping(address => mapping(Kind => Values)) vaults;
    //                                          ^def:4
    //                                 ^ref:2
    //                         ^ref:1

    function store(address _addr, Kind _kind, uint _amount) public {
        //                                         ^def:5
        //                             ^def:6
        //                 ^def:7
        vaults[_addr][_kind].balance += _amount;
        //                              ^ref:5
        //                   ^ref:3
        //            ^ref:6
        //     ^ref:7
        //<ref:4
    }

    function balance(address _addr, Kind _kind) public returns (uint) {
        //                               ^def:8
        //                   ^def:9
        return vaults[_addr][_kind].balance;
        //                          ^ref:3
        //                   ^ref:8
        //            ^ref:9
        //     ^ref:4
    }
}
