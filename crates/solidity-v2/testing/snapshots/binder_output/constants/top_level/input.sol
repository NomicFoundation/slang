contract Constants {
    function test() public {
        assert(MY_DIRECTION != Direction.NORTH);
        assert(MY_UINT > 100);
        require(msg.sender == MY_ADDRESS);
    }
}

enum Direction { NORTH, SOUTH, EAST, WEST }

address constant MY_ADDRESS = 0x777788889999AaAAbBbbCcccddDdeeeEfFFfCcCc;
uint256 constant MY_UINT = 123;
Direction constant MY_DIRECTION = Direction.EAST;
