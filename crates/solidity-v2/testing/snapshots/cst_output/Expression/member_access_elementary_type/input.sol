contract C {
    function f() {
        // The `address` below should be parsed as an elementary type, rather than as an identifier path element
        address.x;
        // This one is an identifier path element
        int.address;
    }
}
