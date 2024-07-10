import "./other.sol" as other;

function foo() returns (int) {
    return other.bar();
}
