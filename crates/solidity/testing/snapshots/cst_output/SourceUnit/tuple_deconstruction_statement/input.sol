contract TupleExamples {
    function test() public pure {
        // VariableDeclarationSatatement
        (uint a, uint b) = (10, 20);


        // Assignment expression
        (a, b) = (30, 40);


        // VariableDeclarationSatatement
        (uint e, ) = (1, 2);
        (, uint f) = (1, 2);

        // Assignment expression
        (e, ) = (1, 2);
        (, f) = (1, 2);
    }

    function returnTupleDeconstruction() public pure  {
        uint a; uint b;
        // The result of an assignment expression over tuples is the empty tuple
        // `()`, I think this is the only correct place to use it.
        return (a, b) = (1, 2);
    }
}
