contract AssemblyFunctions {
    function test(uint256 x) public returns (uint256 r) {
        assembly {
            let y := add(x, 5)
            x, y := swap(x, y)
            r := add(x, y)

            function swap(a, b) -> c, d {
                c := b
                d := a
            }
        }
    }
}
