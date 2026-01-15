use std::collections::HashMap;

use async_trait::async_trait;

use super::Logger;
use super::LoggerImpl;
use super::LogLevel;

#[derive(Clone)]
pub struct FormatLoggerImpl<L: LoggerImpl> {
    underlying_logger_impl: L,
    kwargs: HashMap<String, String>,
    format: String,
}

impl<L: LoggerImpl> FormatLoggerImpl<L> {
    pub fn new(underlying_logger_impl: L, format: String, kwargs: HashMap<String, String>) -> Self {
        Self {
            underlying_logger_impl,
            format,
            kwargs,
        }
    }

    pub fn set_kwarg(&mut self, key: String, value: String) {
        self.kwargs.insert(key, value);
    }
}

#[async_trait]
impl<L: LoggerImpl + Send + Sync> LoggerImpl for FormatLoggerImpl<L> {
    async fn log(&mut self, level: LogLevel, message: String) {
        let mut formatted = self.format.clone();
        for (key, value) in &self.kwargs {
            formatted = formatted.replace(&format!("{{{}}}", key), &value);
        }
        formatted = formatted.replace("{message}", &message);
        formatted = formatted.replace("{level}", &level.to_string());
        self.underlying_logger_impl.log(level, formatted).await;
    }
}

pub type FormatLogger<L> = Logger<FormatLoggerImpl<L>>;

impl<L: LoggerImpl + Send + Sync + 'static> FormatLogger<L> {
    pub fn new(underlying_logger_impl: L, format: String, kwargs: HashMap<String, String>) -> Self {
        Self::from(FormatLoggerImpl::new(underlying_logger_impl, format, kwargs))
    }
}