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

**Core Motto**: Give the agent maximum creative freedom to tailor every section while strictly enforcing deterministic quality invariants and zero hallucinations.

**Operating Mode**: Interactive Q&A and proposal flow. Never one-shot a resume. Always evaluate, propose, confirm, then build.

---

## 1. Core Architecture & Tooling

The toolchain is powered by a single unified binary: **`resumegen`** (available on `$PATH` in the Nix devShell or in `.agents/skills/resume-cover-letter-generator/scripts/resumegen`).

All generated artifacts are stored inside **`.resumegen/`**:
- `.resumegen/resumes/` - Generated `.tex` and `.pdf` documents
- `.resumegen/ledger.csv` - Local application tracking ledger

```
resumegen
├── build          # Turnkey pipeline: Render -> Compile -> Check -> Track
├── render         # Deterministic LaTeX generator with section filtering
├── compile        # Tectonic-backed PDF compiler (.tex -> .pdf)
├── check          # 10-point ATS, page budget, anti-slop, & 8-word reuse validator
├── track          # Unified dual-ledger synchronizer & query tool
└── skill          # Declarative manager: add summaries, bullets, categories to master_resume.yaml
```

---

## 2. Document Quality Invariants (Non-Negotiable)

These rules apply to every generated document without exception:

### Resume Invariants
1. **Max 2 pages** (~5,000-7,000 selectable characters). Dense. No filler.
2. **Exact role title from JD** under candidate name. Never modify or concatenate.
3. **All project URLs must be real** from `master_resume.yaml`. Never fabricate links.
4. **Every project must have a linked repo**: `(\href{repo_url}{display})`.
5. **Education institution matches** `education` in `master_resume.yaml` verbatim.

### Cover Letter Invariants
1. **Strictly 1 page**. 5 focused paragraphs.
2. **No banned words**: `genuinely`, `honestly`, `actually`, `thrilled`, `passionate`, `excited`, `leverage`.
3. **No duration language**: `"four years"`, `"years of experience"`, `"5+ years"`.
4. **No em dashes**. Use `--`, `,` or `-`.
5. **Zero 8+ word verbatim matches** against `reference_cover_letter.tex` (dynamic candidate tokens exempted).

---

## 3. Interactive Q&A Workflow

**This is the only acceptable workflow. Never build without going through Steps 1-4.**

```
[JD + Company Provided]
         │
         ▼
[Step 1: Read master_resume.yaml & Check Prior Applications]
         │
         ▼
[Step 2: JD Analysis & Gap Evaluation]
         │
   ┌─────┴─────┐
[Match]   [Critical Mismatch]
   │             │
   │         [Ask User: Proceed or Skip?]
   │             │ (if proceed)
   └──────┬──────┘
          │
          ▼
[Step 3: Draft Build Proposal → Present to User for Review]
   ├── Which summary will be used or newly authored
   ├── Per-company: which bullets selected, which excluded, and why
   ├── Which projects included/excluded
   ├── What experience descriptions will say
   └── Cover letter hook + body narrative
          │
   [User Reviews Proposal]
   ├── "Looks good" → proceed
   ├── "Change X" → revise proposal
   └── "Add a new summary / bullet" → write it, run `resumegen skill`, then confirm
          │
          ▼
[Step 4: Execute Build (resumegen build ...)]
          │
          ▼
[Step 5: Report Quality Gate Results + Artifact Links]
```

---

### Step 1: Ingest & Check History

Read `master_resume.yaml` completely. Understand all summaries, all bullet IDs, all projects, and all tags.

Then check if the company was already targeted:
```bash
resumegen track query <company>
```

---

### Step 2: JD Analysis & Gap Evaluation

Parse the JD to extract:
1. **Exact role title** (e.g. `Senior TypeScript Engineer`, `Backend Engineer (Go)`)
2. **Core tech stack** listed as required vs. nice-to-have
3. **Domain responsibilities** (e.g. IAM, developer tooling, real-time data, AI products)
4. **Location / work authorization requirements**

