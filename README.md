# `resumegen`

> **Agent-First ATS Resume & Cover Letter Toolchain**  
> Built for coding agents (Antigravity/AGY, Claude Code, Codex, Cursor) to deterministically generate, validate, and track ATS-compliant documents from declarative career facts.

---

## Overview

`resumegen` is an **agent-first toolchain** designed to give coding agents a deterministic, zero-hallucination runtime for producing tailored resumes and matching cover letters.

Instead of having LLMs directly write raw LaTeX or unstructured markdown (which frequently causes compilation errors, formatting drift, page overflow, and AI slop), agents invoke `resumegen` to render, compile, validate against a strict 10-point quality gate, and track applications across a unified dual ledger.

All candidate-specific build outputs and application ledgers are automatically stored in the **`.resumegen/`** directory.

---

## Key Capabilities for Agents

- **Deterministic Facts Bank (`master_resume.yaml`)**: Single source of truth for candidate career history, metrics, tech stacks, open-source projects, and skills.
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

## Declarative Facts Bank (`master_resume.yaml`)

Candidate career facts, relocation preferences, and quality checks are declared in `master_resume.yaml`:

```yaml
candidate:
  name: "Jane Doe"
  title: "Senior Backend Engineer"
  location: "Berlin, Germany"
  email: "jane.doe@example.com"
  phone: "+49 151 12345678"
  links:
    portfolio: "https://janedoe.dev"
    portfolio_display: "janedoe.dev"
    github: "https://github.com/janedoe"
    github_display: "github.com/janedoe"
    linkedin: "https://linkedin.com/in/janedoe"
    linkedin_display: "linkedin.com/in/janedoe"
  
  # OPTIONAL: Relocation, Work Authorization & Custom Closing Statements
  relocation:
    enabled: false                                 # Set true if actively seeking relocation
    target: "Berlin, Germany"
    header_tag: "Open to relocation to Germany"    # Custom text for header line
    custom_statement: "I am based in Berlin and eligible to work in the EU without visa sponsorship."
    work_authorization: "EU Citizen"
    spoken_languages: "English (Fluent) · German (B2)"

# OPTIONAL: Quality Gate Overrides
custom_checks:
  verify_institution: true                         # Enable/disable education institution match
  max_resume_pages: 2                              # Resume page ceiling
  max_cover_letter_pages: 1                        # Cover letter page ceiling
  banned_words:                                    # Custom banned filler words
    - "genuinely"
    - "honestly"
    - "actually"
    - "thrilled"
    - "passionate"
    - "excited"
    - "leverage"
```

---

## Agent 0-to-1 Workflow

### 1. Initialize Workspace (If Not Already Present)
```bash
resumegen init --path .
```
This generates:
- `master_resume.yaml` (starter candidate facts bank)
- `.resumegen/resumes/` (sandboxed output directory)

### 2. Verify Past Applications
Before tailoring for a company, the agent queries the application history:
```bash
resumegen track query "Stripe"
```

### 3. One-Shot Turnkey Generation
The agent triggers the end-to-end pipeline with target parameters from the JD:
```bash
resumegen build \
  --company "Ory" \
  --role "Senior Software Engineer" \
  --location "Munich, Germany / Remote" \
  --summary-id "go_iam_focus" \
  --lead-skills "Go,Rust,TypeScript,PostgreSQL,Docker" \
  --relocation-target "Germany"
```

