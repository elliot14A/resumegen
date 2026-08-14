use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MasterResume {
    pub candidate: CandidateInfo,
    #[serde(default)]
    pub summary_bank: Vec<SummaryItem>,
    #[serde(default)]
    pub experience: Vec<ExperienceItem>,
    #[serde(default)]
    pub projects: Vec<ProjectItem>,
    pub skills: SkillsSection,
    #[serde(default)]
    pub education: Vec<EducationItem>,
    #[serde(default)]
    pub custom_checks: Option<CustomChecks>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CandidateInfo {
    pub name: String,
    pub title: String,
    pub location: String,
    pub email: String,
    pub phone: String,
    pub links: LinksInfo,
    #[serde(default)]
    pub relocation: Option<RelocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RelocationInfo {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub header_tag: Option<String>,
    #[serde(default)]
    pub custom_statement: Option<String>,
    #[serde(default)]
    pub work_authorization: Option<String>,
    #[serde(default)]
    pub default_target: Option<String>,
    #[serde(default)]
    pub sponsorship_needed: bool,
    #[serde(default)]
    pub blue_card_eligible: bool,
    #[serde(default)]
    pub spoken_languages: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CustomChecks {
    #[serde(default)]
    pub banned_words: Option<Vec<String>>,
    #[serde(default)]
    pub verify_institution: Option<bool>,
    #[serde(default)]
    pub max_resume_pages: Option<usize>,
    #[serde(default)]
    pub max_cover_letter_pages: Option<usize>,
    #[serde(default)]
    pub custom_boilerplate_keywords: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinksInfo {
    pub portfolio: String,
    pub portfolio_display: String,
    pub github: String,
    pub github_display: String,
    pub linkedin: String,
    pub linkedin_display: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SummaryItem {
    pub id: String,
    pub focus: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExperienceItem {
    pub id: String,
    pub company: String,
    pub company_url: Option<String>,
    pub role: String,
    pub dates: String,
    pub location: String,
    #[serde(default)]
    pub roles_history: Vec<RoleHistoryItem>,
    pub summary: Option<String>,
    #[serde(default)]
    pub bullets: Vec<BulletItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleHistoryItem {
    pub role: String,
    pub dates: String,
    #[serde(default)]
    pub bullets: Vec<BulletItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BulletItem {
    pub id: String,
    pub tags: Vec<String>,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectItem {
    pub id: String,
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub repo_url: Option<String>,
    #[serde(default)]
    pub repo_display: Option<String>,
    pub stack: Vec<String>,
    pub summary: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillsSection {
    pub categories: Vec<SkillCategory>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillCategory {
    pub name: String,
    pub items: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EducationItem {
    pub institution: String,
    pub degree: String,
    pub dates: String,
    pub location: String,
    #[serde(default)]
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerEntry {
    pub filed_on: String,
    pub company: String,
    pub kind: String,
    pub original_name: String,
    pub stored_path: String,
}
