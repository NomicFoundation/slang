pragma solidity *;

// `A.f` and `B.f` differ only in a parameter's data location, which is valid
// before 0.8.14. `super.f` sees two overloads, and the memory argument picks
// `A.f`. Through `this` or a contract value the two collapse by selector, so
// `B.f` wins.
contract A {
    function f(uint256[] memory a) public virtual returns (uint256) {
        return a.length;
    }
}

contract B is A {
    function f(uint256[] calldata a) public override returns (uint256) {
        return a.length + 1;
    }

    function viaThis(uint256[] calldata a) external returns (uint256) {
        return this.f(a);
    }

    function viaSuper(uint256[] memory a) public returns (uint256) {
        return super.f(a);
    }
}

contract C {
    function viaValue(B b, uint256[] calldata a) external returns (uint256) {
        return b.f(a);
    }
}
