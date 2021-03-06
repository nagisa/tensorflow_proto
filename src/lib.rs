//! `tensorflow_proto`
//!
//! This library exposes protocol buffers from Tensorflow in the form of Rust structs, to allow end
//! users to consume and produce them.

#![allow(clippy::large_enum_variant)]
include!(concat!(env!("OUT_DIR"), "/tensorflow_proto_gen.rs"));

/// Error type for tensorflow_proto. This exists to avoid forcing users to explicitly depend on
/// prost to use `into_bytes`.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to encode protobuf to bytes")]
    Encode(#[source] prost::EncodeError),
}

/// Serialize a protobuf message into a vector of bytes.
///
/// # Examples
///
/// ```rust
/// use tensorflow_proto::{tensorflow, into_bytes};
///
/// let config_proto = tensorflow::ConfigProto {
///     gpu_options: Some(tensorflow::GpuOptions {
///         allow_growth: true,
///         ..Default::default()
///     }),
///     ..Default::default()
/// };
/// let bytes = into_bytes(config_proto).unwrap();
/// assert!(!bytes.is_empty());
/// ```
pub fn into_bytes(msg: impl prost::Message) -> Result<Vec<u8>, Error> {
    let mut bytes = vec![];
    msg.encode(&mut bytes).map_err(Error::Encode)?;
    Ok(bytes)
}
