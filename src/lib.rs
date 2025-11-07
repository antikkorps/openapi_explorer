// Library module to expose internal modules for testing

pub mod app;
pub mod events;
pub mod indexer;
pub mod parser;

// Re-export commonly used types
pub use app::App;
pub use indexer::{FieldData, FieldIndex};
pub use parser::{OpenApiSpec, Schema};
