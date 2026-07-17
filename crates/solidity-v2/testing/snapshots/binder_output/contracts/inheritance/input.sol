contract Base {
    function base_func() public returns (int) {
        return 1;
    }
}

contract Derived is Base {
    function some_func() public returns (int) {
        return 1 + base_func();
    }
}
