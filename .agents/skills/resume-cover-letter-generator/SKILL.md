---
name: resume-cover-letter-generator
description: >-
  Generates tailored, ATS-safe LaTeX resume and cover letter PDF documents from a job description,
  structured master resume bank (master_resume.yaml), and company notes. Enforces stack front-running,
  critical mismatch detection gates, granular section customization, dense 2-page resume standards,
  1-page muscular cover letters, exact role-title headers, dynamic fact verification, clickable repository links,
  zero fabricated facts, anti-slop checks, and 8+ word wording-reuse guardrails. Stores all outputs in .resumegen/resumes/.
---

# ATS Resume & Cover Letter Generator (`resumegen`)

Use this skill when evaluating job descriptions, generating tailored ATS-compliant resumes and matching cover letters in LaTeX and PDF, managing declarative candidate fact banks, or tracking application history.

Designed as an **Agent-First Toolchain** for coding agents (Antigravity/AGY, Claude Code, Codex, Cursor) with a core motto:  
**Empower the agent with maximum creative freedom to tailor content while strictly enforcing deterministic quality invariants with zero hallucinations.**

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
1. **Length & Density**: Strictly **2 pages maximum** (dense, high-information, ~5,000-7,000 selectable characters).
2. **Title Line Invariant**: Under candidate name in header, display **ONLY the exact target role title** from the JD (e.g. `Senior Software Engineer` or `Senior TypeScript Engineer`). Do NOT concatenate subtitles, tags, or domain phrases.
3. **Education Section**:
   - Matches verified degrees in `master_resume.yaml`.
   - `Languages:` MUST be rendered on its own dedicated separate line below the institution name.
4. **Relocation & Work Authorization**:
   - Fully candidate-configurable via `candidate.relocation` in `master_resume.yaml`.
   - If `enabled: false`, no relocation text is injected into the resume header.
   - If `enabled: true`, uses the candidate's `header_tag` or `target`.
5. **Projects & Open Source**:
   - Every project MUST have its repository URL in linked parentheses: `(\href{repo_url}{repo_display})` followed by `-- full description`.
   - Ground-truth repository URLs and descriptions are preserved without fabrication.

---

### B. Cover Letter Standards
1. **Length**: Strictly **1 page** (5 focused, muscular paragraphs).
2. **Opening Hook (Punchy 1-Sentence)**:
   - Summarize the exact stack overlap and engineering culture, ending immediately with the application line.
3. **Paragraph Structure**:
   - *Paragraph 1*: Punchy Hook + Direct Application Statement (`--company-notes`).
   - *Paragraph 2*: Concrete Systems / Backend Engineering Ownership and metrics (`--cover-body` or default).
   - *Paragraph 3*: Domain Architecture & Security Governance (access control, isolated query execution, distributed backends).
   - *Paragraph 4*: Testing Standards & Open Source Tooling (fuzz testing, contract tests, streaming data).
   - *Paragraph 5*: Location / timezone availability, relocation readiness and work authorization (resolved dynamically from `candidate.relocation.custom_statement` in `master_resume.yaml` if provided), and GitHub / portfolio links.
4. **Wording Reuse Guardrail**: Zero 8+ word rolling n-gram verbatim matches against baseline templates (`.agents/skills/resume-cover-letter-generator/assets/reference_cover_letter.tex`). Candidate identity tokens and custom statements are automatically exempt.
5. **No AI Slop / Fluff**: No banned words (`genuinely`, `honestly`, `actually`, `thrilled`, `passionate`, `excited`, `leverage`).
6. **No Duration Language**: No `"four years"`, `"years of experience"`, `"5+ years"`.
7. **No Em Dashes**: Clean ASCII separators (`--`, `, `, `-`).

---

## 3. Cognitive Agent Workflow: JD Evaluation & Tailoring Pipeline

When a user provides a Job Description (JD) or company notes, follow this 6-step intelligent workflow:

```
[Job Description Provided]
           │
           ▼
[Step 1: Ingest Candidate Facts (master_resume.yaml)]
           │
           ▼
[Step 2: JD Parsing & Skillset Match Analysis]
           │
   ┌───────┴───────┐
   │               │
[Strong Match] [Critical Mismatch]
   │               │
   │               ▼
   │       [Ask User: Gaps Detected. Proceed or Abort?]
   │               │ (If Proceed)
   └───────┬───────┘
           │
           ▼
[Step 3: Granular Section Customization & Front-Running]
   ├── Front-run language in --lead-skills (e.g. TypeScript vs Go vs Rust)
   ├── Prioritize relevant experience bullets via --bullet-tags
   ├── Curate relevant projects via --include-projects / --exclude-projects
   ├── Write targeted summary via --summary or select --summary-id
   └── Craft tailored company hook via --company-notes
           │
           ▼
[Step 4: Execute Turnkey Build (resumegen build)]
           │
           ▼
[Step 5: Quality Gate & Invariant Verification (resumegen check)]
           │
           ▼
[Step 6: Report Match Breakdown & Deliver Document Links]
```

### Step 1: Ingest Candidate Facts Bank
Read `master_resume.yaml` (or `.resumegen/master_resume.yaml`) to understand the candidate's verified career history, strengths, summary archetypes, open-source projects, and skill matrix.

