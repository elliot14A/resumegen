# `resumegen`

> **Agent-First ATS Resume & Cover Letter Toolchain**  
> Built for coding agents (Antigravity/AGY, Claude Code, Codex, Cursor) with an interactive Q&A workflow that proposes every tailoring decision before building.

---

## Overview

`resumegen` is an **agent-first toolchain** designed to give coding agents a deterministic, zero-hallucination runtime for producing tailored resumes and matching cover letters.

**Core Philosophy**: Empower the agent with maximum creative freedom to tailor every section — while strictly enforcing deterministic quality invariants and zero hallucinations. Agents never one-shot a resume. They evaluate, propose what will be built, wait for confirmation, then execute.

All candidate-specific build outputs and application ledgers are stored in the **`.resumegen/`** directory.

---

## Agent Operating Flow

```
[JD Provided]
     │
     ▼
[Read master_resume.yaml + Check Prior Applications]
     │
     ▼
[JD Analysis & Critical Mismatch Gate]
     │
     ▼
[Draft Build Proposal → Present to User]
  ├── Which summary / author new one?
  ├── Per-company: bullets IN (by ID + reason) / bullets OUT (by ID + reason)
  ├── Experience descriptions
  ├── Projects included/excluded
  └── Cover letter hook + body narrative
     │
  [User Reviews]
  ├── "Looks good" → Build
  ├── "Change X" → Revise proposal
  └── "Add new summary/bullet" → author, persist via `resumegen skill`, confirm
     │
     ▼
[resumegen build ...]
     │
     ▼
[Quality Gate: 10-point ATS check]
     │
     ▼
[Report: artifact links + check results]
```

---

## Key Capabilities

- **Deterministic Facts Bank (`master_resume.yaml`)**: Single source of truth for candidate career history, metrics, tech stacks, open-source projects, and skills.
- **Interactive Q&A Workflow**: Agents propose every tailoring decision before building. Never blindly one-shots a resume.
- **Granular Section Customization**: Tailor summaries (`--summary`), front-run languages (`--lead-skills`), filter/prioritize bullets by tags (`--bullet-tags`), explicitly exclude specific bullets by ID (`--exclude-bullets`), curate projects (`--include-projects` / `--exclude-projects`), filter skill categories (`--include-categories`), and write per-company experience descriptions (`--experience-summaries`).
- **Author New Content Persistently**: Agents can write new summaries (`resumegen skill add-summary`) and bullets (`resumegen skill add-bullet`) directly into `master_resume.yaml` and confirm with the user before building.
- **Customizable Relocation & Work Authorization**: Configure candidate-specific relocation statements, header tags, work authorization status, or disable relocation completely.
- **ATS-Guaranteed Single-Column LaTeX**: Dense single-column layout, standard UTF-8/T1 font encodings, active hyperlinks, zero multi-column parse traps.
- **Automated 10-Point Invariant Validator (`resumegen check`)**:
  - Valid PDF header verification (`%PDF-`)
  - Selectable text extraction via `pdftotext` (>100 characters)
  - Strict page budget enforcement (<= 2 pages for resumes, strictly 1 page for cover letters)
  - Anti-AI slop filter: *genuinely, honestly, actually, thrilled, passionate, excited, leverage*
  - No duration language: *"four years", "years of experience"*
  - No em dashes
  - Verification of candidate contact details, links, and education institution
  - Dynamic 8+ word rolling n-gram anti-plagiarism guardrail on cover letters
  - LaTeX special character escaping
- **Dual-Ledger Application Tracking (`resumegen track`)**: Indexes past applications across `.resumegen/ledger.csv` and `~/Documents/resumes/ledger.csv`.
- **Declarative Skill Management (`resumegen skill`)**: Add summaries, skills, categories, and experience bullets programmatically.

---

## Installation & Setup

### Option 1: Nix Flakes (Recommended)

```bash
nix develop
resumegen --version
```

### Option 2: Cargo Build

```bash
cargo build --release
cp target/release/resumegen .agents/skills/resume-cover-letter-generator/scripts/
chmod +x .agents/skills/resume-cover-letter-generator/scripts/resumegen
```

---

## Example Tailored Build

### Go / IAM Role (Back-step React/TypeScript, front-run Go/auth)

