// SPDX-License-Identifier: MIT
pragma solidity *;

contract Strings {
    // Printable ASCII, the other quote, and escape sequences.
    string ascii = "plain ASCII, with 'quotes' and \"escapes\"";
    string escaped = "caf\u00e9 a\tb \x1b";

    // A line continuation holds a raw line break.
    string continued = "line \
continued";

    // Unicode string literals can hold anything.
    string unicodeLiteral = unicode"café ☃";

    function f() public pure {
        assembly {
            let a := "plain ASCII"
            let b := "caf\u00e9 a\tb"
        }
    }
}