### Step 2: JD Parsing & Skillset Match Analysis
Parse the target Job Description to extract:
1. **Target Company & Exact Role Title** (e.g. `Senior TypeScript Engineer`, `Senior Backend Engineer (Go/Rust)`).
2. **Core Tech Stack Requirements** (e.g. TypeScript, React, Next.js, Node.js vs. Go, PostgreSQL, Kafka vs. Rust, Systems, Low Latency).
3. **Domain & Engineering Responsibilities** (e.g. developer tooling, distributed access-control, real-time data streaming).
4. **Location & Work Authorization Requirements** (e.g. Remote EU, Hybrid Berlin, US Citizen Only, Relocation support).

#### 🚨 Critical Mismatch Evaluation Gate
Compare the JD requirements against the candidate's verified skills:
- **Strong Match**: The core stack overlaps directly with candidate strengths (e.g. Go, Rust, TypeScript/Node, Distributed Systems, Cloud/Nix, Databases).
  - *Action*: Proceed immediately to Step 3 for tailored section customization.
- **Critical Mismatch**: The JD requires mandatory expertise in technologies the candidate does NOT have in `master_resume.yaml` (e.g. 5+ years Java/Spring, Swift/iOS, Ruby on Rails, C#, Hardware design) OR strict non-relocatable local residency/clearance requirements.
  - *Action*: **STOP AND ASK THE USER**. Prompt the candidate with a transparent gap breakdown:
    > "I analyzed the Job Description for **[Role] at [Company]**. There are critical stack/experience mismatches:
    > - **Required**: [e.g. 5+ years Java/Spring, AWS DynamoDB]
    > - **Candidate Bank**: [Go, Rust, TypeScript, PostgreSQL]
    > 
    > Would you like me to tailor and generate anyway (framing your transferable distributed systems background), or skip this application?"
  - Wait for candidate confirmation before proceeding.

### Step 3: Granular Section Customization & Front-Running Strategy
Tailor every section of the document to maximize relevancy for the target JD using granular CLI parameters:

| Section | Customization Strategy | CLI Options |
| :--- | :--- | :--- |
| **Summary** | Target specific domain/stack focus | `--summary "<custom_text>"` or `--summary-id "<id>"` |
| **Languages & Skills** | Front-run primary JD language | `--lead-skills "TypeScript,JavaScript,Node.js,React..."` |
| **Experience Bullets** | Prioritize bullets matching JD tags | `--bullet-tags "backend,postgres,scale"` or `frontend,react,typescript` |
| **Role Bullet Ceiling**| Keep bullet density tight & balanced | `--max-bullets-per-role 4` |
| **Projects Section** | Include only relevant projects | `--include-projects "fastkv,abel"` or `--exclude-projects "minitraycer"` |
| **Skill Categories** | Filter categories | `--include-categories "Languages,Backend & Systems"` |
| **Cover Letter Hook**| Specific company value proposition | `--company-notes "<crafted_hook>"` |
| **Cover Letter Body**| Tailored systems narrative | `--cover-body "<custom_engineering_win>"` |

#### Concrete Tailoring Examples:
- **For a TypeScript / Node / Fullstack Role**:
  - `--lead-skills "TypeScript,JavaScript,Node.js,React,Next.js,PostgreSQL,Docker"`
  - `--bullet-tags "typescript,frontend,fullstack,react"`
  - `--include-projects "elliot14a,abel"` (highlight portfolio, parser tooling; omit graphics/C++)
  - `--summary "Fullstack software engineer with deep expertise in TypeScript, React, Next.js, and backend API design."`
- **For a Go / IAM / Access Control Role**:
  - `--lead-skills "Go,PostgreSQL,Docker,Kubernetes,Redis,TypeScript,Rust"`
  - `--bullet-tags "go,postgres,iam,auth,security,scale"`
  - `--summary-id "go_iam_focus"`
  - `--include-projects "abel,ruspie,elliot14a"`
- **For a Rust / Low-Latency Systems Role**:
  - `--lead-skills "Rust,Apache Arrow,DataFusion,gRPC,PostgreSQL,Go,Linux"`
  - `--bullet-tags "rust,grpc,systems,performance,arrow"`
  - `--summary-id "backend_systems_focus"`
  - `--include-projects "ruspie,abel,minitraycer"`

### Step 4: Turnkey Build Execution
Invoke `resumegen build` with all tailored parameters:
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
  --company-notes "Ory has established the open-source standard for identity, authentication, and zero-trust authorization systems." \
  --relocation true \
  --relocation-target "Germany"
```

### Step 5: Quality Gate & Verification Audit
`resumegen check` automatically verifies:
- PDF selectability (>100 characters)
- Strict page budget (<= 2 pages resume, 1 page cover letter)
- No banned AI fluff words
- No duration language
- No em dashes
- Plagiarism guardrail against reference baseline (0 rolling 8+ word matches)

### Step 6: Presentation to User
Present a concise report highlighting:
- **Match Assessment & Tailoring Strategy**: What stack was front-run and why.
- **Section Customizations**: Bullets prioritized, projects curated, and summary selected.
- **Verification Status**: 10/10 ATS quality gate result.
- **Generated Artifacts**: Direct links to generated `.pdf` and `.tex` documents in `.resumegen/resumes/`.
