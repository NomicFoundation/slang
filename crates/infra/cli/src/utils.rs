use anyhow::{Ok, Result};
use clap::ValueEnum;
use owo_colors::colors::{BrightBlue, BrightGreen, BrightRed};
use owo_colors::{Color, OwoColorize};
use terminal_size::terminal_size;

pub trait OrderedCommand: Clone + Ord + PartialEq + ValueEnum {
    fn execute(&self) -> Result<()>;

    fn execute_in_order(commands: &[Self]) -> Result<()> {
        let mut commands = commands.to_owned();

        if commands.is_empty() {
            // Execute all commands if none are provided:
            commands.extend(Self::value_variants().iter().cloned());
        } else {
            // Sort and deduplicate user provided commands by order of definition:
            commands.sort();
            commands.dedup();
        }

        for command in commands {
            command.execute()?;
        }

        Ok(())
    }
}

pub trait ClapExtensions {
    fn clap_name(&self) -> String;
}

impl<T: ValueEnum> ClapExtensions for T {
    fn clap_name(&self) -> String {
        return self
            .to_possible_value()
            .expect("Expected Clap ValueEnum to have a name (not skipped).")
            .get_name()
            .to_owned();
    }
}

pub struct Terminal;

impl Terminal {
    pub fn step(title: impl Into<String>) {
        Self::banner::<BrightBlue>(title);
    }

    pub fn success() {
        Self::banner::<BrightGreen>("Success");
    }

    pub fn failure() {
        Self::banner::<BrightRed>("Failure");
    }

    fn banner<C: Color>(title: impl Into<String>) {
        const DEFAULT_WIDTH: usize = 100;
        const BANNER_GLYPHS: usize = 6; // "╾┤  ├╼"

        let title = title.into();

        let terminal_width = terminal_size().map_or(DEFAULT_WIDTH, |(width, _)| width.0 as usize);
        let spacer_width = terminal_width - title.chars().count() - BANNER_GLYPHS;

        let left_spacer_width = spacer_width / 2;
        let right_spacer_width = spacer_width - left_spacer_width;

        let contents = format!(
            "{start} {middle} {end}",
            start = format!("╾{sep}┤", sep = "─".repeat(left_spacer_width)).dimmed(),
            middle = title.fg::<C>().bold(),
            end = format!("├{sep}╼", sep = "─".repeat(right_spacer_width)).dimmed(),
        );

        println!();
        println!();
        println!("{contents}");
        println!();
        println!();
    }
}
