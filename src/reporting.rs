use std::error::Error;

/// Manages error reporting to the user.
pub struct Reporter {
    /// Whether or not a report was previously reported.
    pub had_error: bool,
}

impl Reporter {
    /// Allocates a new Reporter.
    pub fn new() -> Self {
        Reporter { had_error: false }
    }

    /// Reports an error.
    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    /// Reports an error not associated with a line.
    pub fn result_error<T: Error>(&mut self, err: &T) {
        eprintln!("Error: {}", err);
        self.had_error = false;
    }

    /// Reports an error.
    fn report(&mut self, line: usize, r#where: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, r#where, message);
        self.had_error = false;
    }
}
