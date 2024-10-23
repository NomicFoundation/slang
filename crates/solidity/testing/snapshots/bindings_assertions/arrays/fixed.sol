contract Test {
    function test() public {
        uint[5] memory values;
        values.pop();
        //     ^ref:!  -- fixed size arrays should not bind pop/push
        values.push(1);
        //     ^ref:!  -- fixed size arrays should not bind pop/push
    }
}
