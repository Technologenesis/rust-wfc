use std::io::Write;

use async_trait::async_trait;

use super::Logger;
use super::LoggerImpl;
use super::LogLevel;

#[derive(Clone)]
pub struct BasicLoggerImpl<Out: Write + Send + Sync> {
    out: Out,
}

impl<Out: Write + Send + Sync> BasicLoggerImpl<Out> {
    pub fn new(out: Out) -> Self {
        Self { out }
    }
}

#[async_trait]
impl<Out: Write + Send + Sync> LoggerImpl for BasicLoggerImpl<Out> {
    async fn log(&mut self, level: LogLevel, message: String) {
        match level {
            LogLevel::Debug => self.out.write_all(format!("{}\n", message).as_bytes()).unwrap(),
            LogLevel::Info => self.out.write_all(format!("{}\n", message).as_bytes()).unwrap(),
            LogLevel::Warning => self.out.write_all(format!("{}\n", message).as_bytes()).unwrap(),
            LogLevel::Error => self.out.write_all(format!("{}\n", message).as_bytes()).unwrap(),
            LogLevel::Fatal => self.out.write_all(format!("{}\n", message).as_bytes()).unwrap(),
        }
    }
}

pub type BasicLogger<Out> = Logger<BasicLoggerImpl<Out>>;

impl<Out: Write + Send + Sync> BasicLogger<Out> {
    pub fn new(out: Out) -> Self {
        Self::from(BasicLoggerImpl::new(out))
    }
}