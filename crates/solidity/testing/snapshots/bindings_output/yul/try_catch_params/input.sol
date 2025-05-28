// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract Test {
    function test(Service service) public returns (uint r) {
        try service.test() returns (uint v) {
            assembly {
                r := v
            }
        } catch (bytes memory reason) {
            if (reason.length == 0) {
                revert("error");
            } else {
                assembly {
                    revert(add(32, reason), mload(reason))
                }
            }
        }
    }
}

interface Service {
    function test() external returns (uint);
}
