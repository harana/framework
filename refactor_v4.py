#!/usr/bin/env python3
"""
Refactoring script v4: Remove top-level doc comments from structs and enums
"""

import os
import re
from pathlib import Path

def remove_struct_enum_comments(content):
    """
    Remove doc comments that appear directly before struct or enum declarations.
    Preserves module-level comments (//!) and keeps attributes (#[...]).
    """
    lines = content.split('\n')
    result = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        
        # Check if this is a doc comment line
        if line.strip().startswith('///'):
            # Look ahead to find what follows the comment block
            j = i
            comment_lines = []
            
            # Collect all consecutive doc comment lines
            while j < len(lines) and lines[j].strip().startswith('///'):
                comment_lines.append(j)
                j += 1
            
            # Skip blank lines and attributes after comments
            while j < len(lines) and (lines[j].strip() == '' or lines[j].strip().startswith('#[')):
                j += 1
            
            # Check if we have a struct or enum declaration
            if j < len(lines):
                next_code_line = lines[j].strip()
                
                # Check for struct or enum declarations
                is_struct_or_enum = False
                if re.match(r'^pub(\(.*?\))?\s+(struct|enum)\s+\w+', next_code_line):
                    is_struct_or_enum = True
                elif re.match(r'^(struct|enum)\s+\w+', next_code_line):
                    is_struct_or_enum = True
                
                if is_struct_or_enum:
                    # Skip the doc comments, move to the line after them
                    i = comment_lines[-1] + 1
                    continue
        
        result.append(line)
        i += 1
    
    return '\n'.join(result)

def process_file(file_path):
    """Process a single Rust file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            original_content = f.read()
        
        modified_content = remove_struct_enum_comments(original_content)
        
        if modified_content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(modified_content)
            
            # Count removed comments
            original_lines = original_content.split('\n')
            modified_lines = modified_content.split('\n')
            removed = len(original_lines) - len(modified_lines)
            
            print(f"✓ Modified: {file_path}")
            print(f"  • Removed {removed} doc comment line(s)")
            return True
        
        return False
    
    except Exception as e:
        print(f"✗ Error processing {file_path}: {e}")
        return False

def main():
    """Main entry point."""
    import sys
    
    if len(sys.argv) < 2:
        print("Usage: python3 refactor_v4.py <directory>")
        sys.exit(1)
    
    root_dir = Path(sys.argv[1]).resolve()
    
    if not root_dir.exists():
        print(f"Error: Directory {root_dir} does not exist")
        sys.exit(1)
    
    print(f"Processing Rust files in {root_dir}...")
    print("=" * 60)
    
    processed = 0
    modified = 0
    
    for rust_file in root_dir.rglob("*.rs"):
        # Skip target directory
        if 'target' in rust_file.parts:
            continue
        
        processed += 1
        if process_file(rust_file):
            modified += 1
    
    print("=" * 60)
    print(f"Complete! Modified {modified} out of {processed} files")

if __name__ == "__main__":
    main()
