interface IPool {
    struct Info {
        uint256 amount;
    }
}
library Math {
    function nop(uint256 x) public {}
}
contract Test is IPool {
    mapping(uint256 => Info) infos;
    using Math for uint256;
    function test(uint256 x) public {
        infos[x].amount.nop();
    }
}
