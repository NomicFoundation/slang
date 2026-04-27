contract Base {
    modifier foo virtual { _; }
}

contract Test is Base {
    modifier foo override(Base) { _; }
    function test() public foo {}
}
