contract Constants {
    enum Direction { NORTH, SOUTH, EAST, WEST }
    //   ^def:1
    //                             ^def:2

    address public constant MY_ADDRESS = 0x777788889999AaAAbBbbCcccddDdeeeEfFFfCcCc;
    //                      ^def:3
    uint256 public constant MY_UINT = 123;
    //                      ^def:4
    Direction public constant MY_DIRECTION = Direction.EAST;
    //                                                 ^ref:2
    //                                       ^ref:1
    //                        ^def:5
    //<ref:1

    function test() public {
        assert(MY_DIRECTION != Direction.NORTH);
        //     ^ref:5
        assert(MY_UINT > 100);
        //     ^ref:4
        require(msg.sender == MY_ADDRESS);
        //                    ^ref:3
    }
}
