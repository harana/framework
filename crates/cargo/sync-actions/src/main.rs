mod parser;
mod analyzer;
mod generator;
mod syncer;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    SyncActions(SyncActions),
}

#[derive(clap::Args)]
#[command(author, version, about = "Sync action methods from YAML schemas to Rust crates", long_about = None)]
struct SyncActions {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sync actions from schema files to Rust crates
    Sync {
        /// Path to the schema directory
        #[arg(short, long, default_value = "core/schema/actions")]
        schema_dir: PathBuf,

        /// Path to the actions crates directory
        #[arg(short = 'a', long, default_value = "crates/actions")]
        actions_dir: PathBuf,

        /// Dry run - show what would be changed without making changes
        #[arg(short, long)]
        dry_run: bool,

        /// Only process specific action module (e.g., aws_iam)
        #[arg(short = 'm', long)]
        module: Option<String>,
    },
}

fn main() -> Result<()> {
    let CargoCli::SyncActions(args) = CargoCli::parse();

    match args.command {
        Commands::Sync {
            schema_dir,
            actions_dir,
            dry_run,
            module,
        } => {
            println!("🔍 Scanning schema directory: {}", schema_dir.display());
            println!("📁 Actions directory: {}", actions_dir.display());
            
            if dry_run {
                println!("🔍 DRY RUN MODE - No files will be modified");
            }
            
            // Parse all schema files
            let schemas = parser::parse_action_schemas(&schema_dir, module.as_deref())?;
            println!("✅ Found {} schema files", schemas.len());

            // Sync each schema with its corresponding Rust crate
            let mut total_added = 0;
            for schema in schemas {
                let added = syncer::sync_schema(&schema, &actions_dir, dry_run)?;
                total_added += added;
            }

            if dry_run {
                println!("\n🎉 Dry run complete! {} methods would be added", total_added);
            } else {
                println!("\n🎉 Sync complete! {} methods added", total_added);
            }
            
            Ok(())
        }
    }
}
