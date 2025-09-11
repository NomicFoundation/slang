contract Base {
    string internal _name;
}

contract Other {
    string private _name;
}

contract Test is Base, Other {
    function test() internal {
        _name;
    }
}
