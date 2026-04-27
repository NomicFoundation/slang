contract Test {
    function test() public returns (uint) {
        try this.test() returns (uint v) {
            assembly {
                v := 0
            }
        } catch {
        }
    }
}
