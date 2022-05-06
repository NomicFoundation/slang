# Mapping Types

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
