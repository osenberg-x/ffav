pub mod error;
pub use error::EncodeError;

pub mod encoder;
pub use encoder::{
    Encoder,
    EncoderInstance,
    EncoderRegistry
};