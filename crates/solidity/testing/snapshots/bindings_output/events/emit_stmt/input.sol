contract Test {
    event TestEvent(int id);

    function test_emit() public {
        int x = 1;

        emit TestEvent(x);
        emit Utils.Debug(x);
    }
}

library Utils {
    event Debug(int subject);
}
