contract Foo {
    uint x = 10;

    function a_func(uint x) returns (uint) {
        // Solidity < 0.5.0 binds the following `x` differently than >= 0.5.0
        uint y = x + 1;
        {
            uint x = 20;
            return x + y;
        }
    }
}
