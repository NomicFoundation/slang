contract Base {
    constructor(uint) {}
}

contract Derived is Base(OTHER) { // the initializer expression should NOT bind
    uint constant OTHER = 33;
}
