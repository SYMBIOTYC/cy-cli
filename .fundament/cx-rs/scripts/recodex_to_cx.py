#!/usr/bin/env python3
"""
SYMBIOTYC CY-CLI rebranding tracker.
Replaces 'codex' with 'cx', 'OpenAI' with 'oi', 'ChatGPT' with 'gt' in modified files on-the-fly.
"""

import re
import sys
from pathlib import Path

# Script is at .fundament/codex-rs/scripts/recodex_to_cx.py
# repo_root for file paths is .fundament/codex-rs/
REPO_ROOT = Path(__file__).resolve().parent.parent

# Files modified in the current change set (relative to REPO_ROOT)
MODIFIED_FILES = [
    "model-provider-info/src/lib.rs",
    "core/src/config/mod.rs",
    "models-manager/models.json",
]

REPLACEMENTS = [
    # Brand replacements
    (r'\bOpenAI\b', 'oi', 'Replace OpenAI with oi'),
    (r'\bChatGPT\b', 'gt', 'Replace ChatGPT with gt'),
    # (pattern, replacement, description)
    (r'\bcodex\b', 'cx', 'Replace standalone "codex" with "cx"'),
    (r'\bCodex\b', 'CX', 'Replace standalone "Codex" with "CX"'),
    (r'codex-rs', 'cx-rs', 'Replace crate directory name'),
    (r'codex_config', 'cx_config', 'Replace module prefix'),
    (r'codex_protocol', 'cx_protocol', 'Replace module prefix'),
    (r'codex_model_provider', 'cx_model_provider', 'Replace module prefix'),
    (r'codex_model_provider_info', 'cx_model_provider_info', 'Replace module prefix'),
    (r'codex_models_manager', 'cx_models_manager', 'Replace module prefix'),
    (r'codex_login', 'cx_login', 'Replace module prefix'),
    (r'codex_api', 'cx_api', 'Replace module prefix'),
    (r'codex_http_client', 'cx_http_client', 'Replace module prefix'),
    (r'codex_utils', 'cx_utils', 'Replace module prefix'),
    (r'codex_core', 'cx_core', 'Replace module prefix'),
    (r'codex_mcp', 'cx_mcp', 'Replace module prefix'),
    (r'codex_features', 'cx_features', 'Replace module prefix'),
    (r'codex_exec_server', 'cx_exec_server', 'Replace module prefix'),
    (r'codex_thread_store', 'cx_thread_store', 'Replace module prefix'),
    (r'codex_history', 'cx_history', 'Replace module prefix'),
    (r'codex_skills_extension', 'cx_skills_extension', 'Replace module prefix'),
    (r'codex_analytics', 'cx_analytics', 'Replace module prefix'),
    (r'codex_app_server', 'cx_app_server', 'Replace module prefix'),
    (r'codex_rollout', 'cx_rollout', 'Replace module prefix'),
    (r'codex_secrets', 'cx_secrets', 'Replace module prefix'),
    (r'codex_extension_api', 'cx_extension_api', 'Replace module prefix'),
    (r'codex_core_plugins', 'cx_core_plugins', 'Replace module prefix'),
    (r'codex_network_proxy', 'cx_network_proxy', 'Replace module prefix'),
    (r'codex_sandboxing', 'cx_sandboxing', 'Replace module prefix'),
    (r'codex_memories_read', 'cx_memories_read', 'Replace module prefix'),
    (r'codex_git_utils', 'cx_git_utils', 'Replace module prefix'),
    (r'codex_install_context', 'cx_install_context', 'Replace module prefix'),
    (r'codex_agent_graph_store', 'cx_agent_graph_store', 'Replace module prefix'),
    (r'codex_code_mode', 'cx_code_mode', 'Replace module prefix'),
    (r'codex_responses_metadata', 'cx_responses_metadata', 'Replace module prefix'),
]

def rebrand_file(path: Path):
    content = path.read_text(encoding='utf-8')
    original = content
    
    for pattern, replacement, desc in REPLACEMENTS:
        content = re.sub(pattern, replacement, content)
    
    if content != original:
        path.write_text(content, encoding='utf-8')
        print(f"[REBRANDED] {path}")
        return True
    return False

def main():
    print("SYMBIOTYC CY-CLI rebranding tracker")
    print("=" * 50)
    
    changed = []
    for rel_path in MODIFIED_FILES:
        full_path = REPO_ROOT / rel_path
        if full_path.exists():
            if rebrand_file(full_path):
                changed.append(rel_path)
        else:
            print(f"[MISSING] {full_path}")
    
    print("=" * 50)
    if changed:
        print(f"Rebranded {len(changed)} files:")
        for f in changed:
            print(f"  - {f}")
    else:
        print("No files rebranded.")

if __name__ == "__main__":
    main()
