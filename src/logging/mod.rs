pub mod basic;
pub mod format;
pub mod channel;
pub mod noop;

use std::fmt::Display;
use std::ops::DerefMut;
use std::collections::HashMap;
use std::io::Write;
use async_trait::async_trait;

use crate::logging::{
    format::FormatLogger,
    basic::BasicLogger,
    noop::NoopLogger,
};


pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Fatal
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Debug => write!(f, "DEBUG"),
            Self::Info => write!(f, "INFO"),
            Self::Warning => write!(f, "WARNING"),
            Self::Error => write!(f, "ERROR"),
            Self::Fatal => write!(f, "FATAL"),
        }
    }
}

#[async_trait]
pub trait LoggerImpl: Send + Sync {
    async fn log(&mut self, level: LogLevel, message: String);
}

pub struct Logger<L: LoggerImpl>(L);

pub type DynLogger = Logger<Box<dyn LoggerImpl>>;

impl<L: LoggerImpl> From<L> for Logger<L> {
    fn from(logger: L) -> Self {
        Self(logger)
    }
}

#[async_trait]
impl <Wrapper: DerefMut + Send + Sync> LoggerImpl for Wrapper
where Wrapper::Target: LoggerImpl
{
    async fn log(&mut self, level: LogLevel, message: String) {
        (**self).log(level, message).await;
    }
}

impl<L: LoggerImpl + 'static> Logger<L> {
    pub async fn log(&mut self, level: LogLevel, message: String) {
        self.0.log(level, message).await;
    }

    pub async fn debug(&mut self, message: String) {
        self.log(LogLevel::Debug, message).await;
    }

    pub async fn info(&mut self, message: String) {
        self.log(LogLevel::Info, message).await;
    }

    pub async fn warn(&mut self, message: String) {
        self.log(LogLevel::Warning, message).await;
    }

    pub async fn error(&mut self, message: String) {
        self.log(LogLevel::Error, message).await;
    }

    pub async fn fatal(&mut self, message: String) {
        self.log(LogLevel::Fatal, message).await;
    }

    pub fn underlying(self) -> L {
        self.0
    }

    pub fn basic<Out: Write + Send + Sync>(out: Out) -> BasicLogger<Out> {
        BasicLogger::new(out)
    }

    pub fn noop() -> NoopLogger {
        NoopLogger::new()
    }

    pub fn to_dyn(self) -> DynLogger {
        Logger(Box::new(self.0))
    }

    pub fn format(self, format: String, kwargs: HashMap<String, String>) -> FormatLogger<L> {
        FormatLogger::<L>::new(self.0, format, kwargs)
    }
}

impl<L: LoggerImpl> Clone for Logger<L>
where L: Clone {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}