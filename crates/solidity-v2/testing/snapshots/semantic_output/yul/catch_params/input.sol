contract Test {
    function test() public {
        try this.test() {
        } catch Error(string memory reason) {
            assembly {
                let x := reason
            }
        } catch (bytes memory reason) {
            assembly {
                let x := reason
            }
        }
    }
}
