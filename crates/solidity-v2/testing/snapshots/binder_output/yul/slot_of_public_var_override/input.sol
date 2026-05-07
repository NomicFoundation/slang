contract Base {
    function x() public view virtual returns (uint) {}
}

contract Test is Base {
    uint public override x;

    function test() internal pure {
        assembly {
            let a := x.slot
        }
    }
}
