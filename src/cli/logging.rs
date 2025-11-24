
use console::Style;

/// Prints colored text to stdout or stderr depending on message type.
pub struct LogMessage;

#[allow(unused)]
impl LogMessage {
    /// Prints an error message (red) to **stderr**.
    pub fn error(message: &str) {
        let style = Style::new().red().bold();
        eprintln!("{}", style.apply_to(message));
    }

    /// Prints a success message (green) to **stdout**.
    pub fn success(message: &str) {
        let style = Style::new().green().bold();
        println!("{}", style.apply_to(message));
    }

    /// Prints a warning message (yellow) to **stdout**.
    pub fn warning(message: &str) {
        let style = Style::new().yellow().bold();
        println!("{}", style.apply_to(message));
    }

    /// Prints an informational message (blue) to **stdout**.
    pub fn info(message: &str) {
        let style = Style::new().blue();
        println!("{}", style.apply_to(message));
    }

    /// Prints a neutral message (white/grey) to **stdout**.
    pub fn neutral(message: &str) {
        let style = Style::new().white();
        println!("{}", style.apply_to(message));
    }
}
