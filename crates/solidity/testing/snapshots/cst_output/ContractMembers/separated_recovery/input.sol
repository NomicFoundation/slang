function all() override(some.ident unexpected tokens, ISomeInterface, Other) public {
  msg.sender.call{do: 1, arg: 1 }();
  msg.sender.call{, empty: 1, parse: 2 }();
  msg.sender.call{arg: 1, missing_expr: , no_semicolon, , }();
  msg.sender.call{arg: 1 unexpected tokens, not: 2, recovered, yet: 3, }();

}

function empty() override(some.ident, /* empty */, other.arg.here, and.here);

function nested_lists() override(some.ident, next.do.that, other.while, next.one, final, ultimate);
function nested_lists() override(some., next.arg, next.one, ultimate);
