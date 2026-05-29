contract C {
    mapping(uint => uint) x;
    function f() public returns (bool ret) {
        mapping(uint => uint) storage y = x;
        return x == y;
    }
}
