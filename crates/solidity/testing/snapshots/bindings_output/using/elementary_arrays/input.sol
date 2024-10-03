function first(uint[] memory values) returns (uint) {
    return values[0];
}

using {first} for uint[];

function test(uint[] memory values) returns (uint) {
    return values.first();
}
