use crate::models::{BulletItem, MasterResume, SkillCategory};
use crate::render::resolve_master_resume_path;
use anyhow::Result;
use chrono::Local;
use colored::Colorize;
use std::fs;

pub fn handle_skill_list() -> Result<()> {
    let master_path = resolve_master_resume_path(None);
    let content = fs::read_to_string(&master_path)?;
    let resume: MasterResume = serde_yaml::from_str(&content)?;

    println!("\n{}", "========================================================".bold());
    println!("  Declarative Skills Matrix ({})", master_path.display());
    println!("{}\n", "========================================================".bold());
    for cat in &resume.skills.categories {
        println!("{}:", cat.name.green().bold());
        for s in &cat.items {
            println!("  * {}", s);
        }
        println!();
    }
    Ok(())
}

pub fn handle_skill_add(category: &str, skill: &str) -> Result<()> {
    let master_path = resolve_master_resume_path(None);
    let content = fs::read_to_string(&master_path)?;
    let mut resume: MasterResume = serde_yaml::from_str(&content)?;

    let mut found = false;
    for cat in &mut resume.skills.categories {
        if cat.name.eq_ignore_ascii_case(category) {
            if !cat.items.iter().any(|s| s.eq_ignore_ascii_case(skill)) {
                cat.items.push(skill.to_string());
                println!("{} Added '{}' to '{}'", "[OK]".green().bold(), skill.bold(), cat.name.cyan());
            } else {
                println!("{} Skill '{}' already exists", "[EXISTS]".yellow().bold(), skill);
            }
            found = true;
            break;
        }
    }
    if !found {
        resume.skills.categories.push(SkillCategory {
            name: category.to_string(),
            items: vec![skill.to_string()],
        });
        println!("{} Created new category '{}' with skill '{}'", "[OK]".green().bold(), category.cyan(), skill.bold());
    }
    fs::write(&master_path, serde_yaml::to_string(&resume)?)?;
    Ok(())
}

pub fn handle_skill_remove(skill: &str, category: Option<&str>) -> Result<()> {
    let master_path = resolve_master_resume_path(None);
    let content = fs::read_to_string(&master_path)?;
    let mut resume: MasterResume = serde_yaml::from_str(&content)?;

    let mut removed = 0;
    for cat in &mut resume.skills.categories {
        if let Some(target_cat) = category {
            if !cat.name.eq_ignore_ascii_case(target_cat) { continue; }
        }
        let len = cat.items.len();
        cat.items.retain(|s| !s.eq_ignore_ascii_case(skill));
        if cat.items.len() < len {
            println!("{} Removed '{}' from '{}'", "[OK]".green().bold(), skill.bold(), cat.name.cyan());
            removed += 1;
        }
    }
    if removed > 0 {
        fs::write(&master_path, serde_yaml::to_string(&resume)?)?;
    } else {
        println!("{} Skill '{}' not found", "[NOT FOUND]".yellow().bold(), skill);
    }
    Ok(())
}

pub fn handle_add_category(name: &str, skills: Option<&str>) -> Result<()> {
    let master_path = resolve_master_resume_path(None);
    let content = fs::read_to_string(&master_path)?;
    let mut resume: MasterResume = serde_yaml::from_str(&content)?;

    if resume.skills.categories.iter().any(|c| c.name.eq_ignore_ascii_case(name)) {
        println!("{} Category '{}' already exists", "[EXISTS]".yellow().bold(), name);
        return Ok(());
    }
    let items = skills
        .map(|s| s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect())
        .unwrap_or_default();
    resume.skills.categories.push(SkillCategory { name: name.to_string(), items });
    fs::write(&master_path, serde_yaml::to_string(&resume)?)?;
    println!("{} Added category '{}'", "[OK]".green().bold(), name.cyan().bold());
    Ok(())
}

pub fn handle_add_bullet(company: &str, tags: &str, text: &str) -> Result<()> {
    let master_path = resolve_master_resume_path(None);
    let content = fs::read_to_string(&master_path)?;
    let mut resume: MasterResume = serde_yaml::from_str(&content)?;

    let tag_list: Vec<String> = tags.split(',').map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()).collect();
    let bullet_id = format!("{}_{}", company, Local::now().timestamp());
    let mut found = false;
    for exp in &mut resume.experience {
        if exp.id.eq_ignore_ascii_case(company) || exp.company.eq_ignore_ascii_case(company) {
            exp.bullets.push(BulletItem { id: bullet_id.clone(), tags: tag_list.clone(), text: text.to_string() });
            found = true;
            break;
        }
    }
    if found {
        fs::write(&master_path, serde_yaml::to_string(&resume)?)?;
        println!("{} Added bullet to '{}' with ID '{}'", "[OK]".green().bold(), company.bold(), bullet_id.cyan());
    } else {
        eprintln!("{} Company ID '{}' not found", "[ERROR]".red().bold(), company);
    }
    Ok(())
}