#### Critical Mismatch Gate
- **Strong match**: Candidate has direct experience with the JD's core stack. Proceed to Step 3.
- **Critical mismatch**: JD requires expertise the candidate lacks entirely (e.g. Java/Spring, Swift/iOS, Rails). **Stop and ask the user**:
  > "I found critical mismatches for **[Role] at [Company]**:
  > - Required: [e.g. 5+ years Java/Spring, DynamoDB]
  > - Candidate Bank: Go, Rust, TypeScript, PostgreSQL
  >
  > Proceed with transferable framing, or skip this application?"

---

### Step 3: Draft Build Proposal (Always Present This First)

Before running any build command, present a structured proposal to the user. Be specific about every decision.

**Proposal Format:**

```
## Build Proposal: [Role] at [Company]

**Target Role**: [Exact JD role title]
**Stack Front-Run**: [e.g. TypeScript, React, Node.js]

### Summary
Using: [summary_id] / New summary (to be added):
> "[summary text]"

### GaurData Experience
Selected bullets (by ID):
- [bullet_id]: "[bullet text]" (reason: matches TypeScript/React focus)
- [bullet_id]: "[bullet text]" (reason: shows ownership)

Excluded bullets (by ID):
- [bullet_id]: "[bullet text]" (reason: Rust/gRPC not relevant for this role)
- [bullet_id]: "[bullet text]" (reason: DuckDB analytical isolation not core here)

Experience description: "[italicized overview below header]"

### Factly Experience
Selected bullets: [...]
Excluded bullets: [...]
Experience description: "[...]"

### Projects
Included: [project_id, project_id]
Excluded: [project_id] (reason: not relevant to stack)

### Cover Letter
Opening hook: "[company-specific hook]"
Technical narrative: "[tailored engineering wins]"
```

Wait for user response. If they say:
- **"Looks good"** or **"proceed"** → go to Step 4.
- **"Change X"** → update the proposal and re-present it.
- **"Add a new summary for GaurData"** or **"the backend bullet doesn't match"** → author the new content, use `resumegen skill` to persist it, confirm with user, then proceed.

---

### Step 3a: Authoring New Summaries & Bullets

The agent can and should author new summaries or bullets when existing ones do not fit the target role. Rules:

1. **Zero fabrication**: Only include facts, metrics, and technologies that appear in the candidate's existing bullet bank or are verifiable from the YAML.
2. **No banned words or duration language**.
3. **Tight and specific**: No generic claims. Every sentence must map to a concrete deliverable.

To add a new summary:
```bash
resumegen skill add-summary \
  --id "typescript_platform_focus" \
  --focus "TypeScript, React & Developer Tooling" \
  --text "Fullstack engineer who built and owned the entire GaurData platform: TypeScript/Node API layer, React frontend on TanStack Start with type-safe client generation from SQL schema, and AI agent integrations with MCP streaming. Previously led open-source analytics tooling at Factly."
```

Then confirm with the user:
> "Added summary `typescript_platform_focus` to master_resume.yaml:
> *'[summary text]'*
> Using this for the build?"

---

### Step 4: Build Execution

After user confirmation of the proposal, invoke `resumegen build` with precisely the flags from the proposal:

```bash
resumegen build \
  --company "Vercel" \
  --role "Senior TypeScript Engineer" \
  --location "Remote (EU)" \
  --summary-id "typescript_platform_focus" \
  --lead-skills "TypeScript,React,Node.js,Next.js,PostgreSQL,Docker" \
  --bullet-tags "typescript,react,frontend,fullstack,mcp,ai" \
  --exclude-bullets "gaur_grpc_duckdb,gaur_auth_path,factly_ruspie_arrow" \
  --max-bullets-per-role 4 \
  --experience-summaries "gaur_data:Built and owned the full GaurData TypeScript platform;factly:Led open-source analytics frontend and streaming product integrations" \
  --include-projects "elliot14a,abel" \
  --exclude-projects "minitraycer" \
  --company-notes "Vercel defines the modern deployment and developer experience standard for frontend infrastructure at scale." \
  --cover-body "At GaurData I built the complete TypeScript and React product layer, generating type-safe client bindings from the SQL schema, composing TanStack query flows for AI streaming responses, and enforcing MCP boundary policies at the transport layer before any data was surfaced." \
  --relocation true \
  --relocation-target "Germany"
```

