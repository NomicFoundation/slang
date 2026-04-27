library SafeMath {
    function add(uint, uint) internal returns (uint);
}

contract ERC20Basic {
    function balanceOf(address) public returns (uint);
}

contract Token is ERC20Basic {
    function balanceOf(address) public returns (uint);
}

contract Test {
    using SafeMath for uint256;
    Token token;

    function test() public {
        token.balanceOf(this).add(1);
    }
}
