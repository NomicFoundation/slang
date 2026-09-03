// A range's endpoints take no operator of their own, so this is rejected.
// solc parses it and silently discards both operators; see
// `range_with_operator_on_start`.
pragma solidity ^0.1.0 - ^0.9.0;
