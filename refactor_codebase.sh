#!/bin/bash

# Script to refactor codebase:
# 1. Remove field comments (/// comments before struct fields)
# 2. Move inline tests to separate tests.rs files

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "Starting codebase refactoring..."
echo "Total Rust files: $(find . -name '*.rs' -type f | wc -l)"

# Create a Python script to handle the refactoring
cat > /tmp/refactor_rust.py << 'PYTHON_SCRIPT'
#!/usr/bin/env python3
import re
import os
import sys
from pathlib import Path

def remove_field_comments(content):
    """Remove /// comments that appear immediately before struct fields"""
    lines = content.split('\n')
    result_lines = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        
        # Check if this is a doc comment line
        if re.match(r'^\s*///.*$', line):
            # Look ahead to see if the next non-empty, non-comment line is a field
            j = i + 1
            doc_comment_block = [line]
            
            # Collect all consecutive doc comments
            while j < len(lines) and re.match(r'^\s*///.*$', lines[j]):
                doc_comment_block.append(lines[j])
                j += 1
            
            # Skip empty lines
            while j < len(lines) and lines[j].strip() == '':
                j += 1
            
            # Check if next line is a field (has pub/private field with :)
            if j < len(lines):
                next_line = lines[j].strip()
                # Field pattern: optional pub, identifier, colon
                if re.match(r'^(pub(\s+\([^)]+\))?\s+)?\w+\s*:', next_line):
                    # This is a field comment, skip it
                    i = j
                    result_lines.append(lines[j])
                    i += 1
                    continue
        
        result_lines.append(line)
        i += 1
    
    return '\n'.join(result_lines)

def extract_test_module(content, file_path):
    """Extract test module from content and return (cleaned_content, test_content)"""
    # Match #[cfg(test)] mod tests { ... } or mod tests; declaration
    
    # First check if there's already a separate tests.rs or tests module declaration
    if re.search(r'#\[cfg\(test\)\]\s*mod\s+tests\s*;', content):
        # Tests are already in separate file
        return content, None
    
    # Find inline test module
    match = re.search(
        r'(#\[cfg\(test\)\])\s*\n\s*mod\s+tests\s*\{',
        content,
        re.MULTILINE
    )
    
    if not match:
        # Try pub(crate) mod tests
        match = re.search(
            r'(#\[cfg\(test\)\])\s*\n\s*pub\s*\(\s*crate\s*\)\s*mod\s+tests\s*\{',
            content,
            re.MULTILINE
        )
    
    if not match:
        return content, None
    
    start_pos = match.start()
    
    # Find matching closing brace
    brace_count = 0
    i = content.index('{', start_pos)
    test_start = i
    
    for j in range(i, len(content)):
        if content[j] == '{':
            brace_count += 1
        elif content[j] == '}':
            brace_count -= 1
            if brace_count == 0:
                # Found the end
                test_module_content = content[i+1:j]
                
                # Remove the test module from original content
                cleaned_content = content[:start_pos].rstrip() + '\n' + content[j+1:]
                
                # Clean up any extra blank lines
                cleaned_content = re.sub(r'\n\n\n+', '\n\n', cleaned_content)
                
                # Format test content for separate file
                test_file_content = f"""#[cfg(test)]
mod tests {{
{test_module_content}
}}
"""
                
                return cleaned_content.strip() + '\n', test_file_content
    
    return content, None

def process_file(file_path):
    """Process a single Rust file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        modified = False
        
        # Step 1: Remove field comments
        content_no_comments = remove_field_comments(content)
        if content_no_comments != content:
            modified = True
            content = content_no_comments
        
        # Step 2: Extract tests if this is not already a tests.rs file
        if not file_path.endswith('/tests.rs'):
            cleaned_content, test_content = extract_test_module(content, file_path)
            
            if test_content:
                # Write tests to separate file
                test_file_path = os.path.join(os.path.dirname(file_path), 'tests.rs')
                
                # Check if tests.rs already exists
                if os.path.exists(test_file_path):
                    print(f"  WARNING: {test_file_path} already exists, skipping test extraction")
                else:
                    with open(test_file_path, 'w', encoding='utf-8') as f:
                        f.write(test_content)
                    print(f"  ✓ Created {test_file_path}")
                    
                    # Update main file to reference tests module
                    if not re.search(r'#\[cfg\(test\)\]\s*mod\s+tests\s*;', cleaned_content):
                        cleaned_content = cleaned_content.rstrip() + '\n\n#[cfg(test)]\nmod tests;\n'
                    
                    content = cleaned_content
                    modified = True
        
        # Write back if modified
        if modified:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return True
        
        return False
        
    except Exception as e:
        print(f"  ERROR processing {file_path}: {e}")
        return False

def main():
    if len(sys.argv) < 2:
        print("Usage: refactor_rust.py <directory>")
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
                rust_files.append(os.path.join(root, file))
    
    print(f"Found {len(rust_files)} Rust files")
    
    modified_count = 0
    for file_path in rust_files:
        rel_path = os.path.relpath(file_path, root_dir)
        if process_file(file_path):
            print(f"✓ Modified: {rel_path}")
            modified_count += 1
    
    print(f"\nRefactoring complete!")
    print(f"Modified {modified_count} files")

if __name__ == '__main__':
    main()
PYTHON_SCRIPT

# Run the Python script
python3 /tmp/refactor_rust.py "$SCRIPT_DIR"

echo "Refactoring complete!"
