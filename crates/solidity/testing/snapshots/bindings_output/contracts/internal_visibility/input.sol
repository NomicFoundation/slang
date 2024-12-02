abstract contract Ownable {
    address internal owner;
    address default_visibility;
    function _internal_only() internal {}
}
contract Test is Ownable {
    function test() public {
        owner;
        default_visibility;
        _internal_only();
        Ownable._internal_only();
    }
}
