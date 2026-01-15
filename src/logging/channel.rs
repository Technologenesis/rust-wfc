use tokio::sync::mpsc::Sender;

use async_trait::async_trait;

use super::LogLevel;
use super::Logger;
use super::LoggerImpl;

// LoggingChannel wraps a Logger and can be used to send log messages to the wrapped
// logger from multiple threads.
#[derive(Clone)]
pub struct LoggingChannel {
    sender: Sender<LoggingChannelMessage>,
}

pub struct LoggingChannelMessage {
    level: LogLevel,
    message: String,
}

pub struct ChannelLoggerImpl {
    sender: Sender<LoggingChannelMessage>,
}

#[async_trait]
impl LoggerImpl for ChannelLoggerImpl {
    async fn log(&mut self, level: LogLevel, message: String) {
        self.sender.send(LoggingChannelMessage { level, message }).await.unwrap();
    }
}

impl LoggingChannel {
    pub fn new<L: LoggerImpl + Send + Sync + 'static>(mut underlying_logger_impl: Logger<L>) -> Self {
        let (sender, mut receiver) = tokio::sync::mpsc::channel::<LoggingChannelMessage>(100);

        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                underlying_logger_impl.log(message.level, message.message).await;
            }
        });

        Self { sender }
    }

    pub fn logger(&self) -> Logger<ChannelLoggerImpl> {
        Logger::from(ChannelLoggerImpl { sender: self.sender.clone() })
    }
}