contract Base {
    struct Owner { address owner; }
    Owner public owner;
}
contract Test {
    function test(Base base) public {
        base.owner().balance;
    }
}
