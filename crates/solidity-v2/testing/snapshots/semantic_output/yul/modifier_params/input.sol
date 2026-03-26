contract Test {
    modifier test(uint x) {
        assembly {
            let y := x
        }
        _;
    }
}
