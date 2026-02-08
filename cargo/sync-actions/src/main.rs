mod analyzer;
mod generator;
mod parser;
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
    Sync {
        #[arg(short, long, default_value = "core/schema/actions")]
        schema_dir: PathBuf,

        #[arg(short = 'a', long, default_value = "crates/actions")]
        actions_dir: PathBuf,

        #[arg(short, long)]
        dry_run: bool,

        #[arg(short = 'm', long)]
        module: Option<String>,

        #[arg(short, long)]
        force: bool,

        #[arg(short, long)]
        replace: bool,
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
            force,
            replace,
        } => {
            println!("🔍 Scanning schema directory: {}", schema_dir.display());
            println!("📁 Actions directory: {}", actions_dir.display());

            if dry_run {
                println!("🔍 DRY RUN MODE - No files will be modified");
            }

            if force {
                println!("⚠️  FORCE MODE - Actions not in YAML will be removed");
            }

            if replace {
                println!("🔄 REPLACE MODE - Files will be completely regenerated from YAML");
            }

            // Parse all schema files
            let schemas = parser::parse_action_schemas(&schema_dir, module.as_deref())?;
            println!("✅ Found {} schema files", schemas.len());

            // Sync each schema with its corresponding Rust crate
            let mut total_added = 0;
            let mut total_removed = 0;
            for schema in schemas {
                let (added, removed) = syncer::sync_schema(&schema, &actions_dir, dry_run, force, replace)?;
                total_added += added;
                total_removed += removed;
            }

            if dry_run {
                println!(
                    "\n🎉 Dry run complete! {} methods would be added, {} would be removed",
                    total_added, total_removed
                );
            } else {
                println!(
                    "\n🎉 Sync complete! {} methods added, {} removed",
                    total_added, total_removed
                );
            }

            Ok(())
        }
    }
}
