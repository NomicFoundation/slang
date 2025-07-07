contract Test {
    fallback(bytes calldata input) external payable returns (bytes memory output) {
        assembly {
            let x := add(input.length, 1)
            let y := add(output, 1)
        }
    }
}
