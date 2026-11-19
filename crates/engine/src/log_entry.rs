use chrono::{DateTime, Utc};

use crate::LogLevel;

/// A session message with its occurrence time and severity.
#[derive(Debug)]
pub struct LogEntry {
    pub time: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
}
