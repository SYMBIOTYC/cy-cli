#!/usr/bin/env python3

import os
import re
import shutil
from pathlib import Path

# Base path to the repo root
REPO_ROOT = Path(__file__).resolve().parent.parent.parent

# Skip these paths entirely
SKIP_PATHS = {
    '.git',
    'target',
    '.cargo-home-debug',
    'scripts',
}

# Content replacements: (pattern, replacement)
CONTENT_REPLACEMENTS = [
    # Brand names first
    (r'\bOpenAI\b', 'oi'),
    (r'\bChatGPT\b', 'gt'),
    # Module prefixes
    (r'\bcodex\b', 'cx'),
    (r'\bCodex\b', 'CX'),
    # Crate/package names
    (r'codex-rs', 'cx-rs'),
    (r'codex-', 'cx-'),
    # Module paths
    (r'codex_', 'cx_'),
]

# Directory renames: old_name -> new_name
DIR_RENAMES = {
    'codex-rs': 'cx-rs',
}

def should_skip(path: Path) -> bool:
    for skip in SKIP_PATHS:
        if skip in path.parts:
            return True
    return False

def rename_dirs():
    """Rename directories that need renaming."""
    for root, dirs, files in os.walk(REPO_ROOT, topdown=True):
        root_path = Path(root)
        if should_skip(root_path):
            continue
        
        # Filter out dirs we should skip
        dirs[:] = [d for d in dirs if not should_skip(root_path / d)]
        
        for d in dirs:
            if d in DIR_RENAMES:
                old_path = root_path / d
                new_path = root_path / DIR_RENAMES[d]
                if not new_path.exists():
                    print(f"[RENAME DIR] {old_path} -> {new_path}")
                    shutil.move(str(old_path), str(new_path))

def rename_files():
    """Rename files that contain 'codex' in their name."""
    for root, dirs, files in os.walk(REPO_ROOT, topdown=True):
        root_path = Path(root)
        if should_skip(root_path):
            continue
        
        dirs[:] = [d for d in dirs if not should_skip(root_path / d)]
        
        for f in files:
            old_path = root_path / f
            new_name = f
            for old, new in DIR_RENAMES.items():
                new_name = new_name.replace(old, new)
            # Replace codex_ -> cx_ in filenames
            new_name = re.sub(r'codex_', 'cx_', new_name)
            new_name = re.sub(r'Codex_', 'CX_', new_name)
            new_name = re.sub(r'codex-', 'cx-', new_name)
            
            if new_name != f:
                new_path = root_path / new_name
                if not new_path.exists():
                    print(f"[RENAME FILE] {old_path} -> {new_path}")
                    shutil.move(str(old_path), str(new_path))

def replace_content():
    """Replace content in all text files."""
    text_extensions = {'.rs', '.toml', '.json', '.py', '.md', '.yml', '.yaml', '.sh', '.ps1', '.bazel', '.BUILD'}
    
    for root, dirs, files in os.walk(REPO_ROOT, topdown=True):
        root_path = Path(root)
        if should_skip(root_path):
            continue
        
        dirs[:] = [d for d in dirs if not should_skip(root_path / d)]
        
        for f in files:
            file_path = root_path / f
            if file_path.suffix not in text_extensions:
                continue
            
            try:
                content = file_path.read_text(encoding='utf-8')
            except (UnicodeDecodeError, PermissionError):
                continue
            
            original = content
            for pattern, replacement in CONTENT_REPLACEMENTS:
                content = re.sub(pattern, replacement, content)
            
            if content != original:
                file_path.write_text(content, encoding='utf-8')
                print(f"[REBRAND] {file_path}")

def main():
    print("SYMBIOTYC CY-CLI full rebranding")
    print("=" * 60)
    
    print("\n[1/3] Renaming directories...")
    rename_dirs()
    
    print("\n[2/3] Renaming files...")
    rename_files()
    
    print("\n[3/3] Replacing content...")
    replace_content()
    
    print("\n" + "=" * 60)
    print("Rebranding complete.")

if __name__ == "__main__":
    main()
