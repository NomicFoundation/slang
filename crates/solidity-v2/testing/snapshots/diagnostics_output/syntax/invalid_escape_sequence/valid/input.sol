// SPDX-License-Identifier: MIT
pragma solidity *;

contract Escapes {
    // Every escape sequence, and a line continuation.
    string all = "\n \r \t \' \" \\ \x41 \u00e9 \
continued";

    // Unicode string literals take the same escape sequences.
    string unicodeLiteral = unicode"caf\u00e9 \n";

    function f() public pure {
        assembly {
            let a := "\n \x41 \u00e9"
        }
    }
}
