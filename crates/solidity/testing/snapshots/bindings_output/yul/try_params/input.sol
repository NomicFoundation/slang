interface Service { function test() external returns (uint); }

contract Test {
    function test(Service service) public {
        try service.test() returns (uint v) {
            assembly {
                let x := add(v, 1)
            }
        } catch {
        }
    }
}

