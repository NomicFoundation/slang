pragma solidity *;

contract Base {
    function in_base() public virtual {}
}
contract Middle is Base {}
contract Test is Middle {
    function in_base() public override {
        super.in_base();
    }
}
