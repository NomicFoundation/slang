## Naming

| Structure          | Name                                               |
| ------------------ | -------------------------------------------------- |
| `Production`       | production name                                    |
| `CharacterFilter`  | `config.name` or [`not_`] single char name or anon |
| `Choice`           | `config.name` or anon                              |
| `DelimitedBy`      | `config.name` or anon                              |
| `Difference`       | minuend name                                       |
| `End`              | `"end_marker"`                                     |
| `ExpressionMember` | production name                                    |
| `Lookahead`        | child                                              |
| `OneOrMore`        | child pluralized                                   |
| `Optional`         | child                                              |
| `Reference`        | production name                                    |
| `Repeated`         | child pluralized                                   |
| `SeparatedBy`      | child pluralized                                   |
| `Sequence`         | `config.name` or anon                              |
| `TerminalTrie`     | `config.name` or single terminal name or anon      |
| `ZeroOrMore`       | child pluralized                                   |

Anonymous members in structs and enums are converted to `_{position}`.

Duplicate names in structs and enums are made unique by adding `_{seq#}`

## Visiting

Is visiting about only rules and terminals, ignoring other structures?

For visiting, every node needs a name, specifically a member of an enum.

Given that rules generate modules thusly:

```rust
type S = ...
mod s {
    struct _T0 {
        a_1: Ta
        b: Tb
        a_2: Ta
        // ...
    }
}
```

This generates an enum for evey visitable component, not only rules but
also substructure.

```rust
enum Rule {
    S_T0_A_1,
    S_T0_B
    S_T0_A_2,
    // ...
}
```

| Typed                | Untyped                    |
| -------------------- | -------------------------- |
| `visit_S_T0_A_1(Ta)` | `visit(Rule::S_T0_A1, ..)` |
| `visit_S_T0_B(Tb)`   | `visit(Rile::S_T0_B, ..)`  |
| `visit_S_T0_A_2(Ta)` | `visit(Rule::S_T0_A2, ..)` |

```rust
enum E {
    C(C)
    D(D)
    // ...
}
```
