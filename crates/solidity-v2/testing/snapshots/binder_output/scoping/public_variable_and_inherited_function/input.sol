// A public state variable can co-exist with a same-named inherited function:
// the clash is judged on external signatures, and the getter `x()` differs
// from `A.x(uint256)`. An `internal` variable has no getter and is rejected.
//
// The bare name `x` then denotes the variable in every position, so `callee`
// is a type error, not a resolution one, and slang does not report it yet.
// The function stays reachable as `A.x` and `this.x`.
contract A {
    function x(uint256) public pure returns (uint256) {
        return 1;
    }
}

contract B is A {
    uint256 public x;

    function value() internal view returns (uint256) {
        return x;
    }

    function callee() internal view returns (uint256) {
        return x(1);
    }

    function qualified() internal pure returns (uint256) {
        return A.x(1);
    }

    function external_call() external view returns (uint256) {
        return this.x(1);
    }

    function assembly_use() internal pure {
        assembly {
            let y := x.slot
        }
    }
}
