contract Test {
    function test() public {
        assembly {
            let x.y.z := 0
            let r := add(x.y.z, 20)
        }
    }
}
