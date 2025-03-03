// src/lib.rs
pub mod error;
pub mod python;
pub mod venv;
pub mod config;
pub mod download;
pub mod compress;
pub mod modes;
pub mod utils;
pub mod pip;

// Re-export for convenience
pub use error::PyroError;
pub use python::PythonManager;
pub use venv::VirtualEnvManager;
pub use compress::{compress_project, decompress_project};
pub use pip::{install_package, install_requirements};