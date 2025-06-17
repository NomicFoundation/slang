// ----- path: main.sol
import "./base.sol";
contract Test is Base, Service {}

// ----- path: base.sol
import "./service.sol";
abstract contract Base {}

// ----- path: service.sol
interface Service {}
