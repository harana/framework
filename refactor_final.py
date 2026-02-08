#!/usr/bin/env python3
"""
Final refactor script to remove ALL doc comments on structs and enums.
This includes:
1. Top-level /// comments before struct/enum declarations
2. Field comments (already done, but included for completeness)
3. Enum variant comments (already done, but included for completeness)

Preserves:
- Module-level doc comments (//!)
- Function/method doc comments (/// before fn, impl, trait)
"""

import re
import os
import sys
from pathlib import Path
from typing import Optional, Tuple

def remove_struct_enum_comments(content: str) -> str:
    """
    Remove all /// doc comments that appear before struct or enum declarations,
    as well as comments before their fields/variants.
    """
    lines = content.split('\n')
    result_lines = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        # Check if this line is a doc comment
        if re.match(r'^\s*///', line):
            # Collect all consecutive doc comments
            doc_start = i
            doc_lines = []
            j = i
            while j < len(lines) and re.match(r'^\s*///', lines[j]):
                doc_lines.append(lines[j])
                j += 1
            
            # Look ahead past attributes and empty lines to find what this comments
            k = j
            while k < len(lines):
                next_line = lines[k].strip()
                # Skip empty lines and attributes
                if next_line == '' or next_line.startswith('#['):
                    k += 1
                    continue
                break
            
            # Check what follows the doc comments
            should_remove = False
            if k < len(lines):
                next_line = lines[k].strip()
                
                # Remove if followed by struct or enum declaration
                if re.match(r'^(pub(\s*\([^)]+\))?\s+)?(struct|enum)\s+\w+', next_line):
                    should_remove = True
                
                # Remove if it's a field (inside struct/enum body)
                # Field pattern: optional pub, identifier : type
                elif re.match(r'^(pub(\s*\([^)]+\))?\s+)?\w+\s*:', next_line):
                    should_remove = True
                
                # Remove if it's an enum variant
                # Variant pattern: UpperCamelCase optionally followed by (, {, or ,
                elif re.match(r'^[A-Z]\w*(\s*\(|\s*\{|,|$)', next_line):
                    should_remove = True
            
            if should_remove:
                # Skip all the doc comment lines
                i = j
                continue
        
        result_lines.append(line)
        i += 1
    
    return '\n'.join(result_lines)

def process_file(file_path: str) -> bool:
    """
    Process a single Rust file.
    Returns True if file was modified.
    """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        if not content.strip():
            return False
        
        original_content = content
        
        # Remove struct/enum comments
        content_cleaned = remove_struct_enum_comments(content)
        
        if content_cleaned != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content_cleaned)
            return True
        
        return False
        
    except Exception as e:
        print(f"  ✗ ERROR processing {file_path}: {e}")
        import traceback
        traceback.print_exc()
        return False

def main():
    if len(sys.argv) < 2:
        print("Usage: refactor_final.py <directory>")
        sys.exit(1)
    
    root_dir = sys.argv[1]
    
    # Find all .rs files
    rust_files = []
    for root, dirs, files in os.walk(root_dir):
        # Skip target directories
        if 'target' in root.split(os.sep):
            continue
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                rust_files.append(file_path)
    
    print(f"Found {len(rust_files)} Rust files\n")
    print("Removing all struct and enum doc comments...\n")
    
    modified_count = 0
    for file_path in sorted(rust_files):
        rel_path = os.path.relpath(file_path, root_dir)
        
        if process_file(file_path):
            print(f"✓ Modified: {rel_path}")
            modified_count += 1
    
    print(f"\n{'='*60}")
    print(f"Refactoring complete!")
    print(f"Modified {modified_count} out of {len(rust_files)} files")
    print(f"{'='*60}")

if __name__ == '__main__':
    main()
