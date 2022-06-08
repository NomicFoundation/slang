use crate::schema::Grammar;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

impl Grammar {
    pub fn generate_spec_navigation(&self, output_path: &PathBuf) {
        let mut w = File::create(output_path).expect("Unable to create file");

        writeln!(w, "nav:").unwrap();
        writeln!(w, "  - Grammar: grammar.md").unwrap();
        writeln!(w, "  - Specification:").unwrap();

        self.manifest
            .sections
            .iter()
            .enumerate()
            .for_each(|(section_index, section)| {
                writeln!(w, "      - {}. {}:", section_index + 1, section.title).unwrap();
                section
                    .topics
                    .iter()
                    .enumerate()
                    .for_each(|(topic_index, topic)| {
                        writeln!(
                            w,
                            "          - {}.{}. {}: specification/{}/index.md",
                            section_index + 1,
                            topic_index + 1,
                            topic.title,
                            self.generate_topic_slug(section_index, topic_index)
                        )
                        .unwrap();
                    });
            });
    }

    pub fn generate_topic_slug(&self, section_index: usize, topic_index: usize) -> String {
        let section = self.manifest.sections.get(section_index).unwrap();
        let topic = section.topics.get(topic_index).unwrap();

        return format!(
            "{:0>2}-{}/{:0>2}-{}",
            section_index + 1,
            section.title.to_ascii_lowercase().replace(" ", "-"),
            topic_index + 1,
            topic.title.to_ascii_lowercase().replace(" ", "-"),
        );
    }
}