---

### Step 5: Report Results

After a successful build, report:
1. **Quality Gate**: All 10 ATS checks passing (or specific failures).
2. **Generated Artifacts**:
   - Resume: `.resumegen/resumes/{slug}_resume_{company}.pdf`
   - Cover Letter: `.resumegen/resumes/{slug}_cover_letter_{company}.pdf`
3. **Tailoring Summary**: 1-2 sentences on what was front-run and why.

---

## 4. Per-Stack Tailoring Reference

Use this as a guide when constructing proposals:

### TypeScript / Node.js / Fullstack Role
- **Summary**: `fullstack_systems_focus` or new authored `typescript_platform_focus`
- **Lead Skills**: `TypeScript, JavaScript, Node.js, React, Next.js, PostgreSQL, Docker`
- **Prioritize Bullets** (`--bullet-tags`): `typescript, react, frontend, fullstack, mcp, ai, tanstack`
- **Exclude Bullets** (`--exclude-bullets`): Rust/gRPC microservice bullets, DuckDB isolation bullets, low-level Arrow/DataFusion bullets
- **Projects**: `elliot14a` (portfolio), `abel` (parser tooling)
- **Omit**: `minitraycer` (graphics, C++), `ruspie` (Rust-only engine)

### Go / IAM / Access Control Role
- **Summary**: `go_iam_focus`
- **Lead Skills**: `Go, PostgreSQL, Docker, Kubernetes, Redis, TypeScript, Rust`
- **Prioritize Bullets** (`--bullet-tags`): `go, postgres, iam, auth, security, scale, query`
- **Exclude Bullets** (`--exclude-bullets`): React/TanStack/frontend bullets, MCP AI streaming bullets
- **Projects**: `abel`, `ruspie`, `elliot14a`
- **Omit**: `minitraycer`

### Rust / Low-Latency / Data Systems Role
- **Summary**: `backend_systems_focus`
- **Lead Skills**: `Rust, Apache Arrow, DataFusion, gRPC, PostgreSQL, Go, Linux`
- **Prioritize Bullets** (`--bullet-tags`): `rust, grpc, systems, performance, arrow, duckdb, backend`
- **Exclude Bullets** (`--exclude-bullets`): React/TypeScript/frontend bullets
- **Projects**: `ruspie`, `abel`, `minitraycer`

---

## 5. Declarative YAML Management (`resumegen skill`)

```bash
# Add a new summary archetype
resumegen skill add-summary --id "..." --focus "..." --text "..."

# Add a bullet to a company
resumegen skill add-bullet --company gaur_data --tags typescript,react --text "..."

# List all bullets for review
resumegen skill list

# Add a skill
resumegen skill add --category "Languages" --skill "Zig"
```

---

## 6. Configurable Quality Gate Overrides (`master_resume.yaml`)

```yaml
candidate:
  relocation:
    enabled: true
    target: "Germany"
    header_tag: "Open to relocation to Germany"
    custom_statement: "I work regularly in European hours and would welcome relocation to Germany. I am eligible for the EU Blue Card."
    work_authorization: "Eligible for EU Blue Card"

custom_checks:
  verify_institution: true
  max_resume_pages: 2
  max_cover_letter_pages: 1
  banned_words: ["genuinely", "honestly", "actually", "thrilled", "passionate", "excited", "leverage"]
```
