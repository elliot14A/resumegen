use crate::models::LedgerEntry;
use anyhow::Result;
use chrono::Local;

use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn get_documents_resumes_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/kiwi".to_string());
    PathBuf::from(home).join("Documents/resumes")
}

pub fn get_local_resumes_dir() -> PathBuf {
    let dot_resumegen = PathBuf::from(".resumegen");
    if dot_resumegen.exists() {
        dot_resumegen
    } else {
        PathBuf::from("./resumes")
    }
}

pub fn load_ledger_from_csv(csv_path: &Path) -> Vec<LedgerEntry> {
    if !csv_path.exists() {
        return Vec::new();
    }
    let mut entries = Vec::new();
    if let Ok(mut rdr) = csv::Reader::from_path(csv_path) {
        for result in rdr.deserialize() {
            if let Ok(entry) = result {
                entries.push(entry);
            }
        }
    }
    entries
}

pub fn save_ledger_to_csv(entries: &[LedgerEntry], csv_path: &Path) -> Result<()> {
    if let Some(parent) = csv_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(csv_path)?;
    let mut wtr = csv::Writer::from_writer(file);
    for entry in entries {
        wtr.serialize(entry)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn scan_and_collect_entries(dir: &Path) -> Vec<LedgerEntry> {
    let mut discovered = Vec::new();
    if !dir.exists() {
        return discovered;
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/kiwi".to_string());
    let doc_resumes = PathBuf::from(&home).join("Documents/resumes");

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if file_name == "ledger.csv" || file_name.starts_with('.') {
                continue;
            }
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();
            if ext != "pdf" && ext != "docx" && ext != "txt" {
                continue;
            }

            let (company, kind, date) = parse_file_metadata(path, dir);
            let stored_path = if path.starts_with(&doc_resumes) {
                let rel = path.strip_prefix(Path::new(&home)).unwrap_or(path);
                rel.to_string_lossy().to_string()
            } else {
                path.to_string_lossy().to_string()
            };

            discovered.push(LedgerEntry {
                filed_on: date,
                company: company.to_lowercase(),
                kind,
                original_name: file_name.to_string(),
                stored_path,
            });
        }
    }
    discovered
}

pub fn parse_file_metadata(path: &Path, base_dir: &Path) -> (String, String, String) {
    let file_name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let parent = path.parent().unwrap_or(base_dir);
    let parent_name = parent.file_name().and_then(|s| s.to_str()).unwrap_or("");

    let name_lower = file_name.to_lowercase();
    let kind = if name_lower.contains("cover") || name_lower.contains("letter") {
        "cover".to_string()
    } else {
        "resume".to_string()
    };

    let company = if parent != base_dir
        && parent_name != "resumes"
        && parent_name != ".resumegen"
        && !parent_name.is_empty()
    {
        parent_name.to_string()
    } else {
        let parts: Vec<&str> = file_name.split('_').collect();
        if parts.len() >= 3 && (parts[1] == "resume" || parts[1] == "cover") {
            parts[2].to_string()
        } else if parts.len() == 2 && parts[0] == "resume" {
            parts[1].to_string()
        } else {
            "general".to_string()
        }
    };

    let re_date = regex::Regex::new(r"(\d{4}-\d{2}-\d{2})").unwrap();
    let date = if let Some(caps) = re_date.captures(file_name) {
        caps[1].to_string()
    } else if let Ok(meta) = fs::metadata(path) {
        if let Ok(modified) = meta.modified() {
            let dt: chrono::DateTime<Local> = modified.into();
            dt.format("%Y-%m-%d").to_string()
        } else {
            Local::now().format("%Y-%m-%d").to_string()
        }
    } else {
        Local::now().format("%Y-%m-%d").to_string()
    };

    (company, kind, date)
}

pub fn unify_ledgers() -> Result<Vec<LedgerEntry>> {
    let doc_dir = get_documents_resumes_dir();
    let local_dir = get_local_resumes_dir();
    let doc_csv = doc_dir.join("ledger.csv");
    let local_csv = if local_dir.ends_with(".resumegen") {
        local_dir.join("ledger.csv")
    } else {
        PathBuf::from(".resumegen/ledger.csv")
    };

    let mut all_entries = Vec::new();
    all_entries.extend(load_ledger_from_csv(&doc_csv));
    all_entries.extend(load_ledger_from_csv(&local_csv));
    all_entries.extend(scan_and_collect_entries(&doc_dir));
    all_entries.extend(scan_and_collect_entries(&local_dir));
    all_entries.extend(scan_and_collect_entries(&PathBuf::from("./resumes")));

    let mut unique_map: BTreeMap<(String, String, String), LedgerEntry> = BTreeMap::new();
    for mut entry in all_entries {
        entry.company = entry.company.trim().to_lowercase();
        entry.kind = entry.kind.trim().to_lowercase();
        entry.original_name = entry.original_name.trim().to_string();
        entry.stored_path = entry.stored_path.trim().to_string();

        let key = (
            entry.company.clone(),
            entry.kind.clone(),
            entry.original_name.clone(),
        );
        if let Some(existing) = unique_map.get_mut(&key) {
            if entry.filed_on > existing.filed_on {
                existing.filed_on = entry.filed_on;
            }
        } else {
            unique_map.insert(key, entry);
        }
    }

    let mut merged: Vec<LedgerEntry> = unique_map.into_values().collect();
    merged.sort_by(|a, b| {
        b.filed_on
            .cmp(&a.filed_on)
            .then_with(|| a.company.cmp(&b.company))
    });

    let _ = save_ledger_to_csv(&merged, &doc_csv);
    let _ = save_ledger_to_csv(&merged, &local_csv);

    Ok(merged)
}
