contract A {
    uint256 private constant foo = 1;
    uint256 private bar;
    uint256 private quux;
}

contract B is A {
    bool private constant foo = true;
    bool private bar;
    bool quux;

    function test() internal {
        foo;
        bar;
        quux;
    }
}
