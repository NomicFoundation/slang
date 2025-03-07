# 8.3. Jump to definition

Another often used feature of an IDE is the ability to jump to the definition of a given identifier. Again, we can use the [Binding Graph API](../../07-semantic-analysis/02-binding-graph/index.md) to do it:

```ts title="jump-to-definition.mts"
--8<-- "documentation/public/user-guide/08-examples/03-jump-to-definition/examples/jump-to-definition.mts"
```

The following example shows jumping to the definition of the parameter `delta` in line 11:

```solidity
 1: contract Counter {
 2:   uint _count;
 3:   constructor(uint initialCount) {
 4:     _count = initialCount;
 5:   }
 6:   function count() public view returns (uint) {
 7:     return _count;
 8:   }
 9:   function increment(uint delta) public returns (uint) {
                              ^^^^^
10:     require(delta > 0, "Delta must be positive");
11:     _count += delta;
                  ^^^^^
12:     return _count;
13:   }
14: }
```

```ts title="test-jump-to-definition.test.mts"
--8<-- "documentation/public/user-guide/08-examples/03-jump-to-definition/examples/test-jump-to-definition.test.mts"
```
