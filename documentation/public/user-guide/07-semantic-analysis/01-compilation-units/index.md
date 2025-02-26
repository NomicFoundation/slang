# 7.1. Compilation Units

In order for Slang to perform semantic analysis on a program, it first needs to compile a list of all source files into a single `CompilationUnit`.

We offer a `CompilationBuilder` API that allows you easily do that.
It will parse the provided source files, analyze the import statements within, and then transitively resolve all dependencies, until all source files have been loaded.

It is important to note that Slang is designed to be modular, and work in any environment (CLI, browser, etc..).
This means it cannot access the file system directly, or make any assumptions about where your contracts are, and where their dependencies are installed.

Instead, it expects the user to provide it with a couple of callbacks to handle these tasks:

## Reading Files

The first callback is a function that will read the contents of a source file.
That typically means using an API like NodeJS's `fs.readFile()` for local files, or browser's `fetch()` for remote ones.

Note that this API is also error-tolerant. If the file is not found, or cannot be read, your callback can
simply return `undefined` to indicate that the file is not available.

For simplicity, let's assume that we have the source files defined in code:

```ts title="read-file.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/common/read-file.mts"
```

## Resolving Imports

The second callback is a function that will resolve an import statement to the imported source file.
In a real-world scenario, dependencies can be imported from relative paths on disk, a remote provider like IPFS,
or even NPM packages.

For example, a package manager like `npm` would install the dependencies into sub-directory
of `node_modules`, and users can then resolve their locations via NodeJS `path.resolve()` or browsers `import.meta.resolve()` APIs.

Note that likewise, this API is also error-tolerant. If the import cannot be resolved, your callback can
also return `undefined` to indicate that the import is not available, and the builder will skip it.

For simplicity, let's just assume that dependencies will always be imported by their bare file name:

```ts title="resolve-import.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/common/resolve-import.mts"
```

## Running the compilation builder

With these callbacks defined, we can now create a `CompilationBuilder` and add our source files to it.
Note that in the example below, we don't need to add dependencies, as they will be resolved and loaded automatically.

```ts title="compilation-builder.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/common/compilation-builder.mts"
```

## Inspecting the compilation unit

The built `CompilationUnit` will then contain all the source files, along with their syntax trees.

```ts title="compilation-unit.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/01-compilation-units/examples/compilation-unit.test.mts"
```
