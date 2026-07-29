contract Base {
    constructor(uint) {}

    function g() public pure returns (uint) {
        return 1;
    }
}

// A qualified call to a base function: the names resolve in the file scope, but
// `Base` must still be recognised as a base of the contract being defined.
contract Derived is Base(Base.g()) {}