```bash
resumegen build \
  --company "Ory" \
  --role "Senior Backend Engineer" \
  --location "Munich, Germany / Remote" \
  --summary-id "go_iam_focus" \
  --lead-skills "Go,PostgreSQL,Docker,Kubernetes,TypeScript,Rust" \
  --bullet-tags "go,postgres,iam,auth,security,scale" \
  --exclude-bullets "gaur_react_tanstack,gaur_mcp_agent" \
  --max-bullets-per-role 4 \
  --experience-summaries "gaur_data:Architected all-Rust gRPC platform with per-tenant auth and DuckDB analytical isolation;factly:Led Go backend for data analytics" \
  --include-projects "abel,ruspie,elliot14a" \
  --company-notes "Ory has established the open-source standard for identity, authentication, and zero-trust authorization systems."
```

### TypeScript / Fullstack Role (Back-step Rust/gRPC, front-run TypeScript/React)

```bash
resumegen build \
  --company "Vercel" \
  --role "Senior TypeScript Engineer" \
  --location "Remote (EU)" \
  --summary-id "typescript_platform_focus" \
  --lead-skills "TypeScript,JavaScript,React,Node.js,Next.js,PostgreSQL,Docker" \
  --bullet-tags "typescript,react,frontend,fullstack,mcp,ai" \
  --exclude-bullets "gaur_grpc_duckdb,gaur_auth_path,factly_ruspie_arrow" \
  --max-bullets-per-role 4 \
  --experience-summaries "gaur_data:Built and owned the full GaurData TypeScript platform;factly:Led open-source analytics frontend and streaming product integrations" \
  --include-projects "elliot14a,abel" \
  --company-notes "Vercel defines the modern deployment and developer experience standard for frontend infrastructure."
```

---

## CLI Command Reference

```bash
resumegen <COMMAND>
```

| Command | Description |
| :--- | :--- |
| **`init`** | Initialize workspace with starter templates and `.resumegen/` directory |
| **`build`** | Turnkey pipeline with full section customization: Render -> Compile -> Check -> Track |
| **`render`** | Render tailored `.tex` sources from `master_resume.yaml` into `.resumegen/resumes/` |
| **`compile`** | Compile `.tex` documents to PDF via Tectonic |
| **`check`** | Run 10-point ATS, page count, anti-slop, and anti-plagiarism verification |
| **`track`** | Sync dual-ledgers, query history, and list past applications |
| **`skill`** | Declaratively manage summaries, skills, categories, and bullets in `master_resume.yaml` |

### Granular Section Customization Options (`build` & `render`)

| Flag | Type | Description |
| :--- | :--- | :--- |
| `--summary` | String | Direct custom summary paragraph override |
| `--summary-id` | String | Select summary archetype from `summary_bank` |
| `--lead-skills` | CSV | Skills to front-run in Languages section |
| `--bullet-tags` | CSV | Prioritize matching experience bullets (e.g. `backend,postgres` vs `react,typescript`) |
| `--exclude-bullets` | CSV | **Explicitly exclude specific bullet IDs** (e.g. `gaur_react,factly_ruspie`) |
| `--max-bullets-per-role` | Integer | Cap bullets per role for tight page budget control |
| `--experience-summaries` | Semicolons | Per-company experience descriptions (e.g. `"gaur_data:Architected...;factly:Led..."`) |
| `--include-projects` | CSV | Include only specified project IDs |
| `--exclude-projects` | CSV | Omit specified project IDs |
| `--include-categories` | CSV | Include only specified skill categories |
| `--exclude-categories` | CSV | Omit specified skill categories |
| `--company-notes` | String | Tailored cover letter opening hook |
| `--cover-body` | String | Tailored cover letter technical narrative |

### Skill Management (`resumegen skill`)

```bash
# Add a new summary archetype to master_resume.yaml
resumegen skill add-summary \
  --id "typescript_platform_focus" \
  --focus "TypeScript, React & Developer Tooling" \
  --text "Fullstack engineer who built and owned the entire GaurData platform..."

# Add a bullet to an experience entry
resumegen skill add-bullet --company gaur_data --tags typescript,react --text "..."

# Add a skill to a category
resumegen skill add --category "Languages" --skill "Zig"

# List all skills and summaries
resumegen skill list

# Add a new skill category
resumegen skill add-category "Cloud & Orchestration" --skills "Kubernetes,Terraform,AWS"
```

### Application Tracking (`resumegen track`)

```bash
resumegen track query "Stripe"      # Check if already applied
resumegen track list --limit 15     # List recent applications
resumegen track stats               # Aggregate pipeline stats
resumegen track sync                # Re-synchronize dual ledgers
```

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
│   ├── skill.rs                               # Skill matrix & summary editor
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
│           ├── SKILL.md                       # Interactive Q&A skill workflow
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
