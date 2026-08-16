use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn do_init(target_path: &Path, force: bool) -> Result<()> {
    fs::create_dir_all(target_path)?;
    fs::create_dir_all(target_path.join("assets"))?;
    fs::create_dir_all(target_path.join(".resumegen/resumes"))?;

    let master_dest = target_path.join("master_resume.yaml");
    if master_dest.exists() && !force {
        println!("{} master_resume.yaml already exists. Use --force to overwrite.", "[SKIP]".yellow().bold());
    } else {
        let starter_yaml = r#"candidate:
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
  relocation:
    default_target: "Berlin, Germany"
    sponsorship_needed: false
    blue_card_eligible: true
    spoken_languages: "English (Fluent) · German (B2)"

summary_bank:
  - id: "backend_systems_focus"
    focus: "Backend Systems, Distributed Engines & Go/Rust"
    text: "Senior software engineer with deep production experience in Go, Rust, PostgreSQL, and distributed data systems. Passionate about owning products end-to-end and setting high engineering standards."

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
        text: "Owned core backend microservices in Go, cutting API response latency from 450ms to 45ms."
      - id: "acme_auth"
        tags: ["security", "auth", "iam"]
        text: "Designed zero-trust authorization pipelines evaluating request contexts directly in the query execution path."

projects:
  - id: "fastkv"
    name: "fastkv"
    url: "https://github.com/janedoe/fastkv"
    repo_url: "https://github.com/janedoe/fastkv"
    repo_display: "github.com/janedoe/fastkv"
    stack: ["Rust", "Raft", "Tokio"]
    summary: "author. High-performance distributed key-value store in Rust implementing the Raft consensus algorithm with asynchronous disk I/O."

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
"#;
        fs::write(&master_dest, starter_yaml)?;
        println!("{} Created starter master_resume.yaml", "[OK]".green().bold());
    }

    println!("\n{} Project initialized successfully in {}!", "[DONE]".green().bold(), target_path.display());
    println!("  Next step: Edit master_resume.yaml, then run:\n");
    println!("    resumegen build --company \"Google\" --role \"Senior Software Engineer\"\n");

    Ok(())
}
