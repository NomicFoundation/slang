// In Solidity < 0.7.0 using directives are inherited in sub contracts
library Lib {
    function nop(uint256 x) {}
}
contract Base {
    uint256 totalSupply;
}
contract Middle is Base {
    using Lib for uint256;
}
contract Test is Middle {
    function test() public {
        totalSupply.nop();
    }
}
