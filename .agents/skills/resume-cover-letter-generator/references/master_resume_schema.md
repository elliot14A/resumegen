# Master Resume Schema & Guidelines (Agent-First Toolchain)

`master_resume.yaml` is the **single source of truth** for all candidate facts, metrics, skills, projects, and career history used by coding agents (Antigravity/AGY, Claude Code, Codex, Cursor).

All compiled artifacts are generated into `.resumegen/resumes/` and tracked in `.resumegen/ledger.csv`.

> **Note for Agents**: You operate in an **interactive Q&A mode**. Always read this schema fully, propose what you will build per section (which bullets in/out, which summary, which projects), and wait for human confirmation before invoking `resumegen build`. See `SKILL.md` for the full workflow.

---

## 1. Top-Level Structure

```yaml
candidate:
  name: "Jane Doe"
  title: "Senior Software Engineer"
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
    enabled: true
    target: "Germany"
    header_tag: "Open to relocation to Germany"
    custom_statement: "I work regularly in European hours and would welcome relocation to Germany. I am eligible for the EU Blue Card."
    work_authorization: "EU Blue Card Eligible"
    spoken_languages: "English (Fluent) · German (B2)"

# OPTIONAL: Quality Gate Overrides
custom_checks:
  verify_institution: true
  max_resume_pages: 2
  max_cover_letter_pages: 1
  banned_words: ["genuinely", "honestly", "actually", "thrilled", "passionate", "excited", "leverage"]
  custom_boilerplate_keywords: ["berlin", "work authorization"]
```

---

## 2. Summary Bank

Multiple summary archetypes for different target roles. Agents select via `--summary-id` or author new ones via `resumegen skill add-summary`.

```yaml
summary_bank:
  - id: "backend_systems_focus"
    focus: "Backend Systems, Distributed Engines & Go/Rust"
    text: "Senior backend engineer with deep production experience in Go, Rust, PostgreSQL, and distributed data systems."

  - id: "go_iam_focus"
    focus: "Go Backend, Access Control & Open Source"
    text: "Senior backend engineer specializing in Go microservices, access control governance, and low-latency PostgreSQL profiling."

  - id: "typescript_platform_focus"
    focus: "TypeScript, React & Developer Tooling"
    text: "Fullstack engineer who built and owned the entire product platform: TypeScript/Node API layer, React frontend, and AI agent integrations."
```

**Adding a new summary** (agent command):
```bash
resumegen skill add-summary \
  --id "ai_platform_focus" \
  --focus "AI Products, LLM Integration & Data Platforms" \
  --text "Backend and AI systems engineer who built production LLM integration layers..."
```

---

## 3. Experience

Each experience entry has a stable `id` for CLI reference, a pool of bullets with `id` and `tags`, and optional per-focus `summaries` for context-specific descriptions.

```yaml
experience:
  - id: "acme_corp"
    company: "Acme Corp"
    company_url: "https://acme.example.com"
    role: "Senior Backend Engineer"
    dates: "01/2024 -- present"
    location: "Berlin, Germany"
    
    # Default experience description (appears under company header)
    summary: "Built and operated core backend microservices in Go."
    
    # Focus-keyed descriptions (matched against --bullet-tags at render time)
    summaries:
      backend: "Built and operated high-throughput Go microservices with PostgreSQL and Redis."
      fullstack: "Owned backend API layer and TypeScript/React frontend for customer-facing platform."
    
    bullets:
      - id: "acme_go_latency"           # Stable ID for --exclude-bullets
        tags: ["go", "postgres", "scale", "backend"]
        text: "Owned core backend microservices in Go, cutting API response latency from 450ms to 45ms."
      
      - id: "acme_auth_pipeline"
        tags: ["security", "auth", "iam", "go"]
        text: "Designed zero-trust authorization pipelines evaluating request contexts in the query path."
      
      - id: "acme_react_frontend"
        tags: ["typescript", "react", "frontend"]
        text: "Built React dashboard with type-safe bindings generated from SQL schema."
```

### Per-Company Bullet Control (CLI)

```bash
# Prioritize backend bullets, exclude frontend bullet
resumegen build \
  --bullet-tags "go,postgres,scale,backend" \
  --exclude-bullets "acme_react_frontend" \
  --max-bullets-per-role 3

# Prioritize frontend bullets, exclude low-level backend bullet
resumegen build \
  --bullet-tags "typescript,react,frontend" \
  --exclude-bullets "acme_auth_pipeline,acme_go_latency" \
  --max-bullets-per-role 3
```

### Multi-Role Progression

For companies with multiple roles, use `roles_history` instead of a flat `bullets` list:

```yaml
  - id: "factly"
    company: "Factly Research & Media"
    company_url: "https://factly.in"
    role: "Senior Backend Developer & Project Lead"
    dates: "09/2022 -- 01/2026"
    location: "Hyderabad, India"
    summary: "Led backend engineering across Go and Rust data analytics products."
    roles_history:
      - role: "Senior Backend Developer & Project Lead"
        dates: "04/2024 -- 01/2026"
        summary: "Led Gopie Go analytics platform and open-source data engine contributions."
        bullets:
          - id: "factly_gopie_latency"
            tags: ["go", "postgres", "scale", "backend", "query"]
            text: "Led Gopie, cutting query latency from 5s to under 1s via PostgreSQL schema profiling."
      - role: "Backend Developer"
        dates: "02/2023 -- 03/2024"
        bullets:
          - id: "factly_ruspie_rust"
            tags: ["rust", "arrow", "datafusion", "backend"]
            text: "Built ruspie from scratch in Rust on Apache Arrow/DataFusion."
```

---

## 4. Projects

```yaml
projects:
  - id: "fastkv"                          # Stable ID for --include-projects / --exclude-projects
    name: "fastkv"
    url: "https://github.com/janedoe/fastkv"
    repo_url: "https://github.com/janedoe/fastkv"
    repo_display: "github.com/janedoe/fastkv"
    stack: ["Rust", "Raft", "Tokio"]
    summary: "author. High-performance distributed key-value store in Rust implementing Raft consensus with async disk I/O."
```

---

## 5. Skills

```yaml
skills:
  categories:
    - name: "Languages"
      items: ["Go", "Rust", "TypeScript", "SQL", "Python", "Bash"]
    - name: "Backend & Systems"
      items: ["gRPC", "PostgreSQL", "Redis", "Kafka", "Docker", "Kubernetes", "Linux"]
```

---

## 6. Education

```yaml
education:
  - institution: "Technical University of Munich"
    degree: "Bachelor of Science in Computer Science"
    dates: "2018 -- 2022"
    location: "Munich, Germany"
    details: "Languages: English (Fluent) · German (B2)"
```

---

## 7. Invariants & Rules for Coding Agents

1. **Zero fabrication**: Only include facts, metrics, and URLs that appear in `master_resume.yaml`. Never invent bullet text, project links, institution names, or metrics.
2. **Exact role title**: The header title must match the JD role title verbatim. Never modify or concatenate.
3. **Relocation resolution**: If `relocation.custom_statement` is set, the renderer uses the candidate's exact wording. If `relocation.enabled` is false, no relocation text is injected.
4. **Bullet IDs are stable**: When instructing the agent to exclude bullets, always use the `id` field from `master_resume.yaml`, never the text.
5. **Propose before building**: Always draft and present a build proposal before invoking `resumegen build`. Wait for human confirmation.
6. **Artifact sandboxing**: Always output to `.resumegen/resumes/` and keep `.resumegen/` gitignored.
