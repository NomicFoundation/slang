import "./two.sol" as two;
import * as three from "./three.sol";

function f1() returns (int) {
    return two.f2() + three.f3();
}
