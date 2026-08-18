---
name: resume-cover-letter-generator
description: >-
  Generates tailored, ATS-safe LaTeX resume and cover letter PDF documents from a job description,
  structured master resume bank (master_resume.yaml), and company notes. Enforces dense 2-page resume
  standards, 1-page muscular cover letters, exact role-title headers, dynamic fact verification,
  clickable repository links, zero fabricated facts, anti-slop checks, and 8+ word wording-reuse guardrails.
  Stores all outputs in .resumegen/resumes/. Powered by the unified 'resumegen' CLI.
---

# ATS Resume & Cover Letter Generator (`resumegen`)

Use this skill when evaluating job descriptions, generating tailored ATS-compliant resumes and matching cover letters in LaTeX and PDF, managing declarative candidate fact banks, or tracking application history.

Designed as an **Agent-First Toolchain** for coding agents (Antigravity/AGY, Claude Code, Codex, Cursor).

---

## 1. Core Operating Architecture & Tooling

The toolchain is powered by a single unified binary: **`resumegen`** (available on `$PATH` in the Nix devShell or in `.agents/skills/resume-cover-letter-generator/scripts/resumegen`).

All generated artifacts, LaTeX sources, PDFs, and tracking ledgers are stored inside **`.resumegen/`**:
- `.resumegen/resumes/` - Generated `.tex` and `.pdf` documents
- `.resumegen/ledger.csv` - Local application tracking ledger

```
resumegen
├── build          # Turnkey 0-to-1 pipeline: Render -> Compile -> Check -> Track (outputs to .resumegen/resumes/)
├── render         # Deterministic LaTeX generator with automatic character escaping
├── compile        # Tectonic-backed PDF compiler (.tex -> .pdf)
├── check          # 10-point ATS, page budget, banned words, & 8-word reuse validator
├── track          # Unified ledger synchronizer & query tool (unifies .resumegen/ledger.csv and ~/Documents/resumes/)
└── skill          # Declarative manager for skills, categories, and bullets in master_resume.yaml
```

---

## 2. Invariant Rules & Quality Standards

### A. Resume Standards
1. **Length & Density**: Strictly **2 pages maximum** (dense, high-information, ~6,500-7,000 selectable characters).
2. **Title Line Invariant**: Under candidate name in header, display **ONLY the exact target role title** from the JD (e.g. `Senior Software Engineer` or `Backend Engineer`). Do NOT concatenate subtitles, tags, or domain phrases.
3. **Education Section**:
   - Matches verified degrees in `master_resume.yaml`.
   - `Languages:` MUST be rendered on its own dedicated separate line below the institution name.
4. **Relocation & Work Authorization**:
   - Fully candidate-configurable via `candidate.relocation` in `master_resume.yaml`.
   - If `enabled: false`, no relocation text is injected into the resume header.
   - If `enabled: true`, uses the candidate's `header_tag` or `target`.
5. **Projects & Open Source**:
   - Every project MUST have its repository URL in linked parentheses: `(\href{repo_url}{repo_display})` followed by `-- full description`.

---

### B. Cover Letter Standards
1. **Length**: Strictly **1 page** (5 focused, muscular paragraphs).
2. **Opening Hook (Punchy 1-Sentence)**:
   - Summarize the exact stack overlap and engineering culture, ending immediately with the application line.
3. **Paragraph Structure**:
   - *Paragraph 1*: Punchy Hook + Direct Application Statement.
   - *Paragraph 2*: Concrete Systems / Backend Engineering Ownership and metrics.
   - *Paragraph 3*: Domain Architecture & Security Governance (access control, isolated query execution, distributed backends).
   - *Paragraph 4*: Testing Standards & Open Source Tooling (fuzz testing, contract tests, streaming data).
   - *Paragraph 5*: Location / timezone availability, relocation readiness and work authorization (resolved dynamically from `candidate.relocation.custom_statement` in `master_resume.yaml` if provided), and GitHub / portfolio links.
4. **Wording Reuse Guardrail**: Zero 8+ word rolling n-gram verbatim matches against baseline templates (`.agents/skills/resume-cover-letter-generator/assets/reference_cover_letter.tex`). Candidate identity tokens and custom statements are automatically exempt.
5. **No AI Slop / Fluff**: No banned words (`genuinely`, `honestly`, `actually`, `thrilled`, `passionate`, `excited`, `leverage`).
6. **No Duration Language**: No `"four years"`, `"years of experience"`, `"5+ years"`.
7. **No Em Dashes**: Clean ASCII separators (`--`, `, `, `-`).

---

### C. Declarative Configuration & Quality Gate Overrides (`master_resume.yaml`)

Candidates can customize their relocation statement and quality check thresholds directly in `master_resume.yaml`:

```yaml
candidate:
  name: "Jane Doe"
  location: "Berlin, Germany"
  ...
  
  # OPTIONAL: Candidate relocation, work authorization, and custom statements
  relocation:
    enabled: false                                 # Set true if actively seeking relocation
    target: "Berlin, Germany"                      # Target city/country
    header_tag: "Open to relocation to Germany"    # Header text (optional)
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
  custom_boilerplate_keywords:                     # Additional excluded tokens for 8+ word checks
    - "berlin"
    - "work authorization"
```

---

## 3. End-to-End Agent Workflow

### Step 1: Verify Prior Applications
Always query if the company was already targeted:
```bash
resumegen track query <company>
```

### Step 2: Turnkey Build
Execute the turnkey pipeline:
```bash
resumegen build \
  --company "Ory" \
  --role "Senior Software Engineer" \
  --location "Munich, Germany / Remote (Central Europe)" \
  --summary-id "go_iam_focus" \
  --lead-skills "Go,Rust,TypeScript,PostgreSQL,Docker" \
  --company-notes "Ory has established the open-source standard for identity, authentication, and zero-trust authorization systems that scale cleanly across enterprise workloads."
```

### Step 3: Quality Gate Verification
`resumegen check` automatically verifies rules against the candidate's YAML settings:
```bash
resumegen check .resumegen/resumes/{candidate}_resume_{company}.pdf \
  --tex .resumegen/resumes/{candidate}_resume_{company}.tex

resumegen check .resumegen/resumes/{candidate}_cover_letter_{company}.pdf \
  --tex .resumegen/resumes/{candidate}_cover_letter_{company}.tex \
  --reference .agents/skills/resume-cover-letter-generator/assets/reference_cover_letter.tex
```

### Step 4: Declarative Mutations
To persist new skills, categories, or bullets:
```bash
# Add a skill
resumegen skill add --category "Databases & Messaging" --skill "ScyllaDB"

# List skills
resumegen skill list

# Add a bullet
resumegen skill add-bullet --company acme_corp --tags rust,grpc,scale \
  --text "Engineered tonic gRPC microservices with automated health probes and graceful degradation."
```
