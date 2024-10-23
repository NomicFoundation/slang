# Using the AST

Let's try to analyze the following Solidity source file, containing a simple function:

```solidity title="input.sol"
--8<-- "documentation/public/user-guide/inputs/using-the-ast.sol"
```

We start as usual by parsing the input, and then we can use the `ParseOutput` root
to create the CST type. Since it is a node of kind `FunctionDefinition`, we are using
the AST type of the same name to analyze it:

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-ast.test.mts:imports"

--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-ast.test.mts:parse-input"
```

The `FunctionDefinition` type has named fields to access all its children.
For example, we can check the name of the function:

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-ast.test.mts:create-node"
```

We can also list its parameters:

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-ast.test.mts:list-parameters"
```

Or attributes:

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-ast.test.mts:list-attributes"
```
