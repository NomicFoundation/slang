use codegen_schema::types::production::{Production, VersionMap};

use super::ebnf_writer::{EBNFWritable, EBNFWriter};

impl<T: EBNFWriter> EBNFWritable<T> for Production {
    fn write_ebnf(&self, _name: &str, writer: &mut T) {
        writer.write_prelude();

        match &self {
            Production::Scanner { version_map, name } => match version_map {
                VersionMap::Unversioned(scanner) => {
                    writer.write_line_start();
                    scanner.write_ebnf(name, writer);
                    writer.write_line_end();
                }
                VersionMap::Versioned(versions) => {
                    for (version, scanner) in versions {
                        writer.write_line_start();
                        writer.write_comment(&format!("(* v{} *) ", version));
                        scanner.write_ebnf(name, writer);
                        writer.write_line_end();
                    }
                }
            },
            Production::TriviaParser { version_map, name }
            | Production::Parser { version_map, name } => match version_map {
                VersionMap::Unversioned(parser) => {
                    writer.write_line_start();
                    parser.write_ebnf(name, writer);
                    writer.write_line_end();
                }
                VersionMap::Versioned(versions) => {
                    for (version, parser) in versions {
                        writer.write_line_start();
                        writer.write_comment(&format!("(* v{} *) ", version));
                        parser.write_ebnf(name, writer);
                        writer.write_line_end();
                    }
                }
            },
            Production::PrecedenceParser { version_map, name } => match version_map {
                VersionMap::Unversioned(precedence_parser) => {
                    writer.write_line_start();
                    precedence_parser.write_ebnf(name, writer);
                    writer.write_line_end();
                }
                VersionMap::Versioned(versions) => {
                    for (version, precedence_parser) in versions {
                        writer.write_line_start();
                        writer.write_comment(&format!("(* v{} *) ", version));
                        precedence_parser.write_ebnf(name, writer);
                        writer.write_line_end();
                    }
                }
            },
        };

        writer.write_postlude();
    }
}
