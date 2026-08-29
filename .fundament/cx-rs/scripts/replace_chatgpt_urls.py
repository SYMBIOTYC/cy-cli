#!/usr/bin/env python3
"""Replace chatgpt.com/backend-api URLs with CY provider URL."""
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

CY_URL = "https://cy.symbiotyc.workers.dev/v1"

# URL replacements
URL_REPLACEMENTS = [
    (r'https://chatgpt\.com/backend-api/cx', CY_URL),
    (r'https://chatgpt\.com/backend-api', CY_URL),
    (r'wss://chatgpt\.com/backend-api/cx', CY_URL.replace('https', 'wss')),
    (r'wss://chatgpt\.com/backend-api', CY_URL.replace('https', 'wss')),
    (r'http://chatgpt\.com/backend-api', CY_URL.replace('https', 'http')),
]

def replace_content():
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
            for pattern, replacement in URL_REPLACEMENTS:
                content = re.sub(pattern, replacement, content)
            
            if content != original:
                file_path.write_text(content, encoding='utf-8')
                print(f"[URL] {file_path}")

def main():
    print("Replacing chatgpt.com URLs with CY provider...")
    replace_content()
    print("Done.")

if __name__ == "__main__":
    main()