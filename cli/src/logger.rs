use std::time::Duration;

pub struct Logger {
    verbose: bool,
}

impl Logger {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    pub fn log(&self, message: &str) {
        if self.verbose {
            eprintln!("{message}");
        }
    }
}

pub fn format_duration(duration: Duration) -> String {
    let duration_ms = duration.as_millis();
    if duration_ms < 1_000 {
        return format!("{duration_ms}ms");
    }

    format!("{:.3}s", duration.as_secs_f64())
}
