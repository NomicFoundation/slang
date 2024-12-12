contract InContracts {
    uint256 private constant CONTRACT_CONST = 1;

    function test() public {
        assembly {
            function swap(emptyPtr) {
                mstore(emptyPtr, CONTRACT_CONST)
                mstore(emptyPtr, TOP_LEVEL_CONST)
            }
        }
    }
}

library InLibraries {
    uint256 private constant LIB_CONST = 2;

    function test() public {
        assembly {
            function swap(emptyPtr) {
                mstore(emptyPtr, LIB_CONST)
                mstore(emptyPtr, TOP_LEVEL_CONST)
            }
        }
    }
}

uint256 constant TOP_LEVEL_CONST = 0;
