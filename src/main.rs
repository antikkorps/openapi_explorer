use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod app;
mod events;
mod indexer;
mod parser;
mod ui;

#[derive(Parser)]
#[command(name = "openapi-explorer")]
#[command(about = "TUI OpenAPI Field Explorer - Analyze database fields across API endpoints")]
struct Args {
    /// Path to OpenAPI specification file (JSON or YAML)
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,

    /// Interactive mode - choose file from current directory
    #[arg(short, long)]
    interactive: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.debug {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    log::info!("Starting OpenAPI Field Explorer");
    log::debug!("Loading OpenAPI spec from: {:?}", args.file);

    // Parse OpenAPI specification
    let openapi_spec = parser::parse_openapi_or_default(&args.file).await?;
    log::info!("Successfully parsed OpenAPI specification");

    // Index fields and relationships
    let field_index = indexer::build_field_index(&openapi_spec);
    log::info!(
        "Indexed {} fields across {} schemas",
        field_index.fields.len(),
        field_index.schemas.len()
    );

    // Initialize application state with file path for reload capability
    let mut app = app::App::new(openapi_spec, field_index, args.file);

    // Run the TUI application
    ui::run(&mut app)
        .await
        .map_err(|e| anyhow::anyhow!("UI error: {}", e))?;

    Ok(())
}
