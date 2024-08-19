contract Mapping {
    mapping(address => uint256) public myMap;
    //                                 ^def:1

    function get(address _addr) public view returns (uint256) {
        //               ^def:2
        return myMap[_addr];
        //           ^ref:2
        //     ^ref:1
    }

    function set(address _addr, uint256 _i) public {
        //                              ^def:3
        //               ^def:4
        myMap[_addr] = _i;
        //             ^ref:3
        //    ^ref:4
        //<ref:1
    }

    function remove(address _addr) public {
        //                  ^def:5
        delete myMap[_addr];
        //           ^ref:5
        //     ^ref:1
    }
}
