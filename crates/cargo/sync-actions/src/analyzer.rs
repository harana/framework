use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use syn::{File, Item, ItemFn, ItemStruct};

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub start_line: usize,
    #[allow(dead_code)]
    pub end_line: usize,
}

#[derive(Debug, Clone)]
pub struct StructInfo {
    pub name: String,
    #[allow(dead_code)]
    pub start_line: usize,
    #[allow(dead_code)]
    pub end_line: usize,
}

#[derive(Debug)]
pub struct RustModule {
    pub existing_functions: HashSet<String>,
    #[allow(dead_code)]
    pub function_locations: Vec<FunctionInfo>,
    pub lib_content: String,
    pub output_content: Option<String>,
    pub output_structs: Vec<StructInfo>,
}

impl RustModule {
    pub fn analyze(crate_dir: &Path) -> Result<Self> {
        let lib_path = crate_dir.join("src/lib.rs");
        let output_path = crate_dir.join("src/output.rs");

        // Read lib.rs
        let lib_content =
            fs::read_to_string(&lib_path).with_context(|| format!("Failed to read {}", lib_path.display()))?;

        // Parse lib.rs to find existing functions
        let syntax_tree: File =
            syn::parse_file(&lib_content).with_context(|| format!("Failed to parse {}", lib_path.display()))?;

        let mut existing_functions = HashSet::new();
        let mut function_locations = Vec::new();

        for item in syntax_tree.items {
            if let Item::Fn(ItemFn { sig, .. }) = item {
                let function_name = sig.ident.to_string();
                existing_functions.insert(function_name.clone());

                // Store function location (approximate - syn doesn't give exact line numbers easily)
                function_locations.push(FunctionInfo {
                    name: function_name,
                    start_line: 0, // We'll use content-based search
                    end_line: 0,
                });
            }
        }

        // Read output.rs if it exists and parse structs
        let (output_content, output_structs) = if output_path.exists() {
            let content = fs::read_to_string(&output_path)
                .with_context(|| format!("Failed to read {}", output_path.display()))?;

            let syntax_tree: File =
                syn::parse_file(&content).with_context(|| format!("Failed to parse {}", output_path.display()))?;

            let mut structs = Vec::new();
            for item in syntax_tree.items {
                if let Item::Struct(ItemStruct { ident, .. }) = item {
                    structs.push(StructInfo {
                        name: ident.to_string(),
                        start_line: 0,
                        end_line: 0,
                    });
                }
            }

            (Some(content), structs)
        } else {
            (None, Vec::new())
        };

        Ok(RustModule {
            existing_functions,
            function_locations,
            lib_content,
            output_content,
            output_structs,
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
