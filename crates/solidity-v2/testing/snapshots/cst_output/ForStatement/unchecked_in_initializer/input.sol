contract C {
    function f() public {
        for (unchecked { uint x = 2; }; x < 2; x++) {}
    }
}
