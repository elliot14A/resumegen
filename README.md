# `resumegen`

> **Agent-First ATS Resume & Cover Letter Toolchain**  
> Built for coding agents (Antigravity/AGY, Claude Code, Codex, Cursor) to deterministically generate, validate, and track ATS-compliant documents from declarative career facts.

---

## Overview

`resumegen` is an **agent-first toolchain** designed to give coding agents a deterministic, zero-hallucination runtime for producing tailored resumes and matching cover letters.

**Core Philosophy**: Empower the agent with maximum creative freedom to tailor every section (summary, lead skills, experience bullets, and project curation) while strictly enforcing deterministic quality invariants and zero hallucinations.

All candidate-specific build outputs and application ledgers are automatically stored in the **`.resumegen/`** directory.

---

## Key Capabilities for Agents

- **Deterministic Facts Bank (`master_resume.yaml`)**: Single source of truth for candidate career history, metrics, tech stacks, open-source projects, and skills.
- **Granular Section Customization**: Tailor summaries (`--summary`), front-run languages (`--lead-skills`), filter/prioritize bullets by tags (`--bullet-tags`), curate relevant projects (`--include-projects` / `--exclude-projects`), and filter skill categories (`--include-categories`).
- **Customizable Relocation & Work Authorization**: Configure candidate-specific relocation statements, header tags, work authorization status, or disable relocation completely.
- **Configurable Quality Invariants (`custom_checks`)**: Tailor institution verification, page budget ceilings, and custom banned word filters directly in YAML.
- **ATS-Guaranteed Single-Column LaTeX**: Dense single-column layout, standard UTF-8/T1 font encodings, active hyperlinks, zero multi-column parse traps.
- **Automated 10-Point Invariant Validator (`resumegen check`)**:
  - Valid PDF header verification (`%PDF-`)
  - Selectable text extraction via `pdftotext` (>100 characters)
  - Strict page budget enforcement (<= 2 pages for resumes, strictly 1 page for cover letters)
  - Anti-AI slop filter (rejects banned filler: *genuinely, honestly, actually, thrilled, passionate, excited, leverage*)
  - No duration language (*"four years", "years of experience"*)
  - No em dashes
  - Verification of candidate contact details, links, and education institution
  - Dynamic 8+ word rolling n-gram anti-plagiarism guardrail on cover letters with candidate-token exemptions
  - LaTeX special character escaping (`&`, `%`, `$`, `#`, `_`, `{`, `}`)
- **Agent Output Containment (`.resumegen/`)**: All generated `.tex`, `.pdf`, and `ledger.csv` files are cleanly sandboxed inside `.resumegen/`, keeping the repository clean.
- **Dual-Ledger Application Tracking (`resumegen track`)**: Indexes past applications across `.resumegen/ledger.csv` and global storage (`~/Documents/resumes/ledger.csv`) to prevent duplicate submissions.
- **Declarative Skill Management (`resumegen skill`)**: Add, query, and prune skill categories and experience bullets programmatically.
- **Instant Workspace Initialization (`resumegen init`)**: One command for an agent to bootstrap a new candidate workspace.

---

## Installation & Setup

### Option 1: Using Nix Flakes (Recommended)

`resumegen` includes a complete `flake.nix` environment with Rust, Cargo, `tectonic`, `poppler-utils` (`pdftotext`), and font dependencies pre-configured. Entering the dev shell automatically builds and places the binary on `PATH`:

```bash
# Enter dev shell with all dependencies
nix develop

# resumegen is automatically available on PATH
resumegen --version
```

### Option 2: Standalone Cargo Build

Ensure you have **Rust (1.75+)**, **tectonic**, and **poppler-utils** installed:

```bash
# Build the release binary
cargo build --release

# Copy binary to the Agent Skill scripts directory
cp target/release/resumegen .agents/skills/resume-cover-letter-generator/scripts/
chmod +x .agents/skills/resume-cover-letter-generator/scripts/resumegen

# (Optional) Install globally
sudo cp target/release/resumegen /usr/local/bin/
```

---

## Agent Turnkey Tailoring & Build

```bash
resumegen build \
  --company "Ory" \
  --role "Senior Backend Engineer" \
  --location "Munich, Germany / Remote" \
  --summary "Senior backend engineer specializing in Go microservices, access control governance, and low-latency PostgreSQL profiling." \
  --lead-skills "Go,PostgreSQL,Docker,Kubernetes,TypeScript" \
  --bullet-tags "backend,postgres,scale,iam" \
  --max-bullets-per-role 4 \
  --include-projects "abel,ruspie,elliot14a" \
  --company-notes "Ory has established the open-source standard for identity, authentication, and zero-trust authorization systems."
```

