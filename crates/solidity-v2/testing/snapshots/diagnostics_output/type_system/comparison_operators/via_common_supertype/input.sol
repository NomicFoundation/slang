contract Base {}
contract DerivedA is Base {}
contract DerivedB is Base {}

contract C {
    function f(DerivedA da, DerivedB db) public pure {
        da == db;
    }
}
