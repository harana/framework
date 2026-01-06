use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use syn::{File, Item, ItemFn};

#[derive(Debug)]
pub struct RustModule {
    pub existing_functions: HashSet<String>,
    pub lib_content: String,
    pub output_content: Option<String>,
}

impl RustModule {
    pub fn analyze(crate_dir: &Path) -> Result<Self> {
        let lib_path = crate_dir.join("src/lib.rs");
        let output_path = crate_dir.join("src/output.rs");

        // Read lib.rs
        let lib_content = fs::read_to_string(&lib_path)
            .with_context(|| format!("Failed to read {}", lib_path.display()))?;

        // Parse lib.rs to find existing functions
        let syntax_tree: File = syn::parse_file(&lib_content)
            .with_context(|| format!("Failed to parse {}", lib_path.display()))?;

        let mut existing_functions = HashSet::new();

        for item in syntax_tree.items {
            if let Item::Fn(ItemFn { sig, .. }) = item {
                existing_functions.insert(sig.ident.to_string());
            }
        }

        // Read output.rs if it exists
        let output_content = if output_path.exists() {
            Some(fs::read_to_string(&output_path)
                .with_context(|| format!("Failed to read {}", output_path.display()))?)
        } else {
            None
        };

        Ok(RustModule {
            existing_functions,
            lib_content,
            output_content,
        })
    }

    pub fn has_function(&self, function_name: &str) -> bool {
        self.existing_functions.contains(function_name)
    }

    pub fn has_output_struct(&self, struct_name: &str) -> bool {
        if let Some(ref content) = self.output_content {
            content.contains(&format!("pub struct {}", struct_name))
        } else {
            false
        }
    }
}
