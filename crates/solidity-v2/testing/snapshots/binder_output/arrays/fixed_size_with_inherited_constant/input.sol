// A non-`private` base constant IS inherited, so the derived contract may use
// it as an array length. `N` resolves to the base value and the array is
// `uint256[1]`.
contract Base {
    uint256 internal constant N = 1;
}
contract Derived is Base {
    uint256[N] a;
}
