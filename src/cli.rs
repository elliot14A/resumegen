use crate::check::{do_check, DocType};
use crate::compile::do_compile;
use crate::init::do_init;
use crate::models::LedgerEntry;
use crate::render::do_render;
use crate::skill::{
    handle_add_bullet, handle_add_category, handle_skill_add, handle_skill_list,
    handle_skill_remove,
};
use crate::track::{get_documents_resumes_dir, save_ledger_to_csv, unify_ledgers};
use anyhow::Result;
use chrono::Local;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "resumegen")]
#[command(author = "Akshith Katkuri")]
#[command(version = "0.1.0")]
#[command(about = "Agent-First ATS Resume & Cover Letter Toolchain (for Antigravity/AGY, Claude Code, Codex)")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new resume workspace with starter templates and .resumegen/ directory
    Init {
        /// Target directory to initialize in (default: current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Overwrite existing files if present
        #[arg(short, long, default_value_t = false)]
        force: bool,
    },

    /// One-shot end-to-end build: Render -> Compile -> Check -> Track (outputs to .resumegen/resumes)
    Build {
        /// Target company name (e.g. "Ory", "Helsing", "Langfuse")
        #[arg(short, long)]
        company: String,

        /// Exact target role title from JD (e.g. "Senior Software Engineer")
        #[arg(short, long)]
        role: String,

        /// Company location / city (e.g. "Berlin, Germany")
        #[arg(long, default_value = "Berlin, Germany")]
        location: String,

        /// Path to master_resume.yaml
        #[arg(short, long, default_value = "master_resume.yaml")]
        master: PathBuf,

        /// Summary archetype ID
        #[arg(long)]
        summary_id: Option<String>,

        /// Comma-separated list of skills to prioritize in Languages
        #[arg(long)]
        lead_skills: Option<String>,

        /// Company notes string or hook intro
        #[arg(long)]
        company_notes: Option<String>,

        /// Add relocation line and Blue Card sponsorship clause
        #[arg(long, default_value_t = true)]
        relocation: bool,

        /// Relocation target city (default: Germany)
        #[arg(long, default_value = "Germany")]
        relocation_target: String,

        /// Output directory (default: .resumegen/resumes)
        #[arg(short, long, default_value = ".resumegen/resumes")]
        output_dir: PathBuf,

        /// Reference cover letter file for plagiarism guardrail
        #[arg(long, default_value = ".agents/skills/resume-cover-letter-generator/assets/reference_cover_letter.tex")]
        reference_cover: PathBuf,

        /// Max pages for resume (default: 2)
        #[arg(long, default_value_t = 2)]
        max_resume_pages: usize,
    },

    /// Render LaTeX source files from master_resume.yaml
    Render {
        /// Target company name
        #[arg(short, long)]
        company: String,

        /// Exact target role title from JD
        #[arg(short, long)]
        role: String,

        /// Company location
        #[arg(long, default_value = "Berlin, Germany")]
        location: String,

        /// Path to master_resume.yaml
        #[arg(short, long, default_value = "master_resume.yaml")]
        master: PathBuf,

        /// Summary archetype ID
        #[arg(long)]
        summary_id: Option<String>,

        /// Leading skills
        #[arg(long)]
        lead_skills: Option<String>,

        /// Company notes
        #[arg(long)]
        company_notes: Option<String>,

        /// Relocation toggle
        #[arg(long, default_value_t = true)]
        relocation: bool,

        /// Relocation target city
        #[arg(long, default_value = "Germany")]
        relocation_target: String,

        /// Output directory (default: .resumegen/resumes)
        #[arg(short, long, default_value = ".resumegen/resumes")]
        output_dir: PathBuf,
    },

    /// Compile a .tex file to .pdf using tectonic
    Compile {
        /// Path to .tex input file
        input: PathBuf,

        /// Optional output file or directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Validate ATS compliance, page constraints, rule invariants, and wording reuse
    Check {
        /// Path to compiled PDF
        pdf: PathBuf,

        /// Document type (resume, cover-letter, or auto)
        #[arg(short = 't', long, value_enum, default_value_t = DocType::Auto)]
        doc_type: DocType,

        /// Optional path to corresponding .tex file
        #[arg(long)]
        tex: Option<PathBuf>,

        /// Path to master_resume.yaml for candidate verification
        #[arg(short, long, default_value = "master_resume.yaml")]
        master: Option<PathBuf>,

        /// Reference cover letter for plagiarism check
        #[arg(short = 'r', long)]
        reference: Option<PathBuf>,

        /// Maximum allowed pages (default: 2 for resume, 1 for cover letter)
        #[arg(long)]
        max_pages: Option<usize>,
    },

    /// Track applications and sync unified ledger across ~/Documents/resumes and .resumegen
    Track {
        #[command(subcommand)]
        cmd: TrackSubcommands,
    },

    /// Declaratively manage skills, categories, and bullets in master_resume.yaml
    Skill {
        #[command(subcommand)]
        cmd: SkillSubcommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum TrackSubcommands {
    /// Synchronize and index all resumes across .resumegen and ~/Documents/resumes
    Sync,
    /// List tracked applications
    List {
        #[arg(short, long)]
        company: Option<String>,
        #[arg(short, long)]
        kind: Option<String>,
        #[arg(short = 'n', long, default_value_t = 30)]
        limit: usize,
    },
    /// Query application history for a specific company
    Query {
        company: String,
    },
    /// Record a newly built resume or cover letter
    Record {
        company: String,
        kind: String,
        file_path: PathBuf,
    },
    /// Show summary statistics
    Stats,
}

#[derive(Subcommand, Debug)]
pub enum SkillSubcommands {
    /// List all skills grouped by category
    List,
    /// Add a skill to a category
    Add {
        #[arg(short, long)]
        category: String,
        #[arg(short, long)]
        skill: String,
    },
    /// Remove a skill
    Remove {
        #[arg(short, long)]
        skill: String,
        #[arg(short, long)]
        category: Option<String>,
    },
    /// Add a new skill category
    AddCategory {
        name: String,
        #[arg(short, long)]
        skills: Option<String>,
    },
    /// Add an experience bullet
    AddBullet {
        #[arg(short, long)]
        company: String,
        #[arg(short, long)]
        tags: String,
        #[arg(short, long)]
        text: String,
    },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path, force } => {
            do_init(&path, force)?;
        }

        Commands::Build {
            company,
            role,
            location,
            master,
            summary_id,
            lead_skills,
            company_notes,
            relocation,
            relocation_target,
            output_dir,
            reference_cover,
            max_resume_pages,
        } => {
            println!("\n{}", "Starting Turnkey Build Pipeline...".cyan().bold());
            println!("  Target: {} | {} ({})", company.bold(), role.bold(), location);

            // Step 1: Render
            println!("\n{} Step 1: Rendering LaTeX sources...", "[1/4]".bold());
            let (resume_tex, cover_tex) = do_render(
                &company,
                &role,
                &location,
                &master,
                summary_id.as_deref(),
                lead_skills.as_deref(),
                company_notes.as_deref(),
                relocation,
                &relocation_target,
                &output_dir,
            )?;
            println!("  [OK] Rendered {}", resume_tex.display());
            println!("  [OK] Rendered {}", cover_tex.display());

            // Step 2: Compile
            println!("\n{} Step 2: Compiling to PDF via tectonic...", "[2/4]".bold());
            let resume_pdf = do_compile(&resume_tex, None)?;
            let cover_pdf = do_compile(&cover_tex, None)?;

            // Step 3: Check
            println!("\n{} Step 3: Validating ATS & wording guardrails...", "[3/4]".bold());
            let resume_ok = do_check(&resume_pdf, DocType::Resume, Some(&resume_tex), Some(&master), None, Some(max_resume_pages))?;
            let cover_ok = do_check(&cover_pdf, DocType::CoverLetter, Some(&cover_tex), Some(&master), Some(&reference_cover), Some(1))?;

            if !resume_ok || !cover_ok {
                eprintln!("\n{} Validation checks failed! Aborting ledger recording.", "[FAIL]".red().bold());
                std::process::exit(1);
            }

            // Step 4: Track
            println!("\n{} Step 4: Recording to unified application ledger...", "[4/4]".bold());
            let mut entries = unify_ledgers()?;
            let date = Local::now().format("%Y-%m-%d").to_string();

            entries.push(LedgerEntry {
                filed_on: date.clone(),
                company: company.trim().to_lowercase(),
                kind: "resume".to_string(),
                original_name: resume_pdf.file_name().unwrap().to_string_lossy().to_string(),
                stored_path: resume_pdf.to_string_lossy().to_string(),
            });
            entries.push(LedgerEntry {
                filed_on: date.clone(),
                company: company.trim().to_lowercase(),
                kind: "cover".to_string(),
                original_name: cover_pdf.file_name().unwrap().to_string_lossy().to_string(),
                stored_path: cover_pdf.to_string_lossy().to_string(),
            });

            entries.sort_by(|a, b| b.filed_on.cmp(&a.filed_on).then_with(|| a.company.cmp(&b.company)));
            let _ = save_ledger_to_csv(&entries, &get_documents_resumes_dir().join("ledger.csv"));
            let _ = save_ledger_to_csv(&entries, &PathBuf::from(".resumegen/ledger.csv"));

            println!("\n{}", "Turnkey Build COMPLETE!".green().bold());
            println!("  Resume PDF       : {}", resume_pdf.display().to_string().cyan().bold());
            println!("  Cover Letter PDF : {}", cover_pdf.display().to_string().yellow().bold());
            println!("  Ledger Updated   : {} entries indexed across repositories\n", entries.len());
        }

        Commands::Render {
            company,
            role,
            location,
            master,
            summary_id,
            lead_skills,
            company_notes,
            relocation,
            relocation_target,
            output_dir,
        } => {
            let (r, c) = do_render(
                &company,
                &role,
                &location,
                &master,
                summary_id.as_deref(),
                lead_skills.as_deref(),
                company_notes.as_deref(),
                relocation,
                &relocation_target,
                &output_dir,
            )?;
            println!("{} Rendered resume LaTeX -> {}", "[OK]".green().bold(), r.display());
            println!("{} Rendered cover letter LaTeX -> {}", "[OK]".green().bold(), c.display());
        }

        Commands::Compile { input, output } => {
            do_compile(&input, output.as_deref())?;
        }

        Commands::Check {
            pdf,
            doc_type,
            tex,
            master,
            reference,
            max_pages,
        } => {
            let ok = do_check(&pdf, doc_type, tex.as_deref(), master.as_deref(), reference.as_deref(), max_pages)?;
            if !ok {
                std::process::exit(1);
            }
        }

        Commands::Track { cmd } => match cmd {
            TrackSubcommands::Sync => {
                println!("{} Synchronizing unified ledger across ~/Documents/resumes and .resumegen...", "[INFO]".blue().bold());
                let entries = unify_ledgers()?;
                println!(
                    "{} Unified ledger updated! Total indexed documents: {}",
                    "[SUCCESS]".green().bold(),
                    entries.len().to_string().bold()
                );
            }

            TrackSubcommands::List { company, kind, limit } => {
                let entries = unify_ledgers()?;
                let filter_comp = company.map(|c| c.to_lowercase());
                let filter_k = kind.map(|k| k.to_lowercase());

                let filtered: Vec<&LedgerEntry> = entries
                    .iter()
                    .filter(|e| {
                        if let Some(ref c) = filter_comp {
                            if !e.company.contains(c) { return false; }
                        }
                        if let Some(ref k) = filter_k {
                            if !e.kind.contains(k) { return false; }
                        }
                        true
                    })
                    .take(limit)
                    .collect();

                println!("\n{}", "=========================================================================================".bold());
                println!(
                    " {:<12} | {:<22} | {:<8} | {:<48}",
                    "FILED ON".bold(),
                    "COMPANY".bold(),
                    "KIND".bold(),
                    "STORED PATH".bold()
                );
                println!("{}", "-----------------------------------------------------------------------------------------".bold());

                for e in &filtered {
                    let kind_c = if e.kind == "resume" { e.kind.cyan() } else { e.kind.yellow() };
                    println!(" {:<12} | {:<22} | {:<8} | {}", e.filed_on, e.company.green().bold(), kind_c, e.stored_path);
                }
                println!("{}\n", "=========================================================================================".bold());
                println!("Showing {} of {} tracked entries.", filtered.len(), entries.len());
            }

            TrackSubcommands::Query { company } => {
                let entries = unify_ledgers()?;
                let target = company.trim().to_lowercase();
                let matches: Vec<&LedgerEntry> = entries.iter().filter(|e| e.company == target || e.company.contains(&target)).collect();

                if matches.is_empty() {
                    println!("{} No existing applications found for '{}'. Ready to build!", "[AVAILABLE]".green().bold(), company);
                } else {
                    println!("{} Found {} existing document(s) for '{}':", "[FOUND]".yellow().bold(), matches.len(), company.bold());
                    for m in matches {
                        println!("  - [{}] {} ({}) -> {}", m.filed_on, m.kind.cyan(), m.original_name, m.stored_path);
                    }
                }
            }

            TrackSubcommands::Record { company, kind, file_path } => {
                let mut entries = unify_ledgers()?;
                let date = Local::now().format("%Y-%m-%d").to_string();
                let file_name = file_path.file_name().and_then(|s| s.to_str()).unwrap_or("file.pdf");

                entries.push(LedgerEntry {
                    filed_on: date.clone(),
                    company: company.trim().to_lowercase(),
                    kind: kind.trim().to_lowercase(),
                    original_name: file_name.to_string(),
                    stored_path: file_path.to_string_lossy().to_string(),
                });
                entries.sort_by(|a, b| b.filed_on.cmp(&a.filed_on).then_with(|| a.company.cmp(&b.company)));

                let doc_csv = get_documents_resumes_dir().join("ledger.csv");
                let local_csv = PathBuf::from(".resumegen/ledger.csv");
                save_ledger_to_csv(&entries, &doc_csv)?;
                save_ledger_to_csv(&entries, &local_csv)?;

                println!("{} Recorded {} for '{}' -> {}", "[PASS]".green().bold(), kind.bold(), company.bold(), file_path.display());
            }

            TrackSubcommands::Stats => {
                let entries = unify_ledgers()?;
                let mut companies = HashSet::new();
                let mut resume_count = 0;
                let mut cover_count = 0;

                for e in &entries {
                    companies.insert(e.company.clone());
                    if e.kind == "resume" { resume_count += 1; }
                    else if e.kind == "cover" { cover_count += 1; }
                }

                println!("\n{}", "==========================================".bold());
                println!("  Resume & Application Pipeline Stats");
                println!("{}\n", "==========================================".bold());
                println!("  Total Tracked Applications : {}", companies.len().to_string().green().bold());
                println!("  Total Resumes Built        : {}", resume_count.to_string().cyan().bold());
                println!("  Total Cover Letters Built  : {}", cover_count.to_string().yellow().bold());
                println!("  Total Document Files       : {}", entries.len().to_string().bold());
                if let Some(latest) = entries.first() {
                    println!("  Latest Filed Application   : {} ({}, {})", latest.company.bold(), latest.kind, latest.filed_on);
                }
                println!("{}\n", "==========================================".bold());
            }
        },

        Commands::Skill { cmd } => match cmd {
            SkillSubcommands::List => handle_skill_list()?,
            SkillSubcommands::Add { category, skill } => handle_skill_add(&category, &skill)?,
            SkillSubcommands::Remove { skill, category } => handle_skill_remove(&skill, category.as_deref())?,
            SkillSubcommands::AddCategory { name, skills } => handle_add_category(&name, skills.as_deref())?,
            SkillSubcommands::AddBullet { company, tags, text } => handle_add_bullet(&company, &tags, &text)?,
        },
    }

    Ok(())
}