Output:
```
Starting Turnkey Build Pipeline...
  Target: Ory | Senior Backend Engineer (Munich, Germany / Remote)

[1/4] Step 1: Rendering LaTeX sources...
  [OK] Rendered .resumegen/resumes/jane_doe_resume_ory.tex
  [OK] Rendered .resumegen/resumes/jane_doe_cover_letter_ory.tex

[2/4] Step 2: Compiling to PDF via tectonic...
[INFO] Compiling .resumegen/resumes/jane_doe_resume_ory.tex with tectonic...
[PASS] Successfully compiled to .resumegen/resumes/jane_doe_resume_ory.pdf (40112 bytes)
[INFO] Compiling .resumegen/resumes/jane_doe_cover_letter_ory.tex with tectonic...
[PASS] Successfully compiled to .resumegen/resumes/jane_doe_cover_letter_ory.pdf (18947 bytes)

[3/4] Step 3: Validating ATS & wording guardrails...
[PASS] PDF Format Header: Valid PDF header found
[PASS] Text Selectability: Extracted 5262 selectable characters
[PASS] Page Count Constraint: Document has 2 page(s) (Max allowed: 2)
[PASS] No Banned AI Slop: No banned filler words
[PASS] No Duration Language: No duration phrases found
[PASS] No Em Dashes: No em dashes found
[PASS] Institution Name Accuracy: Verified
[PASS] Candidate Identity & Links: Verified
[PASS] ATS: Single-Column Layout: Single-column layout verified
[PASS] LaTeX Special Characters Escaped: Properly escaped

[4/4] Step 4: Recording to unified application ledger...
Turnkey Build COMPLETE!
  Resume PDF       : .resumegen/resumes/jane_doe_resume_ory.pdf
  Cover Letter PDF : .resumegen/resumes/jane_doe_cover_letter_ory.pdf
  Ledger Updated   : 215 entries indexed across repositories
```

---

## CLI Command Reference

```bash
resumegen <COMMAND>
```

| Command | Description |
| :--- | :--- |
| **`init`** | Initialize workspace with starter templates and `.resumegen/` directory |
| **`build`** | Turnkey 0-to-1 pipeline with section customization: Render -> Compile -> Check -> Track |
| **`render`** | Render tailored `.tex` sources from `master_resume.yaml` with custom flags into `.resumegen/resumes/` |
| **`compile`** | Compile `.tex` documents to PDF via Tectonic |
| **`check`** | Run 10-point ATS, page count, anti-slop, and anti-plagiarism verification |
| **`track`** | Sync dual-ledgers (`.resumegen/ledger.csv`), query history, and list past applications |
| **`skill`** | Declaratively query and update skills/bullets in `master_resume.yaml` |

### Granular Section Customization Options (`build` & `render`)

| Flag | Type | Description |
| :--- | :--- | :--- |
| `--summary` | String | Direct custom summary paragraph override |
| `--summary-id` | String | Select summary archetype from `summary_bank` |
| `--lead-skills` | CSV | Skills to front-run in Languages |
| `--bullet-tags` | CSV | Comma-separated tags to prioritize matching experience bullets (e.g. `backend,postgres` vs `frontend,typescript`) |
| `--max-bullets-per-role` | Integer | Cap bullets per role to maintain tight page budget |
| `--include-projects` | CSV | Comma-separated project IDs to include in specified order |
| `--exclude-projects` | CSV | Comma-separated project IDs to omit |
| `--include-categories` | CSV | Skill category names to include |
| `--exclude-categories` | CSV | Skill category names to omit |
| `--company-notes` | String | Tailored cover letter opening hook |
| `--cover-body` | String | Tailored cover letter technical narrative body |

---

## Repository Structure

```
resume-builder/
├── Cargo.toml                                 # Root Cargo crate configuration
├── src/                                       # Modular Rust CLI source code
│   ├── main.rs                                # Application entrypoint
│   ├── lib.rs                                 # Library root
│   ├── cli.rs                                 # Clap CLI commands & flags
│   ├── models.rs                              # Schema data types
│   ├── render.rs                              # LaTeX generator with section filtering
│   ├── compile.rs                             # Tectonic compiler
│   ├── check.rs                               # 10-point ATS quality gate
│   ├── track.rs                               # Ledger manager
│   ├── skill.rs                               # Skill matrix editor
│   └── init.rs                                # Workspace bootstrap
├── tests/                                     # Automated test suite (7 tests)
├── flake.nix                                  # Nix development shell
├── master_resume.example.yaml                 # Example template for candidate facts
├── .resumegen/                                # Sandboxed output directory (gitignored)
│   ├── ledger.csv                             # Application history tracking ledger
│   └── resumes/                               # Compiled PDFs and generated LaTeX sources
├── .agents/                                   # Agent Skills standard package
│   └── skills/
│       └── resume-cover-letter-generator/
│           ├── SKILL.md                       # Canonical skill prompt & execution workflows
│           ├── assets/
│           │   ├── reference_resume.tex
│           │   └── reference_cover_letter.tex
│           ├── references/
│           │   └── master_resume_schema.md
│           └── scripts/
│               └── resumegen
├── AGENTS.md                                  # Repository guide for coding agents
├── CLAUDE.md                                  # Symlinked to AGENTS.md
└── README.md
```

---

## License

Dual-licensed under either **MIT License** or **Apache License, Version 2.0**.
