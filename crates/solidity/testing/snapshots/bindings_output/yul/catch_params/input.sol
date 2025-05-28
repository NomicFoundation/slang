interface Service { function test() external; }

contract Test {
    function test(Service service) public {
        try service.test() {
        } catch Error(bytes memory reason) {
            assembly {
                revert(add(32, reason), mload(reason))
            }
        } catch (bytes memory reason) {
            assembly {
                revert(add(32, reason), mload(reason))
            }
        }
    }
}
