# User Defined Value Types

A user defined value type allows creating a zero cost abstraction over an elementary value type. This is similar to a type alias.
A user defined value type is defined using `type C is V`, where `C` is the name of the newly introduced type and `V` has to be a built-in value type (the underlying type).

```solidity
type MyInteger is uint256;

library MyLibrary {
    function add(MyInteger a, MyInteger b) internal pure returns (MyInteger) {
        return MyInteger.wrap(MyInteger.unwrap(a) + MyInteger.unwrap(b));
    }
}
```
