// Library module to expose internal modules for testing

pub mod parser;
pub mod indexer;
pub mod app;
pub mod events;

// Re-export commonly used types
pub use parser::{OpenApiSpec, Schema};
pub use indexer::{FieldIndex, FieldData};
pub use app::App;
