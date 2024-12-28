fn main() {
    println!(
        "{}",
        serde_json::to_string(&solidity_language::SolidityDefinition::create()).unwrap()
    );
}
