abstract contract Ownable {
    address internal owner;
    address default_visibility;
}
contract Test is Ownable {
    function test() public {
        owner;
        default_visibility;
    }
}
