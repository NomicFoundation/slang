uint constant BASE = 42;

contract Base {
    constructor(uint) {}
}

contract Derived is Base(BASE) {}

contract Other is Base(OTHER) { // the initializer expression should NOT bind
    uint constant OTHER = 33;
}
