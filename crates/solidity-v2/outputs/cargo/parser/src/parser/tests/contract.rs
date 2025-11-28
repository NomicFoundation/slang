use crate::parser::tests::test;

#[test]
fn empty_contract() {
    let source = "contract Foo {}";
    test(source, "33");
}

#[test]
fn two_empty_contracts() {
    let source = "contract Foo {}
    contract Bar {}";
    test(source, "33");
}

#[test]
fn contract_with_function() {
    let source = "contract Foo {
        function bar() public {}
    }";
    test(source, "33");
}
