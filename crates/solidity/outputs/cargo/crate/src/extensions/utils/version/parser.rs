use metaslang_cst::cursor::Cursor;
use metaslang_cst::kinds::NodeKind;

use super::{Comparator, ComparatorSet, Operator, PartialVersion, Range, VersionPart};
use crate::cst::{EdgeLabel, KindTypes, NonterminalKind, TerminalKind};
use crate::parser::Parser;
use crate::utils::LanguageFacts;

type Result<T> = std::result::Result<T, String>;

pub fn parse_range(mut cursor: Cursor<KindTypes>) -> Result<Range> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionExpressionSets)?;
    let mut range = Range::new();

    while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::VersionExpressionSet) {
        let comparator_set = parse_comparator_set(cursor.spawn())?;
        range.comparator_sets.push(comparator_set);
    }

    Ok(range)
}

fn parse_comparator_set(mut cursor: Cursor<KindTypes>) -> Result<ComparatorSet> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionExpressionSet)?;

    let mut comparator_set = ComparatorSet::new();

    while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::VersionExpression) {
        let subset = parse_version_expression(cursor.spawn())?;
        comparator_set.merge(&subset);
    }

    Ok(comparator_set)
}

fn parse_version_expression(mut cursor: Cursor<KindTypes>) -> Result<ComparatorSet> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionExpression)?;

    cursor.go_to_next_nonterminal_with_kinds(&[
        NonterminalKind::VersionRange,
        NonterminalKind::VersionTerm,
    ]);

    match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::VersionRange) => parse_version_range(cursor.spawn()),
        NodeKind::Nonterminal(NonterminalKind::VersionTerm) => parse_version_term(cursor.spawn()),
        _ => Err(format!(
            "Unexpected variant of kind {}",
            cursor.node().kind()
        )),
    }
}

fn parse_version_range(mut cursor: Cursor<KindTypes>) -> Result<ComparatorSet> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionRange)?;

    let version_start = if cursor.go_to_next_nonterminal_with_kind(NonterminalKind::VersionLiteral)
    {
        parse_version_literal(cursor.spawn())?
    } else {
        return Err("Version range requires a starting version literal".to_string());
    };

    let version_end = if cursor.go_to_next_nonterminal_with_kind(NonterminalKind::VersionLiteral) {
        parse_version_literal(cursor.spawn())?
    } else {
        return Err("Version range requires an ending version literal".to_string());
    };

    Ok(ComparatorSet::hyphen_range(&version_start, &version_end))
}

fn parse_version_term(mut cursor: Cursor<KindTypes>) -> Result<ComparatorSet> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionTerm)?;

    let operator = if cursor.go_to_next_terminal_with_kinds(&[
        TerminalKind::Caret,
        TerminalKind::Tilde,
        TerminalKind::Equal,
        TerminalKind::GreaterThan,
        TerminalKind::GreaterThanEqual,
        TerminalKind::LessThan,
        TerminalKind::LessThanEqual,
    ]) {
        Some(cursor.node().as_terminal().unwrap().kind)
    } else {
        None
    };

    cursor.reset();

    if !cursor.go_to_next_nonterminal_with_kind(NonterminalKind::VersionLiteral) {
        return Err("Expected VersionLiteral".to_string());
    }

    let partial_version = parse_version_literal(cursor.spawn())?;

    if operator == Some(TerminalKind::Caret) {
        return Ok(ComparatorSet::caret(&partial_version));
    }

    if operator == Some(TerminalKind::Tilde) {
        return Ok(ComparatorSet::tilde(&partial_version));
    }

    if partial_version.is_complete() {
        let op = match operator {
            Some(TerminalKind::Equal) => Operator::Eq,
            Some(TerminalKind::GreaterThan) => Operator::Gt,
            Some(TerminalKind::GreaterThanEqual) => Operator::GtEq,
            Some(TerminalKind::LessThan) => Operator::Lt,
            Some(TerminalKind::LessThanEqual) => Operator::LtEq,
            _ => Operator::Eq,
        };
        return Ok(ComparatorSet::single(Comparator {
            version: partial_version.lower_bound(),
            op,
        }));
    }

    let comparator_set = match operator {
        Some(TerminalKind::LessThan) => {
            // <0.7 == <0.7.x == <0.7.0
            ComparatorSet::single(Comparator {
                version: partial_version.lower_bound(),
                op: Operator::Lt,
            })
        }
        Some(TerminalKind::LessThanEqual) => {
            if partial_version.minor.is_wild() || partial_version.patch.is_wild() {
                // <=0.7.x == <0.8.0
                match partial_version.tilde_upper_bound() {
                    Some(upper_version) => ComparatorSet::single(Comparator {
                        version: upper_version,
                        op: Operator::Lt,
                    }),
                    None => ComparatorSet::wild(),
                }
            } else {
                // <=0.7 == <=0.7.0
                ComparatorSet::single(Comparator {
                    version: partial_version.lower_bound(),
                    op: Operator::LtEq,
                })
            }
        }
        Some(TerminalKind::GreaterThan) => {
            if partial_version.minor.is_wild() || partial_version.patch.is_wild() {
                // >0.7.x == >0.7.0
                ComparatorSet::single(Comparator {
                    version: partial_version.lower_bound(),
                    op: Operator::Gt,
                })
            } else {
                // >0.7 == >=0.8.0
                match partial_version.tilde_upper_bound() {
                    Some(upper_version) => ComparatorSet::single(Comparator {
                        version: upper_version,
                        op: Operator::GtEq,
                    }),
                    None => ComparatorSet::none(),
                }
            }
        }
        Some(TerminalKind::GreaterThanEqual) => {
            // >=0.7 == >=0.7.x >=0.7.0
            ComparatorSet::single(Comparator {
                version: partial_version.lower_bound(),
                op: Operator::GtEq,
            })
        }
        Some(TerminalKind::Equal) => ComparatorSet::partial_range(&partial_version),
        Some(_) => unreachable!(),
        None => ComparatorSet::partial_range(&partial_version),
    };

    Ok(comparator_set)
}

