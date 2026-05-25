use std::io::{self, Read, Write};
use is_terminal::IsTerminal;

pub struct IOStreams {
    pub in_out: Box<dyn Read + Send + Sync>,
    pub out: Box<dyn Write + Send + Sync>,
    pub err: Box<dyn Write + Send + Sync>,
    pub is_stdout_tty: bool,
    pub is_stderr_tty: bool,
    pub is_stdin_tty: bool,
    pub color_enabled: bool,
    pub progress_indicator_enabled: bool,
}

impl IOStreams {
    pub fn system() -> Self {
        let stdout = io::stdout();
        let stderr = io::stderr();
        let stdin = io::stdin();

        let is_stdout_tty = stdout.is_terminal();
        let is_stderr_tty = stderr.is_terminal();
        let is_stdin_tty = stdin.is_terminal();

        // Basic color detection: if it's a TTY, enable color by default
        // In a full implementation, we'd check NO_COLOR, CLICOLOR, and config.
        let color_enabled = is_stdout_tty;

        Self {
            in_out: Box::new(stdin),
            out: Box::new(stdout),
            err: Box::new(stderr),
            is_stdout_tty,
            is_stderr_tty,
            is_stdin_tty,
            color_enabled,
            progress_indicator_enabled: is_stderr_tty,
        }
    }

    pub fn set_color_enabled(&mut self, enabled: bool) {
        self.color_enabled = enabled;
    }

    pub fn set_progress_indicator_enabled(&mut self, enabled: bool) {
        self.progress_indicator_enabled = enabled;
    }
}
