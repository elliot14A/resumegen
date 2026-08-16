use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn do_compile(input: &Path, output: Option<&Path>) -> Result<PathBuf> {
    if !input.exists() {
        return Err(anyhow::anyhow!("Input file not found: {}", input.display()));
    }

    let input_abs = fs::canonicalize(input)?;
    let input_dir = input_abs.parent().unwrap_or_else(|| Path::new("."));
    let file_stem = input_abs.file_stem().and_then(|s| s.to_str()).unwrap_or("document");

    let (out_dir, target_pdf_path) = match output {
        Some(out) => {
            if out.extension().map_or(false, |ext| ext == "pdf") {
                let parent = out.parent().unwrap_or_else(|| Path::new("."));
                fs::create_dir_all(parent)?;
                (parent.to_path_buf(), Some(out.to_path_buf()))
            } else {
                fs::create_dir_all(out)?;
                let default_pdf = out.join(format!("{}.pdf", file_stem));
                (out.to_path_buf(), Some(default_pdf))
            }
        }
        None => (input_dir.to_path_buf(), None),
    };

    println!("{} Compiling {} with tectonic...", "[INFO]".blue().bold(), input_abs.display());

    let mut cmd = Command::new("tectonic");
    cmd.arg(&input_abs);
    cmd.arg("-o").arg(&out_dir);

    let output_res = cmd.output().map_err(|e| {
        anyhow::anyhow!("Failed to execute tectonic. Is tectonic on PATH? Error: {}", e)
    })?;

    if !output_res.status.success() {
        let stderr = String::from_utf8_lossy(&output_res.stderr);
        let stdout = String::from_utf8_lossy(&output_res.stdout);
        eprintln!("{} Tectonic compilation failed:\n{}\n{}", "[FAIL]".red().bold(), stdout, stderr);
        return Err(anyhow::anyhow!("Compilation failed"));
    }

    let default_generated_pdf = out_dir.join(format!("{}.pdf", file_stem));
    let final_pdf = if let Some(target) = target_pdf_path {
        if target != default_generated_pdf && default_generated_pdf.exists() {
            fs::rename(&default_generated_pdf, &target)?;
        }
        target
    } else {
        default_generated_pdf
    };

    if !final_pdf.exists() {
        return Err(anyhow::anyhow!("Expected PDF not found at {}", final_pdf.display()));
    }

    let content = fs::read(&final_pdf)?;
    if content.len() < 500 || !content.starts_with(b"%PDF-") {
        return Err(anyhow::anyhow!("File is not a valid PDF: {}", final_pdf.display()));
    }

    println!(
        "{} Successfully compiled to {} ({} bytes)",
        "[PASS]".green().bold(),
        final_pdf.display(),
        content.len()
    );

    Ok(final_pdf)
}
