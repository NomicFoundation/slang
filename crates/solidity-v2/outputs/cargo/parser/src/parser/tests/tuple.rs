use crate::parser::tests::test;

#[test]
fn tuple_assignment() {
    let source = "contract C {
        function f() public {
            uint x;
            uint y;
            (x, y) = (1, 2);
        }
    }";
    test(source, "33");
}

#[test]
fn tuple_assignment_with_empty() {
    let source = "contract C {
        function f() public {
            uint y;
            (, y) = (1, 2);
        }
    }";
    test(source, "33");
}

#[test]
fn tuple_declaration() {
    let source = "contract C {
        function f() public {
            (uint x, uint y) = (1, 2);
        }
    }";
    test(source, "33");
}

#[test]
fn tuple_declaration_with_empty() {
    let source = "contract C {
        function f() public {
            (uint y, ) = (1, 2);
        }
    }";
    test(source, "33");
}
