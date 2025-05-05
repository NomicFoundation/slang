use anyhow::Result;
use console::{strip_ansi_codes, style, Color, Term};
use num_format::ToFormattedString;

pub struct Terminal;

impl Terminal {
    pub fn wrap_execution(operation: impl FnOnce() -> Result<()>) -> Result<()> {
        let std_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |info| {
            // Print panic to stderr first, including any backtraces.
            std_hook(info);

            // A lot of children commands can exit without clearly indicating success or failure.
            // This acts as a catch all to make sure they don't go unnoticed.
            Self::banner(Color::Red, "Failure");
        }));

        operation()?;

        Self::banner(Color::Green, "Success");

        Ok(())
    }

    pub fn step(message: impl Into<String>) {
        Self::banner(Color::Blue, message);
    }

    fn banner(color: Color, message: impl Into<String>) {
        const BANNER_GLYPHS: usize = 6; // "╾┤  ├╼"

        let message = message.into();
        let message_width = strip_ansi_codes(&message).chars().count();

        let terminal_width = Term::stdout().size().1 as usize;
        let spacer_width = terminal_width.saturating_sub(message_width - BANNER_GLYPHS);

        let left_spacer_width = spacer_width / 2;
        let right_spacer_width = spacer_width - left_spacer_width;

        let contents = format!(
            "{start} {middle} {end}",
            start = style(format!("╾{sep}┤", sep = "─".repeat(left_spacer_width))).dim(),
            middle = style(message).fg(color).bold().bright(),
            end = style(format!("├{sep}╼", sep = "─".repeat(right_spacer_width))).dim(),
        );

        eprintln!();
        eprintln!();
        eprintln!("{contents}");
        eprintln!();
        eprintln!();
    }
}

/// Provides a default formatting for numbers based on repository's language (English).
/// See [`num_format::ToFormattedString`] and [`num_format::Locale::en`].
pub trait NumbersDefaultDisplay {
    fn num_display(&self) -> String;
}

impl<T: ToFormattedString> NumbersDefaultDisplay for T {
    fn num_display(&self) -> String {
        self.to_formatted_string(&num_format::Locale::en)
    }
}
