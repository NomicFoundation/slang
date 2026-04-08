contract A is A {} // this should bind even though it makes no sense

contract B is C {} // this should bind even though it makes no sense
contract C is B {} // this should bind even though it makes no sense
