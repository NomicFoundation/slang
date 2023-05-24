--8<-- "crates/solidity/inputs/schema/snippets/under-construction.md"

## Importing Files

At a file level, you can use import statements of the following form:

```solidity
import "filename";
```

This statement imports all global symbols from `filename` (and symbols imported there) into the current global scope. This form is not recommended for use, because it unpredictably pollutes the namespace. If you add new top-level items inside `filename`, they automatically appear in all files that import like this from “filename”. It is better to import specific symbols explicitly, which results in all global symbols being available under the `myFile` symbol:

```solidity
import * as myFile from "filename";
// OR
import "filename" as myFile;
```

## Importing Specific Symbols

You can import only the symbols you use from a specific file, using the syntax:

```solidity
import {symbol1, symbol2} from "filename";
```

Which will create `symbol1` and `symbol1` to use in your code. If there is a naming collision, you can rename symbols while importing. For example, the code below creates new global symbols `alias` and `symbol2` which reference `symbol1` and `symbol2` from inside `filename`, respectively:

```solidity
import {symbol1 as alias, symbol2} from "filename";
```

## Virtual File System

In order to be able to support reproducible builds on all platforms, the Solidity compiler has to abstract away the details of the filesystem where source files are stored. For this reason import paths do not refer directly to files in the host filesystem. Instead the compiler maintains an internal database (**virtual filesystem** or **VFS** for short) where each source unit is assigned a unique **source unit name** which is an opaque and unstructured identifier. The import path specified in an import statement is translated into a source unit name and used to find the corresponding source unit in this database.

-   Using the `solc` binary CLI, you can pass disk file system paths to be used.
-   Using the `solc` JSON API, you can pass the explicit file contents to be parsed.
-   Using other tools like the [Remix IDE](https://remix-ide.readthedocs.io/en/latest/import.html) you can import files from HTTP, IPFS and Swarm URLs or refer directly to packages in NPM registry.
