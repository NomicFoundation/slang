before all inlines:
Total:
cargo check -vv > out2.log  10.53s user 21.39s system 91% cpu 34.857 total
parts:
[slang_solidity_v2_parser 1.3.0] processing file `/Users/teofr/Documents/Nomic/slang/crates/solidity-v2/outputs/cargo/parser/src/parser/grammar.modified.lalrpop`
[slang_solidity_v2_parser 1.3.0] Phase `Grammar validation` begun
[slang_solidity_v2_parser 1.3.0] Phase `Grammar validation` completed in 0.002158416 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Conditional compilation` begun
[slang_solidity_v2_parser 1.3.0] Phase `Conditional compilation` completed in 0.000173583 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Grammar resolution` begun
[slang_solidity_v2_parser 1.3.0] Phase `Grammar resolution` completed in 0.002181541 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Precedence expansion` begun
[slang_solidity_v2_parser 1.3.0] Phase `Precedence expansion` completed in 0.001969542 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Macro expansion` begun
[slang_solidity_v2_parser 1.3.0] Phase `Macro expansion` completed in 0.002643291 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Token check` begun
[slang_solidity_v2_parser 1.3.0] Phase `Token check` completed in 0.000611083 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Infer types` begun
[slang_solidity_v2_parser 1.3.0] Phase `Infer types` completed in 0.00288775 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Lowering` begun
[slang_solidity_v2_parser 1.3.0] Phase `Lowering` completed in 0.004661333 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Inlining` begun
[slang_solidity_v2_parser 1.3.0] Phase `Inlining` completed in 0.20589125 seconds
[slang_solidity_v2_parser 1.3.0] Building states for public nonterminal `SourceUnit`
[slang_solidity_v2_parser 1.3.0] Phase `LR(1) state construction (lane)` begun
[slang_solidity_v2_parser 1.3.0] Phase `LR(1) state construction (lane)` completed in 6.640936125 seconds


After inline
total: 
cargo check -vv > out2.log  23.51s user 51.28s system 95% cpu 1:18.65 total
parts:
[slang_solidity_v2_parser 1.3.0] processing file `/Users/teofr/Documents/Nomic/slang/crates/solidity-v2/outputs/cargo/parser/src/parser/grammar.modified.lalrpop`
[slang_solidity_v2_parser 1.3.0] Phase `Grammar validation` begun
[slang_solidity_v2_parser 1.3.0] Phase `Grammar validation` completed in 0.002519541 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Conditional compilation` begun
[slang_solidity_v2_parser 1.3.0] Phase `Conditional compilation` completed in 0.00020625 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Grammar resolution` begun
[slang_solidity_v2_parser 1.3.0] Phase `Grammar resolution` completed in 0.002491334 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Precedence expansion` begun
[slang_solidity_v2_parser 1.3.0] Phase `Precedence expansion` completed in 0.0023725 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Macro expansion` begun
[slang_solidity_v2_parser 1.3.0] Phase `Macro expansion` completed in 0.002723667 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Token check` begun
[slang_solidity_v2_parser 1.3.0] Phase `Token check` completed in 0.000728792 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Infer types` begun
[slang_solidity_v2_parser 1.3.0] Phase `Infer types` completed in 0.00307725 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Lowering` begun
[slang_solidity_v2_parser 1.3.0] Phase `Lowering` completed in 0.005306458 seconds
[slang_solidity_v2_parser 1.3.0] Phase `Inlining` begun
[slang_solidity_v2_parser 1.3.0] Phase `Inlining` completed in 0.394203167 seconds
[slang_solidity_v2_parser 1.3.0] Building states for public nonterminal `SourceUnit`
[slang_solidity_v2_parser 1.3.0] Phase `LR(1) state construction (lane)` begun
[slang_solidity_v2_parser 1.3.0] Phase `LR(1) state construction (lane)` completed in 13.990755917 seconds
