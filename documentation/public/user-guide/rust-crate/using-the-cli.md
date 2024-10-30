# Using the CLI

## Parsing Source Files

The `parse` command will take a path to a Solidity file, and a `--version` flag.
Specifying the correct version is important, as it will affect the grammar used to parse inputs.

All parse errors are printed in a human-readable format; the command will succeed if there are no parse errors, and fail otherwise.

```bash
$ slang_solidity parse --help

Usage: slang_solidity parse [OPTIONS] --version <VERSION> <FILE_PATH>

Arguments:
  <FILE_PATH>
          File path to the Solidity (*.sol) source file to parse

Options:
  -v, --version <VERSION>
          The Solidity language version to use for parsing
      --json
          Print the concrete syntax tree as JSON
  -h, --help
          Print help
```

Here is an example of the JSON output it can print:

```json
// A Nonterminal node
"Nonterminal": {
  // Name of the nonterminal kind
  "kind": "SourceUnit",
  // Length of the nonterminal in Unicode code points, depending on the encoding used
  "text_len": {
    "utf8": 24,
    "utf16": 24,
    "char": 24 // de facto utf32
  },
  "children": [/* Nonterminal or Terminal nodes */]
}
// A Terminal node
"Terminal": {
  // Name of the terminal kind
  "kind": "PragmaKeyword",
  // Literal value, taken from the source code
  "text": "pragma"
}
```

## Inspecting JSON Output

Now let's try to use that command to parse the following Solidity file, and inspect its contents:

```solidity title="input.sol"
pragma solidity ^0.8.0;
```

```bash
slang_solidity parse --json --version "0.8.0" "input.sol" > "output.json"
```

Because the resulting structure is well-defined and recursive, we can use the popular `jq` tool to quickly analyze the resulting output:

```bash
JQ_QUERY='recurse | select(.Terminal?) | .Terminal'
cat output.json | jq "$JQ_QUERY"
```

This gives us a flat list of the Terminal nodes:

```json
{
  "kind": "PragmaKeyword",
  "text": "pragma"
}
{
  "kind": "Whitespace",
  "text": " "
}
{
  "kind": "SolidityKeyword",
  "text": "solidity"
}
{
  "kind": "Whitespace",
  "text": " "
}
{
  "kind": "Caret",
  "text": "^"
}
{
  "kind": "VersionPragmaValue",
  "text": "0"
}
{
  "kind": "Period",
  "text": "."
}
{
  "kind": "VersionPragmaValue",
  "text": "8"
}
{
  "kind": "Period",
  "text": "."
}
{
  "kind": "VersionPragmaValue",
  "text": "0"
}
{
  "kind": "Semicolon",
  "text": ";"
}
{
  "kind": "EndOfLine",
  "text": "\n"
}
```

Now, we can adapt the query to select the `text` fields of the nodes and concatenate them,
which gives us back the reconstructed source code! 🎉

```bash
$ JQ_QUERY='[recurse | select(.Terminal?) | .Terminal.text] | join("")'
$ cat output.json | jq "$JQ_QUERY"

"pragma solidity ^0.8.0;\n"
```
