use crate::models::MasterResume;
use anyhow::Result;
use colored::Colorize;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocType {
    Resume,
    CoverLetter,
    Auto,
}

pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub details: String,
}

pub fn extract_text_from_pdf(pdf_path: &Path) -> Result<String> {
    let output = Command::new("pdftotext")
        .arg(pdf_path)
        .arg("-")
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run pdftotext on {}: {}", pdf_path.display(), e))?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("pdftotext error: {}", String::from_utf8_lossy(&output.stderr)));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn clean_latex_to_plain_text(tex: &str) -> String {
    let re_comments = Regex::new(r"(?m)%.*$").unwrap();
    let no_comments = re_comments.replace_all(tex, "");
    let re_commands = Regex::new(r"\\[a-zA-Z]+(\[[^\]]*\])?(\{([^}]*)\})?").unwrap();
    let text = re_commands.replace_all(&no_comments, "$3");
    let re_braces = Regex::new(r"[{}\\]").unwrap();
    re_braces.replace_all(&text, " ").to_string()
}

pub fn tokenize_words(text: &str) -> Vec<String> {
    let re_word = Regex::new(r"[a-zA-Z0-9]+").unwrap();
    re_word.find_iter(text).map(|m| m.as_str().to_lowercase()).collect()
}

pub fn is_boilerplate_ngram(ngram: &[String], dynamic_keywords: &[String]) -> bool {
    let joined = ngram.join(" ");
    let static_keywords = [
        "dear", "team", "application", "position", "role", "regards", "sincerely",
        "best regards", "curriculum vitae", "resume", "hours", "timezone", "time zone",
        "work regularly", "available to work", "relocation", "relocate", "sponsorship",
        "visa", "work authorization", "blue card", "open-source", "repositories", "portfolio",
    ];

    for kw in static_keywords {
        if joined.contains(kw) {
            return true;
        }
    }
    for kw in dynamic_keywords {
        if !kw.is_empty() && joined.contains(kw) {
            return true;
        }
    }
    false
}

pub fn check_unescaped_latex_chars(tex_content: &str) -> Vec<(usize, char)> {
    let mut violations = Vec::new();
    let lines: Vec<&str> = tex_content.lines().collect();
    let mut in_tabular = false;

    for (line_idx, raw_line) in lines.iter().enumerate() {
        let line_num = line_idx + 1;
        let line = match raw_line.find('%') {
            Some(idx) => &raw_line[..idx],
            None => *raw_line,
        };

        if line.contains(r"\begin{tabular") { in_tabular = true; }
        if line.contains(r"\end{tabular") { in_tabular = false; }
        if line.contains(r"\href") || line.contains(r"\url") { continue; }

        let chars: Vec<char> = line.chars().collect();
        for i in 0..chars.len() {
            let c = chars[i];
            let is_escaped = i > 0 && chars[i - 1] == '\\';

            if c == '&' && !is_escaped && !in_tabular {
                violations.push((line_num, '&'));
            }
            if c == '#' && !is_escaped {
                let next_is_digit = i + 1 < chars.len() && chars[i + 1].is_ascii_digit();
                if !next_is_digit {
                    violations.push((line_num, '#'));
                }
            }
        }
    }
    violations
}

