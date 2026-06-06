contract C {
    function f() public {
        for (uint x = 2; x < 2; unchecked { x++; }) {}
    }
}
