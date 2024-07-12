import "./two.sol" as two;
import * as three from "./three.sol";
import {f3 as g3, f4} from './three.sol';

function f1() returns (int) {
    return two.f2() + three.f3() + g3() + f4();
}
