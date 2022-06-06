use clap::Parser;
use itertools::Itertools;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};
use std::{fs::File, path::PathBuf};
use syntax_schema::schema::Production;

#[derive(Parser, Debug)]
struct ProgramArgs {
    #[clap(long)]
    grammar_file: String,

    #[clap(long)]
    splits_file: String,

    #[clap(long)]
    output_folder: String,
}

#[derive(Deserialize)]
struct TopicMetadata {
    pub productions: Vec<String>,
}

type SectionsList = Vec<HashMap<String, Vec<HashMap<String, TopicMetadata>>>>;

fn main() {
    let args = ProgramArgs::parse();
    let output_folder = PathBuf::from(&args.output_folder);
    let splits_file = PathBuf::from(&args.splits_file);
    let grammar_input = PathBuf::from(&args.grammar_file);

    println!(" => Loading Sections");
    let sections: SectionsList =
        serde_yaml::from_str(&std::fs::read_to_string(splits_file).unwrap()).unwrap();
    validate_split(&sections, &grammar_input);

    println!(" => Generating Manifest");
    generate_manifest(&sections, &output_folder, &grammar_input);
}

fn validate_split(sections: &SectionsList, grammar_file: &PathBuf) {
    let topics_productions = sections
        .iter()
        .flat_map(|section_map| {
            section_map.iter().flat_map(|(_, topics)| {
                topics.iter().flat_map(|topic_map| {
                    topic_map.iter().flat_map(|(_, topic)| {
                        topic
                            .productions
                            .iter()
                            .map(|production| production.clone())
                    })
                })
            })
        })
        .sorted()
        .collect::<Vec<String>>()
        .join("\n")
        + "\n";
    let topics_productions_path =
        std::env::temp_dir().join("slang-syntax-schema-topics-productions");
    std::fs::write(&topics_productions_path, &topics_productions).unwrap();

    let original_productions =
        serde_yaml::from_slice::<Vec<Production>>(&std::fs::read(grammar_file).unwrap()).unwrap();
    let original_productions = original_productions
        .iter()
        .map(|production| production.name.clone())
        .sorted()
        .collect::<Vec<String>>()
        .join("\n")
        + "\n";
    let original_productions_path =
        std::env::temp_dir().join("slang-syntax-schema-original-productions");
    std::fs::write(&original_productions_path, &original_productions).unwrap();

    assert!(
        Command::new("diff")
            .arg("--color=always")
            .arg(&original_productions_path)
            .arg(&topics_productions_path)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success(),
        "Mismatch between topics and grammar productions"
    );
}

fn generate_manifest(sections: &SectionsList, output_folder: &PathBuf, grammar_file: &PathBuf) {
    let topics_dir = output_folder.join("topics");
    if topics_dir.exists() {
        std::fs::remove_dir_all(&topics_dir).unwrap();
    }

    let w = File::create(output_folder.join("manifest.yml")).expect("Unable to create file");
    writeln!(
        &w,
        "# yaml-language-server: $schema=../../../syntax-schema/syntax/schema/manifest.schema.json"
    )
    .unwrap();

    writeln!(&w).unwrap();
    writeln!(&w, "title: Solidity Grammar").unwrap();
    writeln!(&w).unwrap();

    writeln!(&w, "sections:").unwrap();

    sections
        .iter()
        .enumerate()
        .for_each(|(section_index, section_map)| {
            section_map.iter().for_each(|(section_id, topics)| {
                let section_title = generate_title(section_id);

                writeln!(&w, "  - title: {}", section_title).unwrap();
                writeln!(&w, "    topics:").unwrap();

                topics
                    .iter()
                    .enumerate()
                    .for_each(|(topic_index, topic_map)| {
                        topic_map.iter().for_each(|(topic_id, topic)| {
                            let topic_title = generate_title(&topic_id);

                            let topic_relative_path = format!(
                                "{:0>2}-{}/{:0>2}-{}.yml",
                                section_index + 1,
                                section_id,
                                topic_index + 1,
                                topic_id
                            );

                            writeln!(&w, "      - title: {}", topic_title).unwrap();
                            writeln!(&w, "        definition: topics/{}", topic_relative_path)
                                .unwrap();

                            generate_topic(
                                &topics_dir.join(topic_relative_path),
                                &topic.productions,
                                grammar_file,
                            );
                        });
                    });
            });
        });
}

fn generate_topic(topic_file_path: &PathBuf, productions: &Vec<String>, grammar_file: &PathBuf) {
    let grammar_file_contents = std::fs::read_to_string(grammar_file).unwrap();

    std::fs::create_dir_all(&topic_file_path.parent().unwrap()).unwrap();
    let mut w = File::create(topic_file_path).expect("Unable to create file");

    writeln!(
            &w,
            "# yaml-language-server: $schema=../../../../../syntax-schema/syntax/schema/topic.schema.json"
        )
        .unwrap();

    productions.iter().for_each(|production| {
        writeln!(&w).unwrap();
        let mut is_copying = false;

        grammar_file_contents.lines().for_each(|line| {
            if line.eq(&format!("- name: {}", production)) {
                assert!(!is_copying);
                is_copying = true;
            } else if line.starts_with("- name: ") {
                is_copying = false;
            }

            if is_copying {
                writeln!(w, "{}", &line).unwrap();
            }
        });
    });
}

fn generate_title(id: &str) -> String {
    return id
        .split("-")
        .map(|word| {
            let mut chars = word.chars();
            return match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(chars).collect(),
            };
        })
        .collect::<Vec<String>>()
        .join(" ");
}
