use num_bigint::BigInt;
use num_rational::BigRational;

use super::fixtures;
use crate::ast::{self, BuiltIn, ContractBase, ContractMember, Definition, FunctionKind, Number};

#[derive(Default)]
struct IdentifierCounter {
    total: usize,
    definitions: usize,
    references: usize,
}

impl ast::visitor::Visitor for IdentifierCounter {
    fn visit_identifier(&mut self, node: &ast::Identifier) {
        if node.is_name_of_definition() {
            self.definitions += 1;
        }
        if node.is_reference() {
            self.references += 1;
        }
        self.total += 1;
    }
}

#[test]
fn test_ast_visitor() {
    let unit = fixtures::Counter::build_compilation_unit();

    let main_ast = unit.file("main.sol").unwrap().ast();

    let mut main_visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&main_ast, &mut main_visitor);

    assert_eq!(main_visitor.total, 25);
    assert_eq!(main_visitor.definitions, 9);
    assert_eq!(main_visitor.references, 18);

    let ownable_ast = unit.file("ownable.sol").unwrap().ast();

    let mut ownable_visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&ownable_ast, &mut ownable_visitor);

    assert_eq!(ownable_visitor.total, 11);
    assert_eq!(ownable_visitor.definitions, 3);
    assert_eq!(ownable_visitor.references, 8);

    let activatable_ast = unit.file("activatable.sol").unwrap().ast();

    let mut activatable_visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&activatable_ast, &mut activatable_visitor);

    assert_eq!(activatable_visitor.total, 31);
    assert_eq!(activatable_visitor.definitions, 10);
    assert_eq!(activatable_visitor.references, 22);
}

#[test]
fn test_identifier_path_resolve_to_immediate_definition() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("contract is found");
    let counter_bases: Vec<_> = counter
        .inheritance_types()
        .iter()
        .map(|base_type| base_type.type_name())
        .collect();
    assert_eq!(counter_bases.len(), 2);

    assert_eq!(counter_bases[0].name(), "Ownable");
    assert!(matches!(
        counter_bases[0].resolve_to_definition(),
        Some(ast::Definition::Contract(_))
    ));
    assert!(matches!(
        counter_bases[0].resolve_to_immediate_definition(),
        Some(ast::Definition::ImportedSymbol(_))
    ));

    assert_eq!(counter_bases[1].name(), "Activatable");
    assert!(matches!(
        counter_bases[1].resolve_to_definition(),
        Some(ast::Definition::Contract(_))
    ));
    assert!(matches!(
        counter_bases[1].resolve_to_immediate_definition(),
        Some(ast::Definition::ImportedSymbol(_))
    ));
}

#[test]
fn test_identifier_path_resolve_to_immediate_resolves_to_direct_definition() {
    let unit = fixtures::ChainedImports::build_compilation_unit();

    let a1 = unit.find_contract_by_name("A1").expect("contract is found");
    let i1_typename = a1
        .inheritance_types()
        .iter()
        .next()
        .expect("there is at least one inheritance type")
        .type_name();
    assert_eq!(i1_typename.name(), "I1");
    let Some(i1) = i1_typename.resolve_to_definition() else {
        panic!("i1 can be resolved");
    };
    let Some(i1_immediate) = i1_typename.resolve_to_immediate_definition() else {
        panic!("i1 can be resolved immediately");
    };

    assert!(matches!(
        (i1, i1_immediate),
        (ast::Definition::Interface(_), ast::Definition::Interface(_))
    ));
}

#[test]
fn test_chained_imports_resolution() {
    let unit = fixtures::ChainedImports::build_compilation_unit();

    let a1 = unit.find_contract_by_name("A1").expect("contract is found");
    let b1_typename = a1
        .inheritance_types()
        .iter()
        .nth(1)
        .expect("there are at least two inheritance types")
        .type_name();
    assert_eq!(b1_typename.name(), "B1");

    let b1 = b1_typename
        .resolve_to_immediate_definition()
        .expect("b1 base can be resolved");
    let ast::Definition::ImportedSymbol(b1_import) = b1 else {
        panic!("b1 resolves to an import symbol");
    };

    let b2 = b1_import
        .name()
        .resolve_to_immediate_definition()
        .expect("b1 import can be resolved");
    let ast::Definition::ImportedSymbol(b2_import) = b2 else {
        panic!("b2 resolves to an import symbol");
    };

    let b3 = b2_import
        .name()
        .resolve_to_immediate_definition()
        .expect("b2 import can be resolved");
    let ast::Definition::Contract(b3_contract) = b3 else {
        panic!("b3 resolves to a contract");
    };
    assert_eq!(b3_contract.name().name(), "B3");
}

