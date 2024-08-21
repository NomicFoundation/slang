contract Constants {
    function test() public {
        assert(MY_DIRECTION != Direction.NORTH);
        //     ^ref:! (< 0.7.4)
        //     ^ref:5 (>= 0.7.4)
        assert(MY_UINT > 100);
        //     ^ref:! (< 0.7.4)
        //     ^ref:4 (>= 0.7.4)
        require(msg.sender == MY_ADDRESS);
        //                    ^ref:! (< 0.7.4)
        //                    ^ref:3 (>= 0.7.4)
    }
}

enum Direction { NORTH, SOUTH, EAST, WEST }
//   ^def:1 (>= 0.6.0)
//                             ^def:2 (>= 0.6.0)

address constant MY_ADDRESS = 0x777788889999AaAAbBbbCcccddDdeeeEfFFfCcCc;
//               ^def:3 (>= 0.7.4)
uint256 constant MY_UINT = 123;
//               ^def:4 (>= 0.7.4)
Direction constant MY_DIRECTION = Direction.EAST;
//                                          ^ref:2 (>= 0.7.4)
//                                ^ref:1 (>= 0.7.4)
//                 ^def:5 (>= 0.7.4)
//<ref:1 (>= 0.7.4)
