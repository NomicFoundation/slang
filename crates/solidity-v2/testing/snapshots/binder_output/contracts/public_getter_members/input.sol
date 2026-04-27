contract Base {
    address public owner;
}
contract Test {
    function test(Base base) public {
        base.owner().balance;
    }
}
