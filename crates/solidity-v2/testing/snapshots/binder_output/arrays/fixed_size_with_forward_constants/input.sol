// Same-file forward references do not fold as array lengths; a rejected
// length reads back as 0. `E` is declared before its use, but its value
// refers to `N` past the use site, so the chain is rejected as well.

uint256 constant P = 7;
uint256 constant E = N;

contract Test {
    uint256[P] backward;
    uint256[E] chained;
    uint256[N] file_level;
    uint256[M] contract_level;

    uint256 constant M = 4;
}

uint256 constant N = 3;
