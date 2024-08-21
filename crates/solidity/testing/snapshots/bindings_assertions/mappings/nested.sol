contract NestedMapping {
    mapping(address => mapping(uint256 => bool)) public nested;
    //                                                  ^def:1

    function get(address _addr1, uint256 _i) public view returns (bool) {
        //                               ^def:2
        //               ^def:3
        return nested[_addr1][_i];
        //                    ^ref:2
        //            ^ref:3
        //     ^ref:1
    }

    function set(address _addr1, uint256 _i, bool _boo) public {
        //                                        ^def:4
        //                               ^def:5
        //               ^def:6
        nested[_addr1][_i] = _boo;
        //                   ^ref:4
        //             ^ref:5
        //     ^ref:6
        //<ref:1
    }

    function remove(address _addr1, uint256 _i) public {
        //                                  ^def:7
        //                  ^def:8
        delete nested[_addr1][_i];
        //                    ^ref:7
        //            ^ref:8
        //     ^ref:1
    }
}
