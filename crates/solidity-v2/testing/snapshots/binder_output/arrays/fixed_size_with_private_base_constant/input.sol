// A `private` base constant is not inherited, so the same-named constant in the
// derived contract is independent. `N` resolves to the derived value and the
// array is `uint256[2]`. The array length is what exercises this. A plain
// reference would be masked by first-match disambiguation.
contract Base {
    uint256 private constant N = 1;
}
contract Derived is Base {
    uint256 constant N = 2;
    uint256[N] a;
}
