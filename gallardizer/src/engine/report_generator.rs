use crate::utils::markdown_generator::{AsMarkdown, Markdown};
use std::fs::{self, File};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct GasParams {
    pub amount: i64,
}

#[warn(dead_code)]
#[derive(Debug, Clone)]
pub enum Severities {
    GAS(GasParams),
    NC,
    L,
    M,
    H,
}
impl Severities {
    pub fn to_string(&self) -> String {
        match self {
            Severities::GAS(_) => "GAS".to_string(),
            Severities::NC => "NC".to_string(),
            Severities::L => "L".to_string(),
            Severities::M => "M".to_string(),
            Severities::H => "H".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IssueAppearance {
    pub file_path: String,
    pub line: u16,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct IssueMetadata {
    pub title: String,
    pub content: String,
    pub severity: Severities,
}

// An issue is composed by several appearances
#[derive(Clone, Debug)]
pub struct Issue {
    pub issue_appearances: Vec<IssueAppearance>,
    pub metadata: IssueMetadata,
}

pub fn generate_report1() {
    // generate_report_at_dir("out/");
}

pub fn generate_report_locally(issues: Vec<Issue>) {
    let report_dir = "./out";
    if let Err(err) = fs::create_dir_all(report_dir) {
        eprintln!("Failed to create report directory: {}", err);
        return;
    }

    let report_path = "./out/report.md";
    let report_file = match File::create(&report_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create report file: {}", err);
            return;
        }
    };

    generate_report(report_file, issues);
}

pub fn generate_report_at_dir(dir: &str, issues: Vec<Issue>) {
    let report_path = Path::new(dir).join("report.md");
    report_path.to_string_lossy().into_owned();

    let report_file = match File::create(&report_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create report file: {}", err);
            eprintln!("Have you created the target directory?");
            return;
        }
    };
    generate_report(report_file, issues);
}

fn generate_report(file: File, issues: Vec<Issue>) {
    let mut md = Markdown::new(file);

    for issue in &issues {
        let times_found: usize = issue.issue_appearances.len();
        let formatted_issue: FormattedIssue = format_issue(&1, issue, &times_found);

        // Add the title
        md.write(&formatted_issue.heading.heading(2)).unwrap();

        // Add the description body
        md.write(issue.metadata.content.paragraph()).unwrap();

        // Add the occurrences
        md.write(formatted_issue.times_found.italic().paragraph())
            .unwrap();
    }
}

struct FormattedIssue {
    heading: String,
    times_found: String,
}

fn format_issue(position: &u16, issue: &Issue, times_found: &usize) -> FormattedIssue {
    let full_header = format!("[{}-{}]", &issue.metadata.severity.to_string(), &position)
        + " "
        + &issue.metadata.title;
    let times_found_prompt = times_found_text(times_found);

    return FormattedIssue {
        heading: (full_header),
        times_found: (times_found_prompt),
    };
}

fn times_found_text(times_found: &usize) -> String {
    if times_found > &1 {
        return format!("This issue was found {} times:", times_found);
    }

    return format!("This issue found {} time:", times_found);
}
