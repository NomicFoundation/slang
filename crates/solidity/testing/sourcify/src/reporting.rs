use std::io::Write;
use std::time::Duration;

use console::{style, Color, Term};
use indicatif::{InMemoryTerm, MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};

const TICK_FREQUENCY: Duration = Duration::from_millis(250);

pub struct Reporter {
    parent: MultiProgress,
    children: Vec<ProgressBar>,
    is_visible: bool,
}

impl Reporter {
    pub fn new() -> Self {
        let mut reporter = Self {
            parent: MultiProgress::new(),
            children: vec![],
            is_visible: true,
        };

        // [`MultiProgress`] is created visible by default.
        // Hide until there is data to show:
        reporter.hide();

        reporter
    }

    pub fn println(&self, line: impl AsRef<str>) {
        assert!(self.is_visible);

        self.parent.suspend(|| {
            println!("{0}", line.as_ref());
        });
    }

    pub fn show(&mut self) {
        assert!(!self.is_visible);

        self.parent.set_draw_target(ProgressDrawTarget::stderr());
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        assert!(self.is_visible);

        self.parent.clear().unwrap();
        self.parent.set_draw_target(ProgressDrawTarget::hidden());
        self.is_visible = false;
    }

    pub fn print_full_report(&mut self) {
        assert!(!self.is_visible);

        let (rows, cols) = Term::stdout().size();
        let buffer = InMemoryTerm::new(rows, cols);

        self.parent
            .set_draw_target(ProgressDrawTarget::term_like(Box::new(buffer.clone())));

        for child in &self.children {
            child.disable_steady_tick();
            child.tick();
            child.enable_steady_tick(TICK_FREQUENCY);
        }

        self.parent.set_draw_target(ProgressDrawTarget::hidden());

        std::io::stdout()
            .write_all(buffer.contents_formatted().as_slice())
            .unwrap();

        println!();
    }

    pub fn add_blank(&mut self) {
        let message = " ".repeat(1000);
        let template = "{wide_msg}";

        self.add_bar(message, template, 0);
    }

    pub fn add_label(&mut self, message: &str) {
        let template = "{wide_msg}";
        self.add_bar(message, template, 0);
    }

    pub fn add_progress(&mut self, message: impl Into<String>, total: usize) -> ProgressBar {
        let template = "[{elapsed_precise}]  {msg:^17}  [{wide_bar:.cyan/blue}]  {human_pos:>5}/{human_len:<5}  [ETA: {eta_precise:>3}]";

        self.add_bar(message, template, total)
    }

    pub fn add_counter(
        &mut self,
        message: impl Into<String>,
        color: Color,
        total: usize,
    ) -> ProgressBar {
        let template = format!(
            "  {{msg:<25}}  :  {position}  :  {percent}",
            position = style("{human_pos:>7}").fg(color).bright(),
            percent = style("{percent_precise:>7} %").fg(color).bright(),
        );

        self.add_bar(message, template, total)
    }

    fn add_bar(
        &mut self,
        message: impl Into<String>,
        template: impl AsRef<str>,
        total: usize,
    ) -> ProgressBar {
        let style = ProgressStyle::with_template(template.as_ref())
            .unwrap()
            .progress_chars("#>-");

        let bar = ProgressBar::hidden();

        bar.set_message(message.into());
        bar.set_style(style);
        bar.set_length(total as u64);
        bar.enable_steady_tick(TICK_FREQUENCY);

        self.children.push(bar.clone());
        self.parent.add(bar)
    }
}

impl Drop for Reporter {
    fn drop(&mut self) {
        for child in &self.children {
            child.finish_and_clear();

            self.parent.remove(child);
        }
    }
}
