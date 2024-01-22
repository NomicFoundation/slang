## Function Types

Function types are the types of functions. Variables of function type
can be assigned from functions and function parameters of function type
can be used to pass functions to and return functions from function
calls. They come in two flavors, `internal` and `external`.

Function types are notated as follows:

```solidity
function (<parameter types>) {internal|external} [pure|view|payable] [returns (<return types>)]
```

In contrast to the parameter types, the return types cannot be empty.
If the function type should not return anything, the whole
`returns (<return types>)` part has to be omitted.

By default, function types are internal, so the `internal` keyword can
be omitted. Note that this only applies to function types. Visibility
has to be specified explicitly for functions defined in contracts, they
do not have a default.

```solidity
contract Oracle {
    Request[] private requests;

    function query(bytes memory data, function(uint) external callback) public {
        requests.push(Request(data, callback));
    }

    function reply(uint requestID, uint response) public {
        requests[requestID].callback(response);
    }
}
```

## Mapping Types

Mapping types use the syntax `mapping(_KeyType => _ValueType)` and
variables of mapping type are declared using the syntax
`mapping(_KeyType => _ValueType) _VariableName`.

```solidity
contract MappingExample {
    mapping(address => uint) public balances;

    function update(uint newBalance) public {
        balances[msg.sender] = newBalance;
    }
}
```

--8<-- "crates/solidity/inputs/language/snippets/under-construction.md"