pub fn do_check(
    pdf: &Path,
    doc_type: DocType,
    tex: Option<&Path>,
    master: Option<&Path>,
    reference: Option<&Path>,
    max_pages: Option<usize>,
) -> Result<bool> {
    if !pdf.exists() {
        return Err(anyhow::anyhow!("PDF file not found: {}", pdf.display()));
    }

    let pdf_bytes = fs::read(pdf)?;
    let mut results: Vec<CheckResult> = Vec::new();

    // 1. PDF Header
    let is_valid_pdf = pdf_bytes.starts_with(b"%PDF-");
    results.push(CheckResult {
        name: "PDF Format Header".to_string(),
        passed: is_valid_pdf,
        details: if is_valid_pdf { "Valid PDF header found".to_string() } else { "Missing %PDF- header".to_string() },
    });

    // 2. pdftotext
    let extracted_text = match extract_text_from_pdf(pdf) {
        Ok(t) => {
            let passed = t.trim().len() >= 100;
            results.push(CheckResult {
                name: "Text Selectability".to_string(),
                passed,
                details: format!("Extracted {} selectable characters", t.trim().len()),
            });
            t
        }
        Err(e) => {
            results.push(CheckResult {
                name: "Text Selectability".to_string(),
                passed: false,
                details: format!("pdftotext failed: {}", e),
            });
            String::new()
        }
    };

    let file_name_lower = pdf.file_name().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
    let resolved_doc_type = match doc_type {
        DocType::Resume => DocType::Resume,
        DocType::CoverLetter => DocType::CoverLetter,
        DocType::Auto => {
            if file_name_lower.contains("cover") || file_name_lower.contains("letter") {
                DocType::CoverLetter
            } else {
                DocType::Resume
            }
        }
    };

    // Load master_resume if provided for candidate-agnostic checks
    let parsed_master: Option<MasterResume> = master
        .and_then(|p| fs::read_to_string(p).ok())
        .and_then(|c| serde_yaml::from_str(&c).ok());

    let custom_checks = parsed_master.as_ref().and_then(|m| m.custom_checks.as_ref());

    // 3. Page Count
    let page_count = if extracted_text.is_empty() {
        0
    } else {
        let ff_count = extracted_text.matches('\x0c').count();
        if ff_count > 0 { ff_count } else { 1 }
    };

    let default_max = match resolved_doc_type {
        DocType::CoverLetter => custom_checks.and_then(|c| c.max_cover_letter_pages).unwrap_or(1),
        DocType::Resume | DocType::Auto => custom_checks.and_then(|c| c.max_resume_pages).unwrap_or(2),
    };
    let max_allowed = max_pages.unwrap_or(default_max);

    let page_ok = page_count > 0 && page_count <= max_allowed;
    results.push(CheckResult {
        name: "Page Count Constraint".to_string(),
        passed: page_ok,
        details: format!("Document has {} page(s) (Max allowed: {})", page_count, max_allowed),
    });

    // 4. Banned Fluff (Candidate-configurable or defaults)
    let default_banned = vec![
        "genuinely".to_string(), "honestly".to_string(), "actually".to_string(),
        "thrilled".to_string(), "passionate".to_string(), "excited".to_string(), "leverage".to_string()
    ];
    let banned_list = custom_checks.and_then(|c| c.banned_words.as_ref()).unwrap_or(&default_banned);
    let lower_text = extracted_text.to_lowercase();
    let mut banned_found = Vec::new();
    for bw in banned_list {
        let pat = format!(r"\b{}\b", bw.to_lowercase());
        if let Ok(re) = Regex::new(&pat) {
            if re.is_match(&lower_text) {
                banned_found.push(bw.clone());
            }
        }
    }
    let banned_ok = banned_found.is_empty();
    results.push(CheckResult {
        name: "No Banned AI Slop".to_string(),
        passed: banned_ok,
        details: if banned_ok { "No banned filler words".to_string() } else { format!("Found: {}", banned_found.join(", ")) },
    });

    // 5. Duration Language
    let re_dur = Regex::new(r"(?i)\b(\d+|one|two|three|four|five|six|seven|eight|nine|ten)\s+years\s+of\s+experience\b").unwrap();
    let mut dur_found = Vec::new();
    for m in re_dur.find_iter(&extracted_text) {
        dur_found.push(m.as_str().to_string());
    }
    let dur_ok = dur_found.is_empty();
    results.push(CheckResult {
        name: "No Duration Language".to_string(),
        passed: dur_ok,
        details: if dur_ok { "No duration phrases found".to_string() } else { format!("Found: {}", dur_found.join("; ")) },
    });

    // 6. Em dashes
    let em_count = extracted_text.matches('\u{2014}').count();
    results.push(CheckResult {
        name: "No Em Dashes".to_string(),
        passed: em_count == 0,
        details: if em_count == 0 { "No em dashes found".to_string() } else { format!("Found {} em dash(es)", em_count) },
    });

    // 7. Verified Facts: Institution Name Accuracy (optional/dynamic)
    let verify_inst = custom_checks.and_then(|c| c.verify_institution).unwrap_or(true);
    if resolved_doc_type == DocType::Resume && verify_inst {
        if let Some(ref m) = parsed_master {
            if let Some(first_edu) = m.education.first() {
                let first_word = first_edu.institution.split_whitespace().next().unwrap_or("").to_lowercase();
                let ok = lower_text.contains(&first_word);
                results.push(CheckResult {
                    name: "Institution Name Accuracy".to_string(),
                    passed: ok,
                    details: if ok {
                        format!("Verified: {}", first_edu.institution)
                    } else {
                        format!("Failed to verify institution: {}", first_edu.institution)
                    },
                });
            }
        }
    }

    // 8. Verified Candidate Contact & Identity Info
    let (cand_name, cand_email, identity_ok) = if let Some(ref m) = parsed_master {
        let name_match = lower_text.contains(&m.candidate.name.to_lowercase());
        let email_match = lower_text.contains(&m.candidate.email.to_lowercase());
        (m.candidate.name.clone(), m.candidate.email.clone(), name_match && email_match)
    } else {
        let has_email = lower_text.contains("@") && lower_text.contains(".");
        ("Candidate Name".to_string(), "Candidate Email".to_string(), has_email)
    };

    results.push(CheckResult {
        name: "Candidate Identity & Links".to_string(),
        passed: identity_ok,
        details: if identity_ok {
            format!("Verified: {} and contact links present", cand_name)
        } else {
            format!("Missing candidate identity (Name: {}, Email: {})", cand_name, cand_email)
        },
    });

    // 9. Dynamic Wording Reuse (8-word guardrail)
    if resolved_doc_type == DocType::CoverLetter || reference.is_some() {
        if let Some(ref_path) = reference {
            if ref_path.exists() {
                let is_same = fs::canonicalize(ref_path).ok() == fs::canonicalize(pdf).ok()
                    || fs::canonicalize(ref_path).ok() == tex.and_then(|t| fs::canonicalize(t).ok());

                if is_same {
                    results.push(CheckResult {
                        name: "Cover Letter Wording Reuse".to_string(),
                        passed: true,
                        details: "Self-reference mode: skipped plagiarism self-check".to_string(),
                    });
                } else {
                    let ref_text = if ref_path.extension().map_or(false, |e| e == "pdf") {
                        extract_text_from_pdf(ref_path)?
                    } else {
                        clean_latex_to_plain_text(&fs::read_to_string(ref_path)?)
                    };

                    // Collect dynamic candidate boilerplate tokens
                    let mut dyn_keywords = Vec::new();
                    if let Some(ref m) = parsed_master {
                        for word in m.candidate.name.split_whitespace() {
                            dyn_keywords.push(word.to_lowercase());
                        }
                        dyn_keywords.push(m.candidate.email.to_lowercase());
                        dyn_keywords.push(m.candidate.phone.to_lowercase());
                        dyn_keywords.push(m.candidate.links.portfolio_display.to_lowercase());
                        dyn_keywords.push(m.candidate.links.github_display.to_lowercase());
                        dyn_keywords.push(m.candidate.links.linkedin_display.to_lowercase());
                        if let Some(ref reloc) = m.candidate.relocation {
                            if let Some(ref stmt) = reloc.custom_statement {
                                for w in stmt.split_whitespace() {
                                    dyn_keywords.push(w.to_lowercase());
                                }
                            }
                            if let Some(ref tgt) = reloc.target.as_ref().or(reloc.default_target.as_ref()) {
                                dyn_keywords.push(tgt.to_lowercase());
                            }
                        }
                        if let Some(custom_bp) = custom_checks.and_then(|c| c.custom_boilerplate_keywords.as_ref()) {
                            for kw in custom_bp {
                                dyn_keywords.push(kw.to_lowercase());
                            }
                        }
                    }

                    let ref_words = tokenize_words(&ref_text);
                    let gen_words = tokenize_words(&extracted_text);
                    let n = 8;

                    let mut ref_ngrams = HashSet::new();
                    if ref_words.len() >= n {
                        for w in ref_words.windows(n) {
                            if !is_boilerplate_ngram(w, &dyn_keywords) {
                                ref_ngrams.insert(w.join(" "));
                            }
                        }
                    }

                    let mut matches = Vec::new();
                    if gen_words.len() >= n {
                        for w in gen_words.windows(n) {
                            if !is_boilerplate_ngram(w, &dyn_keywords) {
                                let key = w.join(" ");
                                if ref_ngrams.contains(&key) {
                                    matches.push(key);
                                }
                            }
                        }
                    }

                    let reuse_ok = matches.is_empty();
                    results.push(CheckResult {
                        name: "Cover Letter Wording Reuse (8+ word guardrail)".to_string(),
                        passed: reuse_ok,
                        details: if reuse_ok {
                            "Passed: No 8+ word verbatim matches against reference".to_string()
                        } else {
                            format!("Found {} reused phrase(s). Example: \"{}\"", matches.len(), matches.first().unwrap_or(&"".to_string()))
                        },
                    });
                }
            }
        }
    }

    // 10. LaTeX Source Checks (ATS Single-Column & Escaped Chars)
    let tex_candidate = tex.map(|p| p.to_path_buf()).or_else(|| {
        let sib = pdf.with_extension("tex");
        if sib.exists() { Some(sib) } else { None }
    });

    if let Some(tex_path) = tex_candidate {
        if let Ok(tex_content) = fs::read_to_string(&tex_path) {
            let has_multi = tex_content.contains(r"\begin{multicols}");
            results.push(CheckResult {
                name: "ATS: Single-Column Layout".to_string(),
                passed: !has_multi,
                details: if !has_multi { "Single-column layout verified".to_string() } else { "Multicols detected".to_string() },
            });

            let unescaped = check_unescaped_latex_chars(&tex_content);
            let unesc_ok = unescaped.is_empty();
            results.push(CheckResult {
                name: "LaTeX Special Characters Escaped".to_string(),
                passed: unesc_ok,
                details: if unesc_ok { "Properly escaped".to_string() } else { format!("Unescaped chars: {:?}", unescaped) },
            });
        }
    }

    println!("\n{}", "========================================================".bold());
    println!("  ATS Resume & Cover Letter Validation Report");
    println!("  Target: {} ({:?})", pdf.display(), resolved_doc_type);
    println!("{}\n", "========================================================".bold());

    let mut all_passed = true;
    for res in &results {
        let tag = if res.passed {
            "[PASS]".green().bold()
        } else {
            all_passed = false;
            "[FAIL]".red().bold()
        };
        println!("{} {}: {}", tag, res.name.bold(), res.details);
    }
    println!("{}\n", "--------------------------------------------------------".bold());

    Ok(all_passed)
}
