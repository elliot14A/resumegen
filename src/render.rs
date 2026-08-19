use crate::models::{BulletItem, MasterResume};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct RenderOptions<'a> {
    pub company: &'a str,
    pub role: &'a str,
    pub location: &'a str,
    pub master_path: &'a Path,
    pub summary_id: Option<&'a str>,
    pub summary: Option<&'a str>,
    pub lead_skills: Option<&'a str>,
    pub bullet_tags: Option<&'a str>,
    pub max_bullets_per_role: Option<usize>,
    pub include_projects: Option<&'a str>,
    pub exclude_projects: Option<&'a str>,
    pub include_categories: Option<&'a str>,
    pub exclude_categories: Option<&'a str>,
    pub company_notes: Option<&'a str>,
    pub cover_body: Option<&'a str>,
    pub relocation: bool,
    pub relocation_target: &'a str,
    pub output_dir: &'a Path,
}

pub fn escape_latex(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 16);
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '&' => out.push_str(r"\&"),
            '%' => out.push_str(r"\%"),
            '$' => out.push_str(r"\$"),
            '#' => out.push_str(r"\#"),
            '_' => out.push_str(r"\_"),
            '{' => out.push_str(r"\{"),
            '}' => out.push_str(r"\}"),
            '~' => out.push_str(r"\textasciitilde{}"),
            '^' => out.push_str(r"\textasciicircum{}"),
            '·' => out.push_str(r"$\cdot$ "),
            '\u{2014}' => out.push_str(", "),
            '–' => out.push_str("--"),
            _ => out.push(c),
        }
    }
    out
}

pub fn sanitize_slug(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut last_was_underscore = false;

    for c in s.to_lowercase().chars() {
        if c.is_alphanumeric() {
            result.push(c);
            last_was_underscore = false;
        } else if !last_was_underscore {
            result.push('_');
            last_was_underscore = true;
        }
    }

    result.trim_matches('_').to_string()
}

pub fn resolve_master_resume_path(explicit: Option<&Path>) -> PathBuf {
    if let Some(p) = explicit {
        if p.exists() {
            return p.to_path_buf();
        }
    }
    let candidates = [
        PathBuf::from("master_resume.yaml"),
        PathBuf::from(".resumegen/master_resume.yaml"),
        PathBuf::from("master_resume.example.yaml"),
    ];
    for c in candidates {
        if c.exists() {
            return c;
        }
    }
    PathBuf::from("master_resume.yaml")
}

pub fn resolve_reference_cover_letter_path(explicit: Option<&Path>) -> PathBuf {
    if let Some(p) = explicit {
        if p.exists() {
            return p.to_path_buf();
        }
    }
    let candidates = [
        PathBuf::from(".agents/skills/resume-cover-letter-generator/assets/reference_cover_letter.tex"),
        PathBuf::from(".resumegen/assets/reference_cover_letter.tex"),
        PathBuf::from("assets/reference_cover_letter.tex"),
    ];
    for c in candidates {
        if c.exists() {
            return c;
        }
    }
    PathBuf::from(".agents/skills/resume-cover-letter-generator/assets/reference_cover_letter.tex")
}

