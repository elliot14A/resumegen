use resumegen::check::{clean_latex_to_plain_text, is_boilerplate_ngram, tokenize_words};
use resumegen::models::MasterResume;
use resumegen::render::{escape_latex, resolve_master_resume_path, sanitize_slug};
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
