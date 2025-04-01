uint constant BASE = 42;

contract Base {
    constructor(uint) {}
}

contract Derived is Base(BASE) {}