pub fn do_render(opts: RenderOptions) -> Result<(PathBuf, PathBuf)> {
    let resolved_path = resolve_master_resume_path(Some(opts.master_path));
    let master_content = fs::read_to_string(&resolved_path)
        .with_context(|| format!("Failed to read master resume at {}", resolved_path.display()))?;
    let master: MasterResume = serde_yaml::from_str(&master_content)
        .with_context(|| "Failed to parse YAML in master resume")?;

    fs::create_dir_all(opts.output_dir)?;
    let candidate_slug = sanitize_slug(&master.candidate.name);
    let company_slug = sanitize_slug(opts.company);
    let resume_tex_path = opts.output_dir.join(format!("{}_resume_{}.tex", candidate_slug, company_slug));
    let cover_tex_path = opts.output_dir.join(format!("{}_cover_letter_{}.tex", candidate_slug, company_slug));

    let cand = &master.candidate;
    let name_upper = cand.name.to_uppercase();
    let role_esc = escape_latex(opts.role);

    // Dynamic relocation determination
    let is_reloc_enabled = opts.relocation
        || cand.relocation.as_ref().map_or(false, |r| r.enabled);

    let reloc_target = cand
        .relocation
        .as_ref()
        .and_then(|r| r.target.as_deref().or(r.default_target.as_deref()))
        .unwrap_or(opts.relocation_target);

    let reloc_str = if is_reloc_enabled {
        if let Some(ref tag) = cand.relocation.as_ref().and_then(|r| r.header_tag.as_deref()) {
            format!(" $\\cdot$ {}", escape_latex(tag))
        } else {
            format!(" $\\cdot$ Open to relocation to {}", escape_latex(reloc_target))
        }
    } else {
        String::new()
    };

    // 1. Summary selection or direct override
    let summary_text = if let Some(sum) = opts.summary {
        sum
    } else if let Some(s_id) = opts.summary_id {
        master.summary_bank.iter().find(|s| s.id == s_id).map(|s| s.text.as_str()).unwrap_or("")
    } else {
        master.summary_bank.first().map(|s| s.text.as_str()).unwrap_or("")
    };

    let summary_esc = escape_latex(summary_text);

    // 2. Experience & Bullet Filtering/Prioritization
    let target_tags: Vec<String> = opts.bullet_tags
        .map(|t| t.split(',').map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()).collect())
        .unwrap_or_default();

    let filter_and_prioritize_bullets = |bullets: &[BulletItem]| -> Vec<BulletItem> {
        let mut result = bullets.to_vec();
        if !target_tags.is_empty() {
            let mut matched = Vec::new();
            let mut others = Vec::new();
            for b in result {
                let has_match = b.tags.iter().any(|t| target_tags.iter().any(|tt| t.eq_ignore_ascii_case(tt)))
                    || target_tags.iter().any(|tt| b.text.to_lowercase().contains(tt));
                if has_match {
                    matched.push(b);
                } else {
                    others.push(b);
                }
            }
            matched.extend(others);
            result = matched;
        }
        if let Some(max_b) = opts.max_bullets_per_role {
            result.truncate(max_b);
        }
        result
    };

    let mut exp_latex = String::new();
    for exp in &master.experience {
        let mut loc_rendered = escape_latex(&exp.location);
        if let Some(ref c_url) = exp.company_url {
            let host = c_url.trim_start_matches("https://").trim_start_matches("http://");
            if loc_rendered.contains(host) {
                loc_rendered = loc_rendered.replace(host, &format!("\\href{{{}}}{{{}}}", c_url, host));
            }
        }

        if exp.roles_history.is_empty() {
            let tailored_bullets = filter_and_prioritize_bullets(&exp.bullets);
            exp_latex.push_str(&format!(
                "\\entryheader{{{}}}{{{}}}{{{}}}{{}}\n\\begin{{itemize}}\n",
                escape_latex(&exp.company),
                escape_latex(&exp.dates),
                loc_rendered
            ));

            for bullet in &tailored_bullets {
                exp_latex.push_str(&format!("  \\item {}\n", escape_latex(&bullet.text)));
            }
            exp_latex.push_str("\\end{itemize}\n\n");
        } else {
            exp_latex.push_str(&format!(
                "\\entryheader{{{}}}{{{}}}{{{}}}{{}}\n\n",
                escape_latex(&exp.company),
                escape_latex(&exp.dates),
                loc_rendered
            ));

            for sub in &exp.roles_history {
                let tailored_bullets = filter_and_prioritize_bullets(&sub.bullets);
                exp_latex.push_str(&format!(
                    "\\subentryheader{{{}}}{{{}}}\n\\begin{{itemize}}\n",
                    escape_latex(&sub.role),
                    escape_latex(&sub.dates)
                ));
                for bullet in &tailored_bullets {
                    exp_latex.push_str(&format!("  \\item {}\n", escape_latex(&bullet.text)));
                }
                exp_latex.push_str("\\end{itemize}\n\n");
            }
        }
    }

    // 3. Projects Selection & Filtering
    let mut projects_to_render = master.projects.clone();
    if let Some(inc) = opts.include_projects {
        let inc_list: Vec<&str> = inc.split(',').map(|s| s.trim()).collect();
        let mut filtered = Vec::new();
        for id in inc_list {
            if let Some(p) = master.projects.iter().find(|p| p.id.eq_ignore_ascii_case(id) || p.name.eq_ignore_ascii_case(id)) {
                filtered.push(p.clone());
            }
        }
        projects_to_render = filtered;
    } else if let Some(exc) = opts.exclude_projects {
        let exc_list: Vec<&str> = exc.split(',').map(|s| s.trim()).collect();
        projects_to_render.retain(|p| !exc_list.iter().any(|e| p.id.eq_ignore_ascii_case(e) || p.name.eq_ignore_ascii_case(e)));
    }

    let mut proj_latex = String::new();
    for proj in &projects_to_render {
        let repo_display_str = proj.repo_display.as_deref().or_else(|| {
            proj.repo_url.as_deref().map(|u| u.trim_start_matches("https://"))
        });

        let repo_link_str = if let (Some(r_url), Some(r_disp)) = (&proj.repo_url, repo_display_str) {
            format!(" (\\href{{{}}}{{{}}})", r_url, escape_latex(r_disp))
        } else {
            String::new()
        };

        proj_latex.push_str(&format!(
            "  \\item \\textbf{{\\href{{{}}}{{{}}}}}{} -- {}\n",
            proj.url,
            escape_latex(&proj.name),
            repo_link_str,
            escape_latex(&proj.summary)
        ));
    }

    // 4. Skills Categories & Prioritization
    let mut categories = master.skills.categories.clone();
    if let Some(inc) = opts.include_categories {
        let inc_list: Vec<&str> = inc.split(',').map(|s| s.trim()).collect();
        let mut filtered = Vec::new();
        for name in inc_list {
            if let Some(c) = master.skills.categories.iter().find(|c| c.name.eq_ignore_ascii_case(name)) {
                filtered.push(c.clone());
            }
        }
        categories = filtered;
    } else if let Some(exc) = opts.exclude_categories {
        let exc_list: Vec<&str> = exc.split(',').map(|s| s.trim()).collect();
        categories.retain(|c| !exc_list.iter().any(|e| c.name.eq_ignore_ascii_case(e)));
    }

    if let Some(lead) = opts.lead_skills {
        let lead_list: Vec<&str> = lead.split(',').map(|s| s.trim()).collect();
        for cat in &mut categories {
            let mut prioritized = Vec::new();
            let mut others = Vec::new();
            for item in &cat.items {
                if lead_list.iter().any(|l| item.eq_ignore_ascii_case(l)) {
                    prioritized.push(item.clone());
                } else {
                    others.push(item.clone());
                }
            }
            prioritized.extend(others);
            cat.items = prioritized;
        }
    }

    let mut skills_latex = String::new();
    for cat in &categories {
        skills_latex.push_str(&format!(
            "  \\item \\textbf{{{}:}} {}\n",
            escape_latex(&cat.name),
            escape_latex(&cat.items.join(", "))
        ));
    }

    let mut edu_latex = String::new();
    for edu in &master.education {
        edu_latex.push_str(&format!(
            "\\noindent\\textbf{{\\color{{primary}}{}}} \\hfill {{\\small\\color{{meta}}{}}}\\\\[1.5pt]\n\\textit{{\\small {}}}\\\\[2.5pt]\n",
            escape_latex(&edu.degree),
            escape_latex(&edu.dates),
            escape_latex(&edu.institution)
        ));
        if let Some(ref d) = edu.details {
            edu_latex.push_str(&format!("{{\\small \\textbf{{{}}}}}\n", escape_latex(d)));
        }
    }

    let resume_tex = format!(
r#"\documentclass[10pt,a4paper]{{article}}

% --- ATS-Safe Packages ---
\usepackage[utf8]{{inputenc}}
\usepackage[T1]{{fontenc}}
\usepackage[margin=0.6in,top=0.52in,bottom=0.52in]{{geometry}}
\usepackage{{xcolor}}
\usepackage{{hyperref}}
\usepackage{{enumitem}}
\usepackage{{titlesec}}

% --- Color Definitions ---
\definecolor{{primary}}{{RGB}}{{23, 51, 79}}
\definecolor{{darktext}}{{RGB}}{{26, 26, 26}}
\definecolor{{meta}}{{RGB}}{{74, 90, 104}}
\definecolor{{linkblue}}{{RGB}}{{0, 0, 158}}

% --- Hyperlink Setup ---
\hypersetup{{
    colorlinks=true,
    linkcolor=linkblue,
    urlcolor=linkblue,
    pdfauthor={{{}}},
    pdftitle={{{} - Resume}}
}}

% --- Typography & Margins ---
\pagestyle{{empty}}
\setlength{{\parindent}}{{0pt}}
\setlength{{\parskip}}{{0pt}}
\color{{darktext}}

% --- Section Styling (ATS-Compliant Headers) ---
\titleformat{{\section}}
  {{\color{{primary}}\fontsize{{10.5pt}}{{12pt}}\bfseries\uppercase}}
  {{}}{{0em}}{{}}[\vspace{{1.5pt}}\titlerule\vspace{{3.5pt}}]
\titlespacing*{{\section}}{{0pt}}{{8pt}}{{3.5pt}}

% --- Custom List Formatting ---
\setlist[itemize]{{
  leftmargin=12pt,
  labelsep=5pt,
  itemsep=2pt,
  topsep=1.5pt,
  parsep=0pt,
  partopsep=0pt,
  label={{\small\textbullet}}
}}

% --- Helper Macros ---
\newcommand{{\entryheader}}[4]{{
  \noindent\textbf{{\color{{primary}}#1}} \hfill {{\small\color{{meta}}#2}}\\
  \textit{{\small #3}} \hfill {{\small\color{{meta}}#4}}\vspace{{2pt}}
}}

\newcommand{{\subentryheader}}[2]{{
  \noindent\textbf{{\color{{primary}}\textit{{\small #1}}}} \hfill {{\small\color{{meta}}#2}}\vspace{{1.5pt}}
}}

\begin{{document}}

% --- Header ---
\begin{{center}}
  {{\fontsize{{18pt}}{{20pt}}\selectfont \textbf{{\color{{primary}}{}}}}}\\[3pt]
  {{\fontsize{{11pt}}{{13pt}}\selectfont \textbf{{\color{{primary}}{}}}}}\\[3pt]
  {{\small {}{}$\cdot$ {} $\cdot$ \href{{mailto:{}}}{{{}}}}}\\[2.5pt]
  {{\small \href{{{}}}{{{}}} \ \textbf{{|}} \ \href{{{}}}{{{}}} \ \textbf{{|}} \ \href{{{}}}{{{}}}}}
\end{{center}}
\vspace{{-2pt}}

% --- Summary ---
\section*{{Summary}}
{}

% --- Experience ---
\section*{{Experience}}
{}
% --- Projects \& Open Source ---
\section*{{Projects \& Open Source}}
\begin{{itemize}}
{}
\end{{itemize}}

% --- Skills ---
\section*{{Skills}}
\begin{{itemize}}[leftmargin=*,label={{}}]
{}
\end{{itemize}}

% --- Education ---
\section*{{Education}}
{}

\end{{document}}
"#,
        cand.name,
        cand.name,
        name_upper,
        role_esc,
        escape_latex(&cand.location),
        reloc_str,
        escape_latex(&cand.phone),
        cand.email,
        cand.email,
        cand.links.portfolio,
        cand.links.portfolio_display,
        cand.links.github,
        cand.links.github_display,
        cand.links.linkedin,
        cand.links.linkedin_display,
        summary_esc,
        exp_latex,
        proj_latex,
        skills_latex,
        edu_latex
    );

    // Cover Letter Generation
    let company_esc = escape_latex(opts.company);
    let loc_esc = escape_latex(opts.location);

    let reloc_header = if is_reloc_enabled {
        if let Some(ref tag) = cand.relocation.as_ref().and_then(|r| r.header_tag.as_deref()) {
            format!(" $\\cdot$ {}", escape_latex(tag))
        } else {
            format!(" $\\cdot$ Open to relocation to {}", escape_latex(reloc_target))
        }
    } else {
        String::new()
    };

    let reloc_paragraph = if let Some(custom) = cand.relocation.as_ref().and_then(|r| r.custom_statement.as_deref()) {
        escape_latex(custom)
    } else if is_reloc_enabled {
        let work_auth = cand
            .relocation
            .as_ref()
            .and_then(|r| r.work_authorization.as_deref())
            .unwrap_or_else(|| {
                if cand.relocation.as_ref().map_or(false, |r| r.blue_card_eligible) {
                    "I am eligible for the EU Blue Card."
                } else {
                    ""
                }
            });
        if work_auth.is_empty() {
            format!("I work regularly across international time zones and would welcome relocation to {}.", escape_latex(reloc_target))
        } else {
            format!("I work regularly across international time zones and would welcome relocation to {}. {}", escape_latex(reloc_target), escape_latex(work_auth))
        }
    } else {
        format!("I am based in {} and available to work across relevant time zones.", escape_latex(&cand.location))
    };

    let intro_paragraph = if let Some(notes) = opts.company_notes {
        escape_latex(notes)
    } else {
        format!("{} has established a high standard for mission-critical software. Having engineered production backend services, developer tooling, and access-control architectures, I would welcome the opportunity to join {} as a {}.", company_esc, company_esc, role_esc)
    };

    let body_paragraph_1 = if let Some(body) = opts.cover_body {
        escape_latex(body)
    } else {
        "At my venture and prior engineering roles, I designed access-control architectures and high-throughput data backends from the ground up. To eliminate security workarounds, request authorization contexts resolve directly within the execution path before any storage operation proceeds. Each credential binds to a verifiable contract with zero privilege escalation paths, emitting structured audit logs and deterministic error responses.".to_string()
    };

    let cover_tex = format!(
r#"\documentclass[10pt,a4paper]{{article}}

% --- ATS-Safe Packages ---
\usepackage[utf8]{{inputenc}}
\usepackage[T1]{{fontenc}}
\usepackage[margin=0.7in,top=0.55in,bottom=0.55in]{{geometry}}
\usepackage{{xcolor}}
\usepackage{{hyperref}}
\usepackage{{titlesec}}

% --- Color Definitions ---
\definecolor{{primary}}{{RGB}}{{23, 51, 79}}
\definecolor{{darktext}}{{RGB}}{{26, 26, 26}}
\definecolor{{meta}}{{RGB}}{{74, 90, 104}}
\definecolor{{linkblue}}{{RGB}}{{0, 0, 158}}

% --- Hyperlink Setup ---
\hypersetup{{
    colorlinks=true,
    linkcolor=linkblue,
    urlcolor=linkblue,
    pdfauthor={{{}}},
    pdftitle={{{} - Cover Letter ({})}}
}}

% --- Typography & Margins ---
\pagestyle{{empty}}
\setlength{{\parindent}}{{0pt}}
\setlength{{\parskip}}{{6pt}}
\color{{darktext}}

\begin{{document}}

% --- Letterhead (Matches Resume Style) ---
\begin{{center}}
  {{\fontsize{{18pt}}{{20pt}}\selectfont \textbf{{\color{{primary}}{}}}}}\\[2.5pt]
  {{\fontsize{{10.5pt}}{{12pt}}\selectfont \textbf{{\color{{primary}}{}}}}}\\[3pt]
  {{\small {}{}$\cdot$ {} $\cdot$ \href{{mailto:{}}}{{{}}}}}\\[2pt]
  {{\small \href{{{}}}{{{}}} \ \textbf{{|}} \ \href{{{}}}{{{}}} \ \textbf{{|}} \ \href{{{}}}{{{}}}}}
\end{{center}}
\vspace{{2pt}}
\hrule height 0.6pt \color{{primary}}
\vspace{{8pt}}

% --- Recipient Block ---
\noindent \textbf{{{}}}\\
{}\\[3pt]
\textbf{{Application: {}}}

\vspace{{3pt}}

Dear {} team,

{}

{}

In open-source software and production services, I led backend engineering across distributed query platforms, optimizing schema indexing and profiling bottlenecks to reduce query latency from multiple seconds to sub-second responses. I structured service cores so storage and transport could evolve independently, and set the API and testing standards across multiple codebases.

{} You can explore my open-source repositories at \href{{{}}}{{{}}} and review my portfolio at \href{{{}}}{{{}}}.

\vspace{{5pt}}
Best regards,\\[2pt]
\textbf{{{}}}

\end{{document}}
"#,
        cand.name,
        cand.name,
        company_esc,
        name_upper,
        role_esc,
        escape_latex(&cand.location),
        reloc_header,
        escape_latex(&cand.phone),
        cand.email,
        cand.email,
        cand.links.portfolio,
        cand.links.portfolio_display,
        cand.links.github,
        cand.links.github_display,
        cand.links.linkedin,
        cand.links.linkedin_display,
        company_esc,
        loc_esc,
        role_esc,
        company_esc,
        intro_paragraph,
        body_paragraph_1,
        reloc_paragraph,
        cand.links.github,
        cand.links.github_display,
        cand.links.portfolio,
        cand.links.portfolio_display,
        cand.name
    );

    fs::write(&resume_tex_path, resume_tex)?;
    fs::write(&cover_tex_path, cover_tex)?;

    Ok((resume_tex_path, cover_tex_path))
}
