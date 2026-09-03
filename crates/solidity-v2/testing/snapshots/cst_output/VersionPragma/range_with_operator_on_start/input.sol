// A range's endpoints take no operator of their own, so this is rejected.
//
// solc parses it, then overwrites both endpoints' operators with the `>=` and
// `<=` a range stands for -- reading `<0.1.0 - 0.9.0` as `>=0.1.0 <=0.9.0`, the
// opposite of the `<` that was written. Rejecting the input says so plainly.
pragma solidity <0.1.0 - 0.9.0;
