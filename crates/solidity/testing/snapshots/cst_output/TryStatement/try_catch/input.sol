// Make sure that error recovery won't lead to misparsing
// ambiguous function call options with the block following the try expression

try foo() {
  bar();
} catch {
}
