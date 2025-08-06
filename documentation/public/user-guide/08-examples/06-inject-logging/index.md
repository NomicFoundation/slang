# 8.5. Inject logging

In this example, we use the `BaseRewriter` class to achieve something different: instead of removing nodes, we edit them to insert snippets of code. In particular, we insert in every function a call to a fictitious `log` function, using the name of the function to make it a bit more interesting.

In essence, the `LoggingRewriter` class below works as follow:

1. When traversing a function definition, we collect the name and store it in a local variable. Then, we proceed to recurse, using the `rewriteChildren` function of `BaseRewriter`. Before returning, the name is cleared off.
2. When traversing the list of statements of a function, we prepare the code to inject (if the name is set), by parsing the new statement: the function call to the `log` function. In order to make it properly, we _prepend_ the required trivia, to make sure the spacing is respected (although note: it will also copy newlines and comments!). We then construct the new statements of the function by prepending (note the `unshift`) the injected code into the children of the function statements.
3. Since the `log` function should come from somewhere, we also import it, appending its `import` at the end of the source members of the file.

```ts title="logging-rewriter.mts"
--8<-- "documentation/public/user-guide/08-examples/06-inject-logging/examples/logging-rewriter.mts"
```

Again, we test the functionality on the ongoing Solidity example from [Section 4](../04-find-unused-definitions/index.md). Note how the code was properly inserted in the relevant locations.

```ts title="test-logging-rewriter.mts"
--8<-- "documentation/public/user-guide/08-examples/06-inject-logging/examples/test-logging-rewriter.test.mts"
```
