#!/usr/bin/env python3
"""Replace Codex/codex references in Rust source files with cx/CX equivalents."""
import os
import re
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
SKIP_PATHS = {'.git', 'target', '.cargo-home-debug', '.cargo-home-*', 'scripts', '.cargo', '.config'}

def should_skip(path: Path) -> bool:
    for skip in SKIP_PATHS:
        if skip in path.parts or any(p.startswith(skip.rstrip('*')) for p in path.parts):
            return True
    return False

# Replacements for identifiers and constants
# Order matters - more specific first
IDENT_REPLACEMENTS = [
    # Types and structs
    (r'\bCodexClient\b', 'CxClient'),
    (r'\bCodexErr\b', 'CxErr'),
    (r'\bCodexErrKind\b', 'CxErrKind'),
    (r'\bCodexRuntimeMetadata\b', 'CxRuntimeMetadata'),
    (r'\bTurnCodexError\b', 'TurnCxError'),
    (r'\bCodexTurnSteerEvent\b', 'CxTurnSteerEvent'),
    (r'\bCodexCompactionEvent\b', 'CxCompactionEvent'),
    (r'\bCodexGoalEvent\b', 'CxGoalEvent'),
    
    # Constants and env var names (be careful with these)
    (r'\bMINIMUM_SUPPORTED_CODEX_VERSION\b', 'MINIMUM_SUPPORTED_CX_VERSION'),
    (r'\bCODEX_STARTING_DIFF\b', 'CX_STARTING_DIFF'),
    (r'\bCODEX_TEST_RELATIVE_TMPDIR\b', 'CX_TEST_RELATIVE_TMPDIR'),
    (r'\bCODEX_ANALYTICS_EVENTS_CAPTURE_FILE\b', 'CX_ANALYTICS_EVENTS_CAPTURE_FILE'),
    (r'\bCODEX_APP_SERVER_TEST_USER_CONFIG_FILE\b', 'CX_APP_SERVER_TEST_USER_CONFIG_FILE'),
    (r'\bCODEX_EXEC_SERVER_NOISE_AUTH_TOKEN_ENV_VAR\b', 'CX_EXEC_SERVER_NOISE_AUTH_TOKEN_ENV_VAR'),
    
    # Other CODEX_* constants
    (r'\bCODEX_([A-Z_]+)\b', r'CX_\1'),  # Generic CODEX_* -> CX_*
    
    # Lowercase codex in identifiers
    (r'\bcodex\b', 'cx'),
    (r'\bCodex\b', 'CX'),  # Capitalized
]

# Comment and string replacements (be very careful here)
COMMENT_REPLACEMENTS = [
    # Don't modify string literals or actual code, just comments
    # This is tricky - we'll do a simple approach and then verify
]

def replace_in_rs_file(file_path: Path):
    """Replace Codex references in a single Rust file."""
    try:
        content = file_path.read_text(encoding='utf-8')
    except (UnicodeDecodeError, PermissionError):
        return False
    
    original = content
    
    # Apply identifier replacements
    for pattern, replacement in IDENT_REPLACEMENTS:
        content = re.sub(pattern, replacement, content)
    
    # For safety, let's not modify comments/strings automatically for now
    # We'll just do the identifier replacements
    
    if content != original:
        file_path.write_text(content, encoding='utf-8')
        print(f"[RS] {file_path}")
        return True
    return False

def main():
    print("Replacing Codex/codex references in Rust source files...")
    count = 0
    
    for root, dirs, files in os.walk(REPO_ROOT, topdown=True):
        root_path = Path(root)
        if should_skip(root_path):
            continue
        
        dirs[:] = [d for d in dirs if not should_skip(root_path / d)]
        
        for f in files:
            if f.endswith('.rs'):
                file_path = root_path / f
                if replace_in_rs_file(file_path):
                    count += 1
    
    print(f"Modified {count} Rust files.")

if __name__ == "__main__":
    main()