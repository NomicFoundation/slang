contract Base {
    function base_func() returns (int) {
        return 1;
    }
}

contract Derived is Base {
    function some_func() returns (int) {
        return 1 + base_func();
    }
}
