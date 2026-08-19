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
│   ├── render.rs                              # LaTeX generator with section filtering
│   ├── compile.rs                             # Tectonic compiler
│   ├── check.rs                               # 10-point ATS quality gate
│   ├── track.rs                               # Ledger manager
│   ├── skill.rs                               # Skill matrix & summary editor
│   └── init.rs                                # Workspace bootstrap
├── tests/                                     # Automated test suite
│   ├── test_resumegen.rs                      # 7 integration tests
│   ├── sample_job_description.txt             # Test fixture
│   └── sample_company_notes.txt              # Test fixture
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
│           ├── SKILL.md                       # Skill prompt & interactive Q&A workflow
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
   - All candidate metadata, custom relocation statements, work authorization, and custom quality check overrides are resolved dynamically from `master_resume.yaml`.

4. **10-Point Quality Gate (`resumegen check`)**:
   - Every generated document must pass PDF selectability, page budget limits, anti-slop checks, and 8+ word anti-plagiarism checks before ledger recording.

---

## 3. Key Data Models (`src/models.rs`)

### `ExperienceItem`
```rust
pub struct ExperienceItem {
    pub id: String,
    pub company: String,
    pub role: String,
    pub dates: String,
    pub location: String,
    pub summary: Option<String>,              // Default experience description
    pub summaries: Option<BTreeMap<String, String>>, // Focus-keyed descriptions (e.g. "backend", "fullstack")
    pub roles_history: Vec<RoleHistoryItem>,  // Multi-role progression
    pub bullets: Vec<BulletItem>,
}
```

### `BulletItem`
```rust
pub struct BulletItem {
    pub id: String,          // Stable ID used for --exclude-bullets
    pub tags: Vec<String>,   // Used for --bullet-tags prioritization
    pub text: String,
}
```

### `SummaryItem`
```rust
pub struct SummaryItem {
    pub id: String,    // Referenced by --summary-id
    pub focus: String, // Human-readable description of the summary archetype
    pub text: String,
}
```

---

## 4. Render Pipeline & Section Customization (`src/render.rs`)

`do_render(RenderOptions)` applies the following transformations in order:

1. **Summary selection**: `--summary` overrides text directly. `--summary-id` looks up `summary_bank`. Falls back to first summary.
2. **Bullet filtering per company**:
   - `--exclude-bullets <id,id>`: hard-removes bullet IDs before any filtering.
   - `--bullet-tags <tag,tag>`: floats matching bullets to the top of each role's list.
   - `--max-bullets-per-role <n>`: truncates the final list to n bullets.
3. **Experience descriptions**: `--experience-summaries "company_id:text;company_id:text"` injects per-company italic descriptions below the company header. Falls back to YAML `summary` or focus-keyed `summaries` matched against `--bullet-tags`.
4. **Project curation**: `--include-projects` or `--exclude-projects` (mutually exclusive).
5. **Skill front-running**: `--lead-skills` moves matching items to the front of each category's item list.
6. **Category filtering**: `--include-categories` or `--exclude-categories`.

---

## 5. Development & Build Workflows

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
# Run Rust unit tests (7 integration tests)
cargo test

# Validate a generated PDF against all 10 ATS rules
resumegen check .resumegen/resumes/jane_doe_resume_testcorp.pdf \
  --tex .resumegen/resumes/jane_doe_resume_testcorp.tex
```

### Adding New CLI Flags
1. Add the field to the `Commands::Build` and `Commands::Render` variants in `src/cli.rs`.
2. Add the field to `RenderOptions<'a>` in `src/render.rs`.
3. Thread it through the match destructure and `RenderOptions` construction in `cli.rs`.
4. Use it in `do_render()` in `render.rs`.
5. Update `tests/test_resumegen.rs` to include the new field in the `RenderOptions` struct literal.

### Git & Security Hygiene
- **Never commit compiled binaries** (`.agents/**/scripts/resumegen` is gitignored).
- **Never commit candidate personal data** (`master_resume.yaml` is gitignored; use `master_resume.example.yaml` for public commits).
