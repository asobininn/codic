//! # Codic API binding

mod app;
mod config;
pub mod error;
pub mod clap_params;

pub use app::App;
pub use config::Config;
pub use error::Error;