Output:
```
Starting Turnkey Build Pipeline...
  Target: Ory | Senior Software Engineer (Munich, Germany / Remote)

[1/4] Step 1: Rendering LaTeX sources...
  [OK] Rendered .resumegen/resumes/jane_doe_resume_ory.tex
  [OK] Rendered .resumegen/resumes/jane_doe_cover_letter_ory.tex

[2/4] Step 2: Compiling to PDF via tectonic...
[INFO] Compiling .resumegen/resumes/jane_doe_resume_ory.tex with tectonic...
[PASS] Successfully compiled to .resumegen/resumes/jane_doe_resume_ory.pdf (41726 bytes)
[INFO] Compiling .resumegen/resumes/jane_doe_cover_letter_ory.tex with tectonic...
[PASS] Successfully compiled to .resumegen/resumes/jane_doe_cover_letter_ory.pdf (20679 bytes)

[3/4] Step 3: Validating ATS & wording guardrails...
[PASS] PDF Format Header: Valid PDF header found
[PASS] Text Selectability: Extracted 6947 selectable characters
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
  Ledger Updated   : 204 entries indexed across repositories
```

---

## CLI Command Reference

```bash
resumegen <COMMAND>
```

| Command | Description |
| :--- | :--- |
| **`init`** | Initialize workspace with starter templates and `.resumegen/` directory |
| **`build`** | Turnkey 0-to-1 pipeline: Render -> Compile -> Check -> Track (outputs to `.resumegen/resumes/`) |
| **`render`** | Render tailored `.tex` sources from `master_resume.yaml` into `.resumegen/resumes/` |
| **`compile`** | Compile `.tex` documents to PDF via Tectonic |
| **`check`** | Run 10-point ATS, page count, anti-slop, and anti-plagiarism verification |
| **`track`** | Sync dual-ledgers (`.resumegen/ledger.csv`), query history, and list past applications |
| **`skill`** | Declaratively query and update skills/bullets in `master_resume.yaml` |

### Application Tracking (`track`)
```bash
# Query application history for a specific company
resumegen track query "Stripe"

# List recent applications
resumegen track list --limit 15

# View aggregate stats
resumegen track stats

# Resynchronize dual ledgers
resumegen track sync
```

### Declarative Skill Management (`skill`)
```bash
# List all skill categories
resumegen skill list

# Add a skill to an existing category
resumegen skill add --category "Languages" --skill "Zig"

# Create a new skill category
resumegen skill add-category "Cloud & Orchestration" --skills "Kubernetes, Terraform, AWS"

# Add an experience bullet
resumegen skill add-bullet --company acme_corp --tags scale,postgres \
  --text "Optimized PostgreSQL connection pool sizing, reducing query latency by 40%."
```

### Invariant & Compliance Checking (`check`)
```bash
# Validate a resume PDF against ATS rules & master_resume.yaml
resumegen check .resumegen/resumes/jane_doe_resume_google.pdf \
  --tex .resumegen/resumes/jane_doe_resume_google.tex

# Validate a cover letter with anti-plagiarism guardrails against a reference baseline
resumegen check .resumegen/resumes/jane_doe_cover_letter_google.pdf \
  --tex .resumegen/resumes/jane_doe_cover_letter_google.tex \
  --reference .agents/skills/resume-cover-letter-generator/assets/reference_cover_letter.tex
```

---

## Repository Structure

```
resume-builder/
├── Cargo.toml                                 # Root Cargo crate configuration
├── src/                                       # Modular Rust CLI source code
│   ├── main.rs                                # Application entrypoint
│   ├── lib.rs                                 # Library root
│   ├── cli.rs                                 # Clap CLI commands
│   ├── models.rs                              # Schema data types
│   ├── render.rs                              # LaTeX generator
│   ├── compile.rs                             # Tectonic compiler
│   ├── check.rs                               # 10-point ATS quality gate
│   ├── track.rs                               # Ledger manager
│   ├── skill.rs                               # Skill matrix editor
│   └── init.rs                                # Workspace bootstrap
├── flake.nix                                  # Nix development shell
├── master_resume.example.yaml                 # Example template for candidate facts
├── .resumegen/                                # Sandboxed output directory (gitignored)
│   ├── ledger.csv                             # Application history tracking ledger
│   └── resumes/                               # Compiled PDFs and generated LaTeX sources
├── .agents/                                   # Agent Skills standard package
│   └── skills/
│       └── resume-cover-letter-generator/
│           ├── SKILL.md
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