fn parse_version_literal(mut cursor: Cursor<KindTypes>) -> Result<PartialVersion> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionLiteral)?;
    go_to_child_with_label(&mut cursor, EdgeLabel::Variant)?;

    match cursor.node().kind() {
        NodeKind::Terminal(_) => parse_quoted_version_literal(&cursor),
        NodeKind::Nonterminal(NonterminalKind::SimpleVersionLiteral) => {
            parse_simple_version_literal(cursor.spawn())
        }
        NodeKind::Nonterminal(_) => Err(format!(
            "Unexpected node with kind {}",
            cursor.node().kind()
        )),
    }
}

fn parse_simple_version_literal(mut cursor: Cursor<KindTypes>) -> Result<PartialVersion> {
    expect_nonterminal_kind(&cursor, NonterminalKind::SimpleVersionLiteral)?;

    let mut partial = PartialVersion::new();

    if !cursor.go_to_next_terminal_with_kind(TerminalKind::VersionSpecifier) {
        return Err(
            "Expected at least one non-trivia node when parsing SimpleVersionLiteral".to_string(),
        );
    }

    {
        let node = cursor.node();
        let major_specifier = node
            .as_terminal_with_kind(TerminalKind::VersionSpecifier)
            .unwrap();

        partial.major = VersionPart::try_from(major_specifier.text.as_str())?;
    }

    if cursor.go_to_next_terminal_with_kind(TerminalKind::VersionSpecifier) {
        let node = cursor.node();
        let minor_specifier = node
            .as_terminal_with_kind(TerminalKind::VersionSpecifier)
            .unwrap();

        partial.minor = VersionPart::try_from(minor_specifier.text.as_str())?;
    }

    if cursor.go_to_next_terminal_with_kind(TerminalKind::VersionSpecifier) {
        let node = cursor.node();
        let patch_specifier = node
            .as_terminal_with_kind(TerminalKind::VersionSpecifier)
            .unwrap();

        partial.patch = VersionPart::try_from(patch_specifier.text.as_str())?;
    }

    Ok(partial)
}

fn parse_quoted_version_literal(cursor: &Cursor<KindTypes>) -> Result<PartialVersion> {
    let quote = if expect_terminal_kind(cursor, TerminalKind::SingleQuotedVersionLiteral).is_ok() {
        '\''
    } else if expect_terminal_kind(cursor, TerminalKind::DoubleQuotedVersionLiteral).is_ok() {
        '"'
    } else {
        return Err(format!(
            "Expected quoted version literal, found {}",
            cursor.node().kind()
        ));
    };

    let version_string = cursor.node().unparse();
    let version_string = version_string
        .strip_prefix(quote)
        .unwrap_or(&version_string);
    let version_string = version_string.strip_suffix(quote).unwrap_or(version_string);

    let parser = Parser::create(LanguageFacts::LATEST_VERSION).unwrap();
    let version_literal =
        parser.parse_nonterminal(NonterminalKind::SimpleVersionLiteral, version_string);

    let cursor = version_literal.create_tree_cursor();

    parse_simple_version_literal(cursor)
}

fn expect_nonterminal_kind(cursor: &Cursor<KindTypes>, kind: NonterminalKind) -> Result<()> {
    if cursor.node().is_nonterminal_with_kind(kind) {
        Ok(())
    } else {
        Err(format!(
            "Expected kind {kind}, found {}",
            cursor.node().kind()
        ))
    }
}

fn expect_terminal_kind(cursor: &Cursor<KindTypes>, kind: TerminalKind) -> Result<()> {
    if cursor.node().is_terminal_with_kind(kind) {
        Ok(())
    } else {
        Err(format!(
            "Expected kind {kind}, found {}",
            cursor.node().kind()
        ))
    }
}

fn expect_label(cursor: &Cursor<KindTypes>, label: EdgeLabel) -> Result<()> {
    if cursor.label() == label {
        Ok(())
    } else {
        Err(format!("Expected label {label}, found {}", cursor.label()))
    }
}

fn go_to_child_with_label(cursor: &mut Cursor<KindTypes>, label: EdgeLabel) -> Result<()> {
    cursor.go_to_first_child();

    loop {
        let is_skippable = cursor.node().is_trivia() || cursor.label() == EdgeLabel::Separator;
        if !is_skippable && expect_label(cursor, label).is_ok() {
            return Ok(());
        }

        if !cursor.go_to_next_sibling() {
            break;
        }
    }

    cursor.go_to_parent();

    Err(format!("No child found with label {label}"))
}
