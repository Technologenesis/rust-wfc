use super::Logger;
use super::LoggerImpl;
use super::LogLevel;

use async_trait::async_trait;

pub type NoopLogger = Logger<NoopLoggerImpl>;

impl NoopLogger {
    pub fn new() -> Self {
        Self::from(NoopLoggerImpl)
    }
}

pub struct NoopLoggerImpl;

#[async_trait]
impl LoggerImpl for NoopLoggerImpl {
    async fn log(&mut self, _level: LogLevel, _message: String) {
        // do nothing
    }
}