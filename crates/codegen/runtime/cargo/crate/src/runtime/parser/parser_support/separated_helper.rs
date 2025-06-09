use crate::cst::{Edge, EdgeLabel, IsLexicalContext, Node, TerminalKind, TextRangeExtensions};
use crate::parser::lexer::Lexer;
use crate::parser::parser_support::parser_result::{ParserResult, SkippedUntil};
use crate::parser::parser_support::recovery::skip_until_with_nested_delims;
use crate::parser::parser_support::ParserContext;
use crate::parser::ParseError;

pub struct SeparatedHelper;

impl SeparatedHelper {
    pub(crate) fn run<L: Lexer, LexCtx: IsLexicalContext>(
        input: &mut ParserContext<'_>,
        lexer: &L,
        body_parser: impl Fn(&mut ParserContext<'_>) -> ParserResult,
        separator: TerminalKind,
        separator_label: EdgeLabel,
    ) -> ParserResult {
        let mut accum = vec![];
        loop {
            match body_parser(input) {
                ParserResult::Match(r#match) => {
                    accum.extend(r#match.nodes);

                    match lexer.peek_terminal_with_trivia::<LexCtx>(input) {
                        Some(scanned) if scanned.accepted_as(separator) => {
                            match lexer
                                .parse_terminal_with_trivia::<LexCtx>(input, separator)
                                .with_label(separator_label)
                            {
                                ParserResult::Match(r#match) => {
                                    accum.extend(r#match.nodes);
                                    continue;
                                }
                                _ => unreachable!("We just checked that the separator matches"),
                            }
                        }

                        // Unrecognized, return the accumulated matches.
                        // NOTE: We can't correctly attempt recovery until #600 lands, otherwise we'd risk misparses,
                        // as we need to stop at certain synchronizing terminals (and we can't reliably scan until
                        // a delimiter, as not every list is enclosed in a delimited group).
                        Some(..) | None => return ParserResult::r#match(accum, vec![separator]),
                    }
                }
                // Body was partially parsed, so try to recover by skipping terminals until we see a separator
                ParserResult::IncompleteMatch(incomplete) => {
                    accum.extend(incomplete.nodes);

                    let start = input.position();

                    match skip_until_with_nested_delims::<_, LexCtx>(input, lexer, separator) {
                        // A separator was found, so we can recover the incomplete match
                        Some((found, skipped_range)) if found == separator => {
                            let (kind, label) = if skipped_range.is_empty() {
                                (TerminalKind::MISSING, EdgeLabel::Missing)
                            } else {
                                (TerminalKind::UNRECOGNIZED, EdgeLabel::Unrecognized)
                            };
                            let skipped = input.content(skipped_range.utf8()).to_owned();
                            accum.push(Edge {
                                label,
                                node: Node::terminal(kind, skipped),
                            });
                            input.emit(ParseError::create(
                                skipped_range,
                                incomplete.expected_terminals,
                            ));

                            match lexer
                                .parse_terminal_with_trivia::<LexCtx>(input, separator)
                                .with_label(separator_label)
                            {
                                ParserResult::Match(r#match) => {
                                    accum.extend(r#match.nodes);
                                    continue;
                                }
                                _ => unreachable!("We just checked that the separator matches"),
                            }
                        }

                        // Didn't find a separator during recovery. It might've been the last of the
                        // separatees, so we can't recover to not risk misparses.
                        Some(..) | None => {
                            // Undo the recovery attempt
                            input.set_position(start);

                            return ParserResult::incomplete_match(
                                accum,
                                incomplete.expected_terminals,
                            );
                        }
                    }
                }
                ParserResult::NoMatch(no_match) => {
                    return if accum.is_empty() {
                        ParserResult::no_match(no_match.expected_terminals)
                    } else {
                        ParserResult::incomplete_match(accum, no_match.expected_terminals)
                    };
                }

                ParserResult::SkippedUntil(skipped) => {
                    accum.extend(skipped.nodes);

                    return ParserResult::SkippedUntil(SkippedUntil {
                        nodes: accum,
                        ..skipped
                    });
                }

                ParserResult::PrattOperatorMatch(..) => {
                    unreachable!("PrattOperatorMatch in SeparatedHelper")
                }
            }
        }
    }
}
