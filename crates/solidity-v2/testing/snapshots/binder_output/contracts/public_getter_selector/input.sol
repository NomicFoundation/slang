interface Face {
    function x() external view returns (uint);
}

contract Test is Face {
    uint public override x;

    function test() internal {
        this.x.selector;
    }
}
