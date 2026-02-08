#!/usr/bin/env python3
"""
Refactor Rust codebase to:
1. Remove all field comments (/// comments before struct fields)
2. Move inline test modules to separate tests.rs files
"""

import re
import os
import sys
from pathlib import Path
from typing import Optional, Tuple

def remove_field_comments_v2(content: str) -> str:
    """
    Remove /// comments that appear before struct fields.
    Handles cases where attributes like #[serde(...)] are between comment and field.
    """
    lines = content.split('\n')
    result_lines = []
    i = 0
    in_struct = False
    brace_depth = 0
    
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        # Track if we're inside a struct
        if re.search(r'\bstruct\s+\w+', line):
            in_struct = True
        
        # Track braces
        brace_depth += line.count('{') - line.count('}')
        
        if brace_depth == 0 and in_struct:
            in_struct = False
        
        # Check if this line is a doc comment
        if re.match(r'^\s*///', line):
            # Collect all consecutive doc comments
            doc_lines = []
            j = i
            while j < len(lines) and re.match(r'^\s*///', lines[j]):
                doc_lines.append((j, lines[j]))
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
            
            # Check if we found a field definition
            is_field = False
            if k < len(lines):
                next_line = lines[k].strip()
                # Field pattern: optional pub with visibility, then identifier : type
                # Also check if we're likely in a struct (brace_depth > 0 after struct keyword)
                if in_struct or brace_depth > 0:
                    # Match field patterns like:
                    # pub field: Type,
                    # pub(crate) field: Type,
                    # field: Type,
                    if re.match(r'^(pub(\s*\([^)]+\))?\s+)?\w+\s*:', next_line):
                        is_field = True
            
            if is_field:
                # Skip the doc comments (remove them)
                i = j
                continue
        
        result_lines.append(line)
        i += 1
    
    return '\n'.join(result_lines)

def extract_test_module_v2(content: str, file_path: str) -> Tuple[str, Optional[str]]:
    """
    Extract inline test module from content.
    Returns (cleaned_content, test_content) or (original_content, None)
    """
    # First check if there's already a separate tests module declaration
    if re.search(r'#\[cfg\(test\)\]\s*\n\s*mod\s+tests\s*;', content, re.MULTILINE):
        return content, None
    
    # Find inline test module - match various patterns
    patterns = [
        r'#\[cfg\(test\)\]\s*\n\s*mod\s+tests\s*\{',
        r'#\[cfg\(test\)\]\s*\n\s*pub\s*\(\s*crate\s*\)\s*mod\s+tests\s*\{',
    ]
    
    match = None
    for pattern in patterns:
        match = re.search(pattern, content, re.MULTILINE)
        if match:
            break
    
    if not match:
        return content, None
    
    # Find the opening brace of the module
    start_pos = match.start()
    brace_start = content.index('{', match.start())
    
    # Find matching closing brace
    brace_count = 0
    i = brace_start
    
    for j in range(i, len(content)):
        if content[j] == '{':
            brace_count += 1
        elif content[j] == '}':
            brace_count -= 1
            if brace_count == 0:
                # Found the end of the module
                module_content = content[brace_start+1:j]
                
                # Extract just the test functions/content (without outer mod tests {})
                cleaned_module_content = module_content.strip()
                
                # Remove the entire test module from original content
                cleaned_content = content[:start_pos].rstrip() + '\n' + content[j+1:].lstrip()
                
                # Clean up excessive blank lines
                cleaned_content = re.sub(r'\n\n\n+', '\n\n', cleaned_content)
                cleaned_content = cleaned_content.strip() + '\n'
                
                # Add mod tests; declaration at the end if not already there
                if not re.search(r'#\[cfg\(test\)\]\s*\n\s*mod\s+tests\s*;', cleaned_content):
                    cleaned_content += '\n#[cfg(test)]\nmod tests;\n'
                
                # Format test content for separate file
                test_file_content = f"""#[cfg(test)]
mod tests {{
{cleaned_module_content}
}}
"""
                
                return cleaned_content, test_file_content
    
    return content, None

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
        modified = False
        
        # Step 1: Remove field comments
        content_no_comments = remove_field_comments_v2(content)
        if content_no_comments != content:
            modified = True
            content = content_no_comments
            print(f"  • Removed field comments")
        
        # Step 2: Extract tests if this is not already a tests.rs file
        if not file_path.endswith('/tests.rs') and not file_path.endswith('\\tests.rs'):
            cleaned_content, test_content = extract_test_module_v2(content, file_path)
            
            if test_content:
                # Determine test file path
                test_file_path = os.path.join(os.path.dirname(file_path), 'tests.rs')
                
                # Check if tests.rs already exists
                if os.path.exists(test_file_path):
                    print(f"  ⚠ Tests file already exists: {test_file_path}")
                else:
                    with open(test_file_path, 'w', encoding='utf-8') as f:
                        f.write(test_content)
                    print(f"  ✓ Created tests file: {os.path.basename(test_file_path)}")
                    content = cleaned_content
                    modified = True
        
        # Write back if modified
        if modified:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return True
        
        return False
        
    except Exception as e:
        print(f"  ✗ ERROR: {e}")
        import traceback
        traceback.print_exc()
        return False

def main():
    if len(sys.argv) < 2:
        print("Usage: refactor_v2.py <directory>")
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
    
    modified_count = 0
    for file_path in sorted(rust_files):
        rel_path = os.path.relpath(file_path, root_dir)
        
        # Show progress
        sys.stdout.write(f"Processing: {rel_path}")
        sys.stdout.flush()
        
        if process_file(file_path):
            print()  # New line after processing messages
            modified_count += 1
        else:
            print(" (no changes)")
    
    print(f"\n{'='*60}")
    print(f"Refactoring complete!")
    print(f"Modified {modified_count} out of {len(rust_files)} files")
    print(f"{'='*60}")

if __name__ == '__main__':
    main()
