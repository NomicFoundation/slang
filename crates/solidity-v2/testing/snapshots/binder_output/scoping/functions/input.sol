contract Sample {
    function test() public returns (int) {
        return from_sample() + top_level() + MyLib.from_lib();
    }
    function from_sample() public returns (int) {
        return 1;
    }
}

library MyLib {
    function from_lib() public returns (int) {
        return 3;
    }
}

function top_level() returns (int) {
    return 2;
}
