# 8.2. Find usages

A typical use case for an IDE is finding where some variable, function, or type is used in the code base. In Slang this can be easily accomplished by using the [Binding Graph API](../../07-semantic-analysis/02-binding-graph/index.md):

```ts title="find-usages.mts"
--8<-- "documentation/public/user-guide/08-examples/02-find-usages/examples/find-usages.mts"
```

For example, we can look for usages of the `_count` state variable defined in the sample contract:

```solidity
 1: contract Counter {
 2:   uint _count;
           ^^^^^^
 3:   constructor(uint initialCount) {
 4:     _count = initialCount;
        ^^^^^^
 5:   }
 6:   function count() public view returns (uint) {
 7:     return _count;
               ^^^^^^
 8:   }
 9:   function increment(uint delta) public returns (uint) {
10:     require(delta > 0, "Delta must be positive");
11:     _count += delta;
        ^^^^^^
12:     return _count;
               ^^^^^^
13:   }
14: }
```

```ts title="test-find-usages.test.mts"
--8<-- "documentation/public/user-guide/08-examples/02-find-usages/examples/test-find-usages.test.mts"
```
