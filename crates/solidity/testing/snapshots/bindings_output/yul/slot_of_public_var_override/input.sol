contract Base {
    function x() external view virtual returns (uint) {}
}

contract Test is Base {
    uint public override x;

    function test() internal pure {
        assembly {
            let a := x.slot
        }
    }
}
