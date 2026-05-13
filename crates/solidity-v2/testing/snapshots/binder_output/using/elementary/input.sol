function square(int value) returns (int) {}

using {square} for int;

function test(int x) returns (int) {
    return x.square();
}
