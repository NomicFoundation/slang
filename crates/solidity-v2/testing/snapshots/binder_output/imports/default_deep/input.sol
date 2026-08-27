// ----- path: main.sol
pragma solidity *;

import "./base.sol";
contract Test is Base, Service {}

// ----- path: base.sol
pragma solidity *;

import "./service.sol";
contract Base {}

// ----- path: service.sol
pragma solidity *;

interface Service {}
