contract Base {
    struct Single { address a; }
    Single public single;

    struct Multi { uint x; uint y; }
    Multi public multi;
}

contract Test {
    function test(Base base) public {
        base.single().balance;
        (uint x, uint y) = base.multi();
    }
}
