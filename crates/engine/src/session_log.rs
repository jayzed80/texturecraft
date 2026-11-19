use chrono::Utc;

use crate::{LogEntry, LogLevel};

/// The current session's log in occurrence order.
#[derive(Debug, Default)]
pub struct SessionLog {
    entries: Vec<LogEntry>,
}

impl SessionLog {
    pub fn push(&mut self, message: String) {
        self.record(LogLevel::Info, message);
    }

    pub fn error(&mut self, message: String) {
        self.record(LogLevel::Error, message);
    }

    pub fn warn(&mut self, message: String) {
        self.record(LogLevel::Warning, message);
    }

    pub fn extend_warnings(&mut self, messages: impl IntoIterator<Item = String>) {
        for message in messages {
            self.warn(message);
        }
    }

    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    pub fn last(&self) -> Option<&String> {
        self.entries.last().map(|entry| &entry.message)
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.entries.iter().map(|entry| &entry.message)
    }

    fn record(&mut self, level: LogLevel, message: String) {
        self.entries.push(LogEntry { time: Utc::now(), level, message });
    }
}

impl Extend<String> for SessionLog {
    fn extend<T: IntoIterator<Item = String>>(&mut self, messages: T) {
        for message in messages {
            self.push(message);
        }
    }
}
