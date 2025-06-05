// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

type MemoryPointer is uint256;

using MemoryReaders for MemoryPointer;

MemoryPointer constant FreeMemoryPPtr = MemoryPointer.wrap(0x40);

function getFreeMemoryPointer() pure returns (MemoryPointer mPtr) {
    // `readMemoryPointer` should bind to the library function (issue #1333)
    mPtr = FreeMemoryPPtr.readMemoryPointer();
}

library MemoryReaders {
    function readMemoryPointer(MemoryPointer mPtr) internal pure returns (MemoryPointer value) {
    }
}
