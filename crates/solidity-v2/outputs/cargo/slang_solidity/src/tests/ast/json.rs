use serde_json::Value;

use super::fixtures;

#[test]
fn source_unit_serializes_to_json_with_expected_shape() {
    let unit = fixtures::Counter::build_compilation_unit();
    let source_unit = unit.file(&"main.sol".into()).unwrap().ast();

    let json = serde_json::to_value(&source_unit).expect("serialization succeeds");

    let root = json.as_object().expect("root is an object");
    assert_eq!(root["type"], "SourceUnit");
    assert_eq!(root["file"], "main.sol");
    assert!(root["id"].is_u64(), "id must be an integer");

    let range = root["range"].as_object().expect("range is an object");
    assert!(range["start"].is_u64());
    assert!(range["end"].is_u64());

    let members = root["members"].as_array().expect("members is an array");
    assert!(!members.is_empty());

    let contract = members
        .iter()
        .find_map(|node| find_first_node_by_type(node, "ContractDefinition"))
        .expect("at least one contract definition");
    assert_eq!(contract["type"], "ContractDefinition");
    assert_eq!(
        contract["name"]
            .as_object()
            .and_then(|name| name["text"].as_str()),
        Some("Counter"),
    );
}

#[test]
fn function_definition_field_kind_does_not_collide_with_discriminator() {
    let unit = fixtures::Counter::build_compilation_unit();
    let source_unit = unit.file(&"ownable.sol".into()).unwrap().ast();

    let json = serde_json::to_value(&source_unit).unwrap();
    let function =
        find_first_node_by_type(&json, "FunctionDefinition").expect("a FunctionDefinition");

    let obj = function.as_object().unwrap();
    assert_eq!(obj["type"], "FunctionDefinition");
    assert!(
        obj.contains_key("kind"),
        "FunctionDefinition.kind keeps its label since the discriminator is `type`"
    );
}

fn find_first_node_by_type<'a>(node: &'a Value, target: &str) -> Option<&'a Value> {
    let obj = node.as_object()?;
    if obj.get("type").and_then(|v| v.as_str()) == Some(target) {
        return Some(node);
    }
    for child in obj.values() {
        if let Some(found) = find_first_node_by_type(child, target) {
            return Some(found);
        }
        if let Some(items) = child.as_array() {
            for item in items {
                if let Some(found) = find_first_node_by_type(item, target) {
                    return Some(found);
                }
            }
        }
    }
    None
}
