# Master Resume Schema & Guidelines (Agent-First Toolchain)

`master_resume.yaml` is the **single source of truth** for all candidate facts, metrics, skills, projects, and career history used by coding agents (Antigravity/AGY, Claude Code, Codex, Cursor).

All compiled artifacts are generated into `.resumegen/resumes/` and tracked in `.resumegen/ledger.csv`.

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
  
  # OPTIONAL: Candidate relocation, work authorization, and custom statements
  relocation:
    enabled: false                                 # Set true if actively seeking relocation
    target: "Berlin, Germany"                      # Target city/country for relocation
    header_tag: "Open to relocation to Germany"    # Optional header line text
    custom_statement: "I am based in Berlin and eligible to work in the EU without visa sponsorship."
    work_authorization: "EU Citizen"               # e.g. "EU Blue Card Eligible", "US Citizen"
    spoken_languages: "English (Fluent) · German (B2)"

# OPTIONAL: Candidate-Specific Quality Gate Overrides
custom_checks:
  verify_institution: true                         # Enable/disable education institution match
  max_resume_pages: 2                              # Maximum allowed pages for resumes
  max_cover_letter_pages: 1                        # Maximum allowed pages for cover letters
  banned_words:                                    # Custom banned filler words
    - "genuinely"
    - "honestly"
    - "actually"
    - "thrilled"
    - "passionate"
    - "excited"
    - "leverage"

summary_bank:
  - id: "backend_systems_focus"
    focus: "Backend Systems, Distributed Engines & Go/Rust"
    text: "..."

experience:
  - id: "acme_corp"
    company: "Acme Corp"
    company_url: "https://acmeworks.example.com"
    role: "Senior Backend Engineer"
    dates: "01/2024 -- present"
    location: "Berlin, Germany · distributed infrastructure"
    bullets:
      - id: "acme_lead"
        tags: ["go", "postgres", "distributed_systems"]
        text: "..."
      - id: "acme_auth"
        tags: ["security", "auth", "iam"]
        text: "..."

projects:
  - id: "fastkv"
    name: "fastkv"
    url: "https://github.com/janedoe/fastkv"
    repo_url: "https://github.com/janedoe/fastkv"
    repo_display: "github.com/janedoe/fastkv"
    stack: ["Rust", "Raft", "Tokio"]
    summary: "..."

skills:
  categories:
    - name: "Languages"
      items: ["Go", "Rust", "TypeScript", "SQL", "Python", "Bash"]
    - name: "Backend & Systems"
      items: ["gRPC", "PostgreSQL", "Redis", "Kafka", "Docker", "Kubernetes", "Linux"]

education:
  - institution: "Technical University of Munich"
    degree: "Bachelor of Science in Computer Science"
    dates: "2018 -- 2022"
    location: "Munich, Germany"
    details: "Languages: English (Fluent) · German (B2)"
```

---

## 2. Invariants & Rules for Coding Agents

1. **Exact Role Header**: Title under name in resume and cover letter headers must be strictly the role name from the JD.
2. **Clickable Links**: All project repository URLs and company URLs must be formatted as active `\href{url}{display}` hyperlinks.
3. **Education Institution**: Dynamically verified against `education` in YAML.
4. **Relocation Customization**: If `relocation.custom_statement` is provided in YAML, the renderer uses the candidate's exact wording. If `relocation.enabled` is false, no relocation text is injected.
5. **No AI Fluff or Duration Language**: Zero banned words (`genuinely`, `honestly`, `actually`, `thrilled`, `passionate`, `excited`, `leverage`) and no duration phrases (`"four years"`, `"years of experience"`).
6. **Artifact Sandboxing**: Always output to `.resumegen/resumes/` and keep `.resumegen/` gitignored.
