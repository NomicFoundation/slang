contract C {
    function f(bool a, bool b) public pure {
        bool c;
        // OK
        c = a == b;
        c = a != b;

        // Not OK
        c = a > b;
        c = a < b;
        c = a >= b;
        c = a <= b;
    }
}
