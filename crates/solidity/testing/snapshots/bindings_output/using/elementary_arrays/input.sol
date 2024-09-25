function sum(uint[] memory values) returns (uint) {}

using {sum} for uint[];

function test(uint[] memory values) returns (uint) {
    return values.sum();
}
