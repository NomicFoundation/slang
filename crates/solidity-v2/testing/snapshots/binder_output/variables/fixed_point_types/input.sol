contract Foo {
    function bar() public pure {
        // Bare 'fixed'/'ufixed' are aliases for the '128x18' variants.
        fixed a = 0.5;
        ufixed b = 0.5;
        fixed128x18 c = a;
        ufixed8x0 d = 1;
        a;
        b;
        c;
        d;
    }
}