#[test]
fn test_get_type() {
    let unit = fixtures::Counter::build_compilation_unit();

    let ownable = unit
        .find_contract_by_name("Ownable")
        .expect("contract is found");

    let state_variables = ownable
        .members()
        .iter()
        .filter_map(|member| {
            if let ast::ContractMember::StateVariableDefinition(definition) = member {
                Some(definition)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(state_variables.len(), 1);
    let owner = &state_variables[0];
    assert_eq!(owner.name().name(), "_owner");

    let owner_type = owner
        .get_type()
        .expect("_owner state variable has resolved type");
    assert!(matches!(owner_type, ast::Type::Address(_)));
}

#[test]
fn test_function_get_type() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("contract is found");

    let increment = counter
        .members()
        .iter()
        .find_map(|member| {
            if let ast::ContractMember::FunctionDefinition(function_definition) = member {
                if function_definition
                    .name()
                    .is_some_and(|name| name.name() == "increment")
                {
                    Some(function_definition)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .expect("increment method is found");

    let increment_type = increment.get_type().expect("increment method has a type");
    let ast::Type::Function(function_type) = increment_type else {
        panic!("method's type is expected to be a function");
    };
    assert!(function_type.is_externally_visible());
    assert!(matches!(function_type.return_type(), ast::Type::Integer(_)));
    assert!(function_type
        .associated_definition()
        .is_some_and(|definition| matches!(definition, ast::Definition::Function(_))));
}

#[test]
fn test_contract_direct_bases() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("can find Counter contract");
    let bases = counter.direct_bases();
    assert_eq!(bases.len(), 2);

    let ContractBase::Contract(ownable) = &bases[0] else {
        panic!("Base is not a contract");
    };
    assert_eq!(ownable.name().name(), "Ownable");
    let ContractBase::Contract(activatable) = &bases[1] else {
        panic!("Base is not a contract");
    };
    assert_eq!(activatable.name().name(), "Activatable");
}

#[test]
fn test_contract_compute_linearised_bases() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("can find Counter contract");
    let bases = counter.compute_linearised_bases();
    assert_eq!(bases.len(), 3);

    let ContractBase::Contract(counter) = &bases[0] else {
        panic!("Base is not a contract");
    };
    assert_eq!(counter.name().name(), "Counter");
    let ContractBase::Contract(activatable) = &bases[1] else {
        panic!("Base is not a contract");
    };
    assert_eq!(activatable.name().name(), "Activatable");
    let ContractBase::Contract(ownable) = &bases[2] else {
        panic!("Base is not a contract");
    };
    assert_eq!(ownable.name().name(), "Ownable");
}

#[test]
fn test_definition_references() {
    let unit = fixtures::Counter::build_compilation_unit();

    let ownable = unit
        .find_contract_by_name("Ownable")
        .expect("can find Ownable contract");

    // find the `onlyOwner` modifier defined in the `Ownable` contract
    let only_owner = ownable
        .members()
        .iter()
        .find_map(|member| {
            let ContractMember::FunctionDefinition(function) = member else {
                return None;
            };
            if matches!(function.kind(), FunctionKind::Modifier)
                && function
                    .name()
                    .is_some_and(|name| name.name() == "onlyOwner")
            {
                Some(function)
            } else {
                None
            }
        })
        .expect("can find onlyOwner modifier");

    let references = only_owner.references();
    assert_eq!(references.len(), 3);
    assert!(references.iter().all(|reference| reference
        .resolve_to_definition()
        .and_then(|definition| {
            if let Definition::Modifier(modifier) = definition {
                Some(modifier)
            } else {
                None
            }
        })
        .is_some_and(|modifier| modifier
            .name()
            .is_some_and(|name| name.name() == "onlyOwner"))));
}

#[test]
fn test_contract_compute_linearised_state_variables() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("can find Counter contract");

    let state_variables = counter.compute_linearised_state_variables();
    assert_eq!(state_variables.len(), 4);

    assert_eq!(state_variables[0].name().name(), "_owner");
    assert_eq!(state_variables[1].name().name(), "_state");
    assert_eq!(state_variables[2].name().name(), "count");
    assert_eq!(state_variables[3].name().name(), "_clickers");
}

#[test]
fn test_contract_compute_linearised_functions() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("can find Counter contract");

    let functions = counter.compute_linearised_functions();
    assert_eq!(functions.len(), 5);

    assert!(functions[0]
        .name()
        .is_some_and(|name| name.name() == "click"));
    assert!(functions[1]
        .name()
        .is_some_and(|name| name.name() == "disable"));
    assert!(functions[2]
        .name()
        .is_some_and(|name| name.name() == "enable"));
    assert!(functions[3]
        .name()
        .is_some_and(|name| name.name() == "increment"));
    assert!(functions[4]
        .name()
        .is_some_and(|name| name.name() == "isEnabled"));
}

#[test]
fn test_contract_constructor_and_modifiers() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("can find Counter contract");

    let constructor = counter.constructor();
    assert!(constructor.is_some());

    let modifiers = counter.modifiers();
    assert_eq!(modifiers.len(), 0);
}

#[test]
fn test_contract_compute_linearised_functions_with_overrides() {
    let unit = fixtures::Overrides::build_compilation_unit();

    let inherited = unit
        .find_contract_by_name("Inherited")
        .expect("can find contract");
    let functions = inherited.compute_linearised_functions();
    assert_eq!(functions.len(), 3);
    assert!(functions[0]
        .name()
        .is_some_and(|name| name.name() == "in_base"));
    assert!(functions[1]
        .name()
        .is_some_and(|name| name.name() == "in_middle"));
    assert!(functions[2]
        .name()
        .is_some_and(|name| name.name() == "override_me"));
}

#[test]
fn test_resolve_to_built_in() {
    let unit = fixtures::Counter::build_compilation_unit();

    // Collect all identifiers that resolve to built-ins from the ownable.sol file,
    // which contains `msg.sender` and `require`.
    let ownable_ast = unit.file("ownable.sol").unwrap().ast();

    let mut built_in_collector = BuiltInCollector::default();
    ast::visitor::accept_source_unit(&ownable_ast, &mut built_in_collector);

    assert!(
        built_in_collector
            .found
            .contains(&("msg".to_string(), BuiltIn::Msg)),
        "expected Msg built-in, found: {:?}",
        built_in_collector.found,
    );
    assert!(
        built_in_collector
            .found
            .contains(&("sender".to_string(), BuiltIn::MsgSender)),
        "expected MsgSender built-in, found: {:?}",
        built_in_collector.found,
    );
    assert!(
        built_in_collector
            .found
            .contains(&("require".to_string(), BuiltIn::Require)),
        "expected Require built-in, found: {:?}",
        built_in_collector.found,
    );
}

#[derive(Default)]
struct BuiltInCollector {
    found: Vec<(String, BuiltIn)>,
}

impl ast::visitor::Visitor for BuiltInCollector {
    fn visit_identifier(&mut self, node: &ast::Identifier) {
        if let Some(built_in) = node.resolve_to_built_in() {
            self.found.push((node.name(), built_in));
        }
    }
}

#[test]
fn test_get_file_id_and_text_range() {
    let unit = fixtures::Counter::build_compilation_unit();

    let ownable = unit
        .find_contract_by_name("Ownable")
        .expect("contract is found");
    assert_eq!(ownable.get_file_id(), "ownable.sol");

    let owner = ownable
        .members()
        .iter()
        .find_map(|member| {
            if let ast::ContractMember::StateVariableDefinition(definition) = member {
                Some(definition)
            } else {
                None
            }
        })
        .expect("_owner state variable is found");
    assert_eq!(owner.name().name(), "_owner");
    assert_eq!(owner.get_file_id(), "ownable.sol");

    // The state variable's range must sit within the enclosing contract's range.
    assert!(ownable.get_text_range().start <= owner.get_text_range().start);
    assert!(ownable.get_text_range().end >= owner.get_text_range().end);

    let activatable = unit
        .find_contract_by_name("Activatable")
        .expect("contract is found");
    assert_eq!(activatable.get_file_id(), "activatable.sol");

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("contract is found");
    assert_eq!(counter.get_file_id(), "main.sol");
}

#[derive(Default)]
struct NumberLiteralCollector {
    hex: Vec<(String, Option<BigInt>, Option<Number>)>,
    decimal: Vec<(String, Option<BigInt>, Option<Number>)>,
}

impl ast::visitor::Visitor for NumberLiteralCollector {
    fn enter_hex_number_expression(&mut self, node: &ast::HexNumberExpression) -> bool {
        let text = node.literal().unparse().to_owned();
        self.hex
            .push((text, node.integer_value(), node.number_value()));
        true
    }

    fn enter_decimal_number_expression(&mut self, node: &ast::DecimalNumberExpression) -> bool {
        let text = node.literal().unparse().to_owned();
        self.decimal
            .push((text, node.integer_value(), node.number_value()));
        true
    }
}

fn integer(value: u128) -> Number {
    Number::Integer(BigInt::from(value))
}

fn rational(numerator: i64, denominator: u64) -> Number {
    Number::Rational(BigRational::new(
        BigInt::from(numerator),
        BigInt::from(denominator),
    ))
}

#[test]
fn test_hex_number_expression_value() {
    let unit = fixtures::NumberLiterals::build_compilation_unit();
    let ast = unit.file("main.sol").unwrap().ast();

    let mut collector = NumberLiteralCollector::default();
    ast::visitor::accept_source_unit(&ast, &mut collector);

    // Each entry: (literal source text, expected integer value). A 40-digit hex literal is
    // typed as an address, but its integer/number value is still surfaced as an integer.
    let expected: &[(&str, u128)] = &[
        ("0xff", 255),
        ("0xDeAdBeEf", 3_735_928_559),
        ("0x1234_5678", 305_419_896),
        ("0x0", 0),
        ("0x000000000000000000000000000000000000beef", 0xbeef),
    ];

    assert_eq!(
        collector.hex.len(),
        expected.len(),
        "unexpected number of hex literals visited: {:?}",
        collector.hex,
    );

    for ((text, integer_value, number_value), (expected_text, expected_value)) in
        collector.hex.iter().zip(expected.iter())
    {
        assert_eq!(text, expected_text);
        assert_eq!(integer_value, &Some(BigInt::from(*expected_value)));
        assert_eq!(number_value, &Some(integer(*expected_value)));
    }
}

#[test]
fn test_decimal_number_expression_value() {
    let unit = fixtures::NumberLiterals::build_compilation_unit();
    let ast = unit.file("main.sol").unwrap().ast();

    let mut collector = NumberLiteralCollector::default();
    ast::visitor::accept_source_unit(&ast, &mut collector);

    // Each entry: (literal source text, expected integer value if any, expected number value).
    // The expected integer value is None for rational literals that do not reduce to an integer
    // (with their unit applied).
    let expected: &[(&str, Option<u128>, Number)] = &[
        ("0", Some(0), integer(0)),
        ("42", Some(42), integer(42)),
        ("1_000_000", Some(1_000_000), integer(1_000_000)),
        ("1e3", Some(1_000), integer(1_000)),
        ("1500e-2", Some(15), integer(15)),
        ("1", Some(1), integer(1)),
        ("1", Some(1_000_000_000), integer(1_000_000_000)),
        (
            "1",
            Some(1_000_000_000_000_000_000),
            integer(1_000_000_000_000_000_000),
        ),
        (
            "0.5",
            Some(500_000_000_000_000_000),
            integer(500_000_000_000_000_000),
        ),
        ("1", Some(3_600), integer(3_600)),
        ("0.5", None, rational(1, 2)),
        ("4", Some(4), integer(4)),
        ("1.5", None, rational(3, 2)),
        ("2", Some(2), integer(2)),
    ];

    assert_eq!(
        collector.decimal.len(),
        expected.len(),
        "unexpected number of decimal literals visited: {:?}",
        collector.decimal,
    );

    for ((text, integer_value, number_value), (expected_text, expected_integer, expected_number)) in
        collector.decimal.iter().zip(expected.iter())
    {
        assert_eq!(text, expected_text);
        assert_eq!(
            integer_value,
            &expected_integer.map(BigInt::from),
            "integer_value mismatch for literal {text:?}",
        );
        assert_eq!(
            number_value,
            &Some(expected_number.clone()),
            "number_value mismatch for literal {text:?}",
        );
    }
}
