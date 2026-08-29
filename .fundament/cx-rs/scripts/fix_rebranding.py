#!/usr/bin/env python3
"""
Fix remaining rebranding issues:
- Rename openapi-models -> oi-models (user request)
- Replace openai_ env vars with oi_ equivalents
- Replace chatgpt references
"""
import os
import re
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
SKIP_PATHS = {'.git', 'target', '.cargo-home-debug', '.cargo-home-*', 'scripts', '.cargo', '.config'}

def should_skip(path: Path) -> bool:
    for skip in SKIP_PATHS:
        if skip in path.parts or any(p.startswith(skip.rstrip('*')) for p in path.parts):
            return True
    return False

# Content replacements: (pattern, replacement) - only in text files
CONTENT_REPLACEMENTS = [
    # openapi-models -> oi-models (user request)
    (r'openapi-models', 'oi-models'),
    (r'openapi_models', 'oi_models'),
    
    # env var names: openai_ -> oi_
    (r'\bopenai_identity_token_file\b', 'oi_identity_token_file'),
    (r'\bopenai_federation_rule_id\b', 'oi_federation_rule_id'),
    (r'\bOpenAI_Identity_Token_File\b', 'Oi_Identity_Token_File'),
    (r'\bOpenAI_Federation_Rule_Id\b', 'Oi_Federation_Rule_Id'),
    (r'\bOpenAI_Workload_Identity_Context\b', 'Oi_Workload_Identity_Context'),
    
    # chatgpt references
    (r'\bchatgpt_base_url\b', 'gt_base_url'),
    (r'\bcx-chatgpt\b', 'cx-gt'),
    (r'\bcx_chatgpt\b', 'cx_gt'),
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
            for pattern, replacement in CONTENT_REPLACEMENTS:
                content = re.sub(pattern, replacement, content)
            
            if content != original:
                file_path.write_text(content, encoding='utf-8')
                print(f"[REBRAND] {file_path}")

def main():
    print("Fixing remaining rebranding issues...")
    replace_content()
    print("Done.")

if __name__ == "__main__":
    main()