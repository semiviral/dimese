#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate tracing;

pub mod crypto;
pub mod message;

#[derive(Debug, serde::Deserialize)]
struct VideoMetadata {
    format: String, // TODO create format type
}
