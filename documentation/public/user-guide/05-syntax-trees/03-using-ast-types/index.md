# 5.3. Using AST Types

So far we've been using the CST of a Solidity file or fragment. However, the CST is too verbose, and not always
convenient to use for extracting information of the program. In the following example, we are going to show how to
obtain the parameters and attributes of a function using the AST, an abstract representation of the program tree.

We start as usual by parsing the input, and then we can use the `ParseOutput` root
to create the CST type. Since it is a node of kind `FunctionDefinition`, we are using
the AST type of the same name to analyze it.

The `FunctionDefinition` type has named fields to access all its children.
For example, we can check the name of the function, list its parameters, or attributes:

```ts title="using-ast-types.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/03-using-ast-types/examples/01-using-ast-types.test.mts"
```
