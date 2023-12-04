At its core, Slang is a collection of APIs that are meant to analyze the source code, starting with the source code itself and ending with a rich structure that can be reasoned about. This is a departure from the classic approach of "black-box" compilers, which are handed the input and only their output can be observed.

## Concrete Syntax Tree (CST)

At the time of writing, Slang is capable of parsing the source code into a Concrete Syntax Tree (CST; also sometimes called "full-fidelity"), which is a tree structure of the program that also includes things like punctuation or whitespace.

This is done by using the (standard) approach of lexical analysis followed by syntax analysis - the source text as a sequence of characters is recognized into a sequence of tokens (lexical analysis), which then in turn is _parsed_ into the CST.

The resulting CST is a regular tree data structure that you can visit. The tree nodes are represented by the [`Node`](https://docs.rs/slang_solidity/latest/slang_solidity/cst/enum.Node.html) structure. _Rule_ nodes (aka _non-terminals_) represent sub-trees, while the _token_ nodes (aka _terminals_) are leaves and represent a lexical token (i.e. an identifier, keyword, punctuation) in the source.

To help navigate the tree, we define and expose a [`Cursor`](https://docs.rs/slang_solidity/latest/slang_solidity/cursor/struct.Cursor.html) API in both Rust and TypeScript.
