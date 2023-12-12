#[macro_export]
macro_rules! query {

    ( @clause ) => {};

    ( @clause $text:literal                $( $tail:tt )* ) => { query! { @clause $( $tail )* } };
    ( @clause .                            $( $tail:tt )* ) => { query! { @clause $( $tail )* } };
    ( @clause ! ( $name:ident )            $( $tail:tt )* ) => { query! { @clause $( $tail )* } };
    ( @clause ( #$name:ident $($arg:tt)* ) $( $tail:tt )* ) => { query! { @clause $( $tail )* } };
    ( @clause $node:tt                     $( $tail:tt )* ) => { query! { @node $node } query! { @node_quantifier $( $tail )* } };

    ( @node_quantifier ? $( $tail:tt )* ) => { query! { @node_binding $( $tail )* } };
    ( @node_quantifier * $( $tail:tt )* ) => { query! { @node_binding $( $tail )* } };
    ( @node_quantifier + $( $tail:tt )* ) => { query! { @node_binding $( $tail )* } };
    ( @node_quantifier   $( $tail:tt )* ) => { query! { @node_binding $( $tail )* } };

    ( @node_binding @$name:ident $( $tail:tt )* ) => { query! { @clause $( $tail )* } };
    ( @node_binding              $( $tail:tt )* ) => { query! { @clause $( $tail )* } };

    ( @node ( _ )                                   ) => { query! { @clause $( $child_clauses )* } };
    ( @node ( $head:ident $( $child_clauses:tt )* ) ) => { query! { @clause $( $child_clauses )* } };
    ( @node (             $( $child_clauses:tt )* ) ) => { query! { @clause $( $child_clauses )* } };
    ( @node [             $( $child_clauses:tt )* ] ) => { query! { @clause $( $child_clauses )* } };

    ( $query:tt + ) => { query! { @node $query } };
    ( $query:tt   ) => { query! { @node $query } };

}

query! {
    (comment)+
}

query! {
    (class_declaration
        (decorator)* @the_decorator
        (identifier) @the_name)
}

query! {
    (call_expression
        (identifier) @the_function
        (arguments (string)? @the_string_arg))
}

query! {
    (
        (comment)
        (function_declaration)
    )
}

query! {
    (
        (number)
        ("," (number))*
    )
}

query! {
    (call_expression
        (identifier) @the_function
        (member_expression
            (property_identifier) @the_method))
}
