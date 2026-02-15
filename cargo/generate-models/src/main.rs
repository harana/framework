mod generators;
mod models;
mod parser;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    GenerateModels(GenerateModels),
}

#[derive(clap::Args)]
#[command(author, version, about = "Generate Python and Rust models from YAML schemas", long_about = None)]
struct GenerateModels {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        #[arg(short, long, default_value = "schema")]
        schema_dir: PathBuf,

        #[arg(short, long, default_value = "models")]
        rust_output: PathBuf,

        #[arg(short, long)]
        python_output: Option<PathBuf>,

        #[arg(long)]
        rust_only: bool,

        #[arg(long)]
        python_only: bool,
    },
}

fn main() -> Result<()> {
    let CargoCli::GenerateModels(args) = CargoCli::parse();

    match args.command {
        Commands::Generate {
            schema_dir,
            rust_output,
            python_output,
            rust_only,
            python_only,
        } => {
            println!("🔍 Scanning schema directory: {}", schema_dir.display());

            // Parse all schema files
            let models = parser::parse_schema_directory(&schema_dir)?;
            println!(
                "✅ Found {} models across {} files",
                models.iter().map(|f| f.models.len()).sum::<usize>(),
                models.len()
            );

            // Generate Rust code
            if !python_only {
                println!("🦀 Generating Rust code...");
                generators::rust::generate(&models, &rust_output)?;
                println!("✅ Rust code generated in: {}", rust_output.display());
            }

            // Generate Python code (only when --python-output is specified)
            if let Some(ref py_out) = python_output {
                if !rust_only {
                    println!("🐍 Generating Python code...");
                    generators::python::generate(&models, py_out)?;
                    println!("✅ Python code generated in: {}", py_out.display());
                }
            }

            println!("🎉 Code generation complete!");
            Ok(())
        }
    }
}
