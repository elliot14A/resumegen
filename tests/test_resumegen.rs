use resumegen::check::{clean_latex_to_plain_text, is_boilerplate_ngram, tokenize_words};
use resumegen::models::MasterResume;
use resumegen::render::{do_render, escape_latex, resolve_master_resume_path, sanitize_slug, RenderOptions};
use std::fs;
use std::path::Path;

#[test]
fn test_escape_latex_special_chars() {
    let raw = "C++ & Rust: 100% #1 {foo_bar} $50 ~100 ^test · item — dash";
    let escaped = escape_latex(raw);

    assert!(escaped.contains(r"\&"));
    assert!(escaped.contains(r"\%"));
    assert!(escaped.contains(r"\#"));
    assert!(escaped.contains(r"\_"));
    assert!(escaped.contains(r"\{"));
    assert!(escaped.contains(r"\}"));
    assert!(escaped.contains(r"\$"));
    assert!(escaped.contains(r"\textasciitilde{}"));
    assert!(escaped.contains(r"\textasciicircum{}"));
    assert!(escaped.contains(r"$\cdot$ "));
    assert!(!escaped.contains('—')); // No em dashes
}

#[test]
fn test_sanitize_slug() {
    assert_eq!(sanitize_slug("Jane Doe"), "jane_doe");
    assert_eq!(sanitize_slug("Acme Corp! @Berlin"), "acme_corp_berlin");
    assert_eq!(sanitize_slug("---Stripe---"), "stripe");
}

#[test]
fn test_master_resume_example_deserialization() {
    let example_path = Path::new("master_resume.example.yaml");
    assert!(example_path.exists(), "master_resume.example.yaml must exist");

    let content = fs::read_to_string(example_path).expect("Failed to read example YAML");
    let resume: MasterResume = serde_yaml::from_str(&content).expect("Failed to parse example YAML");

    assert_eq!(resume.candidate.name, "Jane Doe");
    assert!(!resume.experience.is_empty());
    assert!(!resume.projects.is_empty());
    assert!(!resume.skills.categories.is_empty());
    assert!(!resume.education.is_empty());
}

#[test]
fn test_resolve_master_resume_path() {
    let resolved = resolve_master_resume_path(None);
    assert!(resolved.exists(), "Resolved master resume path must exist");
}

#[test]
fn test_tokenize_and_clean_latex() {
    let tex = r"Dear \textbf{Helsing} team, I am applying for the \href{https://helsing.ai}{Helsing} role.";
    let cleaned = clean_latex_to_plain_text(tex);
    let tokens = tokenize_words(&cleaned);

    assert!(tokens.contains(&"dear".to_string()));
    assert!(tokens.contains(&"helsing".to_string()));
    assert!(tokens.contains(&"team".to_string()));
    assert!(tokens.contains(&"applying".to_string()));
}

#[test]
fn test_boilerplate_ngram_filter() {
    let ngram = vec![
        "i".to_string(),
        "am".to_string(),
        "applying".to_string(),
        "for".to_string(),
        "the".to_string(),
        "senior".to_string(),
        "software".to_string(),
        "role".to_string(),
    ];
    let dynamic_tokens = vec!["helsing".to_string()];

    assert!(is_boilerplate_ngram(&ngram, &dynamic_tokens));
}

#[test]
fn test_render_with_custom_options() {
    let temp_out = Path::new(".resumegen/test_output");
    let (resume_tex, cover_tex) = do_render(RenderOptions {
        company: "TestCorp",
        role: "Senior Backend Engineer",
        location: "Berlin, Germany",
        master_path: Path::new("master_resume.example.yaml"),
        summary_id: None,
        summary: Some("Direct custom summary for testing."),
        lead_skills: Some("Go,Rust"),
        bullet_tags: Some("postgres,go"),
        max_bullets_per_role: Some(2),
        include_projects: Some("fastkv"),
        exclude_projects: None,
        include_categories: Some("Languages,Backend & Systems"),
        exclude_categories: None,
        company_notes: Some("TestCorp is leading high scale systems."),
        cover_body: Some("Custom test body for cover letter."),
        relocation: true,
        relocation_target: "Germany",
        output_dir: temp_out,
    }).expect("Rendering should succeed");

    assert!(resume_tex.exists());
    assert!(cover_tex.exists());

    let resume_content = fs::read_to_string(&resume_tex).unwrap();
    assert!(resume_content.contains("Direct custom summary for testing."));
    assert!(resume_content.contains("fastkv"));

    let cover_content = fs::read_to_string(&cover_tex).unwrap();
    assert!(cover_content.contains("TestCorp is leading high scale systems."));
    assert!(cover_content.contains("Custom test body for cover letter."));

    let _ = fs::remove_dir_all(temp_out);
}
