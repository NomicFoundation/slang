// Literal arithmetic uses unbounded precision, so intermediate values larger
// than 4096 bits still fold when the result reduces into range. solc limits
// rational constants to 4096 bits of precision and rejects all of these.

contract Test {
    uint256[2 ** 5000 / 2 ** 4990] power;
    uint256[1e2000 / 1e1999] scientific;
    uint256[(1 << 5000) >> 4995] shifted;
}
