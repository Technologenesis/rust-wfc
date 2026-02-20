use std::error::Error as StdError;

pub type MessageReceiverComponent = Box<dyn MessageReceiverTrait<Error = Box<dyn StdError>>>;

pub trait MessageReceiverTrait {
    type Error;

    fn send_message(&self, message: String) -> Result<(), Self::Error>;
}

impl<E: StdError + 'static> MessageReceiverTrait for Box<dyn MessageReceiverTrait<Error = E>> {
    type Error = Box<dyn StdError>;

    fn send_message(&self, message: String) -> Result<(), Self::Error> {
        (**self).send_message(message).map_err(|e| -> Self::Error { Box::new(e) })
    }
}