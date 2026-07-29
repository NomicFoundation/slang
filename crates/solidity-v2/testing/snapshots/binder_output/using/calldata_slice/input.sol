// A calldata range index `a[i:j]` produces a "calldata slice" type, which solc
// treats as distinct from the underlying array for `using` resolution:
//
//   - A type-specific `using L for T` directive applies to the whole array but
//     NOT to a slice of it (solc: `Member ... not found ... in T calldata
//     slice`), so `firstByte` is unresolved on the slice below.
//   - A `using L for *` directive applies to every type, including a slice.

library L {
    function firstByte(bytes calldata b) internal pure returns (bytes1) {
        return b[0];
    }
}

contract TypeSpecific {
    using L for bytes;

    // Receiver is `bytes`: the directive applies, so this resolves.
    function whole(bytes calldata data) external pure returns (bytes1) {
        return data.firstByte();
    }

    // Receiver is a `bytes` slice: the directive does not apply, so
    // `firstByte` is left unresolved (matching solc).
    function slice(bytes calldata data) external pure returns (bytes1) {
        return data[1:3].firstByte();
    }
}

contract Wildcard {
    using L for *;

    // `for *` reaches every type, including the slice, so this resolves.
    function slice(bytes calldata data) external pure returns (bytes1) {
        return data[1:3].firstByte();
    }
}
