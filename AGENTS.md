# Agent Guide & Repository Architecture (`AGENTS.md`)

This document provides orientation, architectural guidelines, and development workflows for coding agents (Antigravity/AGY, Claude Code, Codex, Cursor) working directly on the **`resumegen`** codebase.

> **Note**: If you are looking for instructions on how to use `resumegen` as an Agent Skill to generate tailored resumes and cover letters from a job description, refer to [`.agents/skills/resume-cover-letter-generator/SKILL.md`](file:///.agents/skills/resume-cover-letter-generator/SKILL.md).

---

## 1. Repository Layout

```
resume-builder/
├── Cargo.toml                                 # Root Cargo configuration (single-binary crate)
├── src/                                       # Modular Rust implementation
│   ├── main.rs                                # Binary entrypoint
│   ├── lib.rs                                 # Library root
│   ├── cli.rs                                 # Clap CLI commands
│   ├── models.rs                              # Schema data structures
│   ├── render.rs                              # LaTeX generator
│   ├── compile.rs                             # Tectonic compiler
│   ├── check.rs                               # 10-point ATS quality gate
│   ├── track.rs                               # Ledger manager
│   ├── skill.rs                               # Skill matrix editor
│   └── init.rs                                # Workspace bootstrap
├── flake.nix                                  # Nix development shell (Rust, Tectonic, poppler-utils)
├── master_resume.example.yaml                 # Generic candidate facts bank template
├── .gitignore                                 # Ignores target/, .resumegen/, personal YAML, and binaries
├── README.md                                  # User & open-source documentation
├── AGENTS.md                                  # This codebase orientation file
├── CLAUDE.md                                  # Claude Code symlink to AGENTS.md
│
├── .agents/                                   # Agent Skills standard package
│   └── skills/
│       └── resume-cover-letter-generator/
│           ├── SKILL.md                       # Skill prompt & job-application workflow
│           ├── assets/                        # Canonical reference LaTeX templates
│           │   ├── reference_resume.tex
│           │   └── reference_cover_letter.tex
│           ├── references/                    # Master resume schema documentation
│           │   └── master_resume_schema.md
│           └── scripts/                       # Gitignored release binary destination
│               └── resumegen
│
└── .resumegen/                                # Sandboxed runtime directory (GITIGNORED)
    ├── ledger.csv                             # Local dual-sync application tracking ledger
    └── resumes/                               # Output directory for all generated .tex and .pdf files
```

---

## 2. Core Architectural Principles

1. **Single-Binary Engine (`resumegen`)**:
   - All capabilities (`init`, `build`, `render`, `compile`, `check`, `track`, `skill`) are subcommands of a single Rust binary implemented in `src/`.
   - Never introduce separate standalone binaries; extend subcommands in `src/cli.rs`.

2. **Zero Root Pollution (`.resumegen/` Sandbox)**:
   - All generated artifacts (`.tex`, `.pdf`, `.log`, and `ledger.csv`) must strictly be written inside `.resumegen/`.
   - The `.resumegen/` directory and candidate `master_resume.yaml` are gitignored to preserve privacy.

3. **Candidate-Agnostic Design**:
   - The CLI must never hardcode candidate names, URLs, institutions, or relocation preferences.
   - All candidate metadata, custom relocation statements, work authorization, and custom quality check overrides are resolved dynamically from `master_resume.yaml` (or `.resumegen/master_resume.yaml` / `master_resume.example.yaml`).

4. **10-Point Quality Gate (`resumegen check`)**:
   - Every generated document must pass PDF selectability, page budget limits (strictly <= 2 pages for resumes, 1 page for cover letters), anti-slop checks, and 8+ word anti-plagiarism checks before ledger recording.

---

## 3. Development & Build Workflows

### Building the Binary
```bash
# Build the optimized release binary
cargo build --release

# Deploy the binary to the agent skill scripts folder
cp target/release/resumegen .agents/skills/resume-cover-letter-generator/scripts/
chmod +x .agents/skills/resume-cover-letter-generator/scripts/resumegen
```

In the Nix dev shell (`nix develop`), this deployment happens automatically on shell initialization.

### Running Checks & Tests
```bash
# Run Rust unit tests
cargo test

# Validate output
resumegen check .resumegen/resumes/jane_doe_resume_testcorp.pdf
```

### Git & Security Hygiene
- **Never commit compiled binaries** (`.agents/**/scripts/resumegen` is gitignored).
- **Never commit candidate personal data** (`master_resume.yaml` is gitignored; use `master_resume.example.yaml` for public commits).
