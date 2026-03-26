contract Base {
    modifier foo virtual { _; }
}

contract A is Base {
    modifier foo virtual override { _; }
}

contract B is Base {
    modifier foo virtual override { _; }
}

contract Test is B, A {
    function test() public foo {}
}
