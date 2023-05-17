use crate::utils::markdown_generator::{AsMarkdown, Markdown};
use std::fs::{self, File};
use std::path::Path;

#[warn(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Severities {
    Gas,
    NC,
    L,
    M,
    H,
}

impl Severities {
    pub fn to_string(&self) -> String {
        match self {
            Severities::Gas => "G".to_string(),
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
    pub line: u32,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct IssueMetadata {
    pub title: String,
    pub content: String,
    pub severity: Severities,
    pub gas_saved_per_instance: i64,
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

    // Sort the issues by severity in the desired order
    let mut sorted_issues: Vec<&Issue> = issues.iter().collect();
    sorted_issues.sort_by_key(|issue| match issue.metadata.severity {
        Severities::H => 0,
        Severities::M => 1,
        Severities::L => 2,
        Severities::NC => 3,
        Severities::Gas => 4,
    });

    let issue_summary_header = format!("Issues Summary");
    md.write(issue_summary_header.heading(1)).unwrap();

    // Generate the report for each severity group
    for severity in &[
        Severities::H,
        Severities::M,
        Severities::L,
        Severities::NC,
        Severities::Gas, // Replace with the actual GasParams value if available
    ] {
        // Filter the issues for the current severity
        let severity_issues: Vec<&Issue> = sorted_issues
            .iter()
            .filter(|issue| issue.metadata.severity == *severity)
            .copied()
            .collect();

        if !severity_issues.is_empty() {
            // Add the severity heading
            let severity_header = severity_enum_to_title(severity);
            md.write(severity_header.heading(2)).unwrap();

            //  generate_summary_table(&mut md, &severity_issues);
            let issues_found: usize = severity_issues.len();
            let mut total_issue_instances: usize = 0;

            for issue in severity_issues {
                total_issue_instances += issue.issue_appearances.len();
            }

            let instance_word = if (total_issue_instances != 1) {
                "instances".to_string()
            } else {
                "instance".to_string()
            };

            let issue_word = if (issues_found != 1) {
                "issues".to_string()
            } else {
                "issue".to_string()
            };

            let instances_detail = format!(
                "Total: {} {instance_word} were found over {} {issue_word}.",
                &total_issue_instances.to_string(),
                &issues_found.to_string()
            );

            md.write(instances_detail.paragraph()).unwrap();
            md.write("\n\n").unwrap();
        }
    }

    // Generate the report for each severity group
    for severity in &[
        Severities::H,
        Severities::M,
        Severities::L,
        Severities::NC,
        Severities::Gas, // Replace with the actual GasParams value if available
    ] {
        // Filter the issues for the current severity
        let severity_issues: Vec<&Issue> = sorted_issues
            .iter()
            .filter(|issue| issue.metadata.severity == *severity)
            .copied()
            .collect();

        if !severity_issues.is_empty() {
            // Add the severity heading
            let severity_header = severity_enum_to_title(severity);
            md.write(severity_header.heading(1)).unwrap();

            // Add the summary table
            //  generate_summary_table(&mut md, &severity_issues);

            // Add the findings for each issue
            let mut position_id: u32 = 1;
            for issue in severity_issues {
                let issue_instances: usize = issue.issue_appearances.len();
                format_issue(&mut md, issue, &position_id, &issue_instances);
                position_id += 1;
            }

            // Add a newline for readability between severity groups
            md.write("\n\n").unwrap();
        }
    }
}

// fn generate_report_summary(md: &Markdown<File>)

// Generate the details for an individual issue
fn format_issue(md: &mut Markdown<File>, issue: &Issue, position: &u32, times_found: &usize) {
    let full_header = format!("[{}-{}]", &issue.metadata.severity.to_string(), &position)
        + " "
        + &issue.metadata.title;
    let times_found_prompt = times_found_text(times_found);

    // Add the issue title
    md.write(full_header.heading(2)).unwrap();

    // Add the issue description
    md.write(issue.metadata.content.paragraph()).unwrap();

    // Add the instances
    md.write(times_found_prompt.italic().paragraph()).unwrap();

    // Add the code blocks for each occurrence
    for appearance in &issue.issue_appearances {
        md.write(appearance.content.paragraph()).unwrap();
        md.write("\n").unwrap();
    }

    // Add a newline for readability between issues
    md.write("\n").unwrap();
}

fn severity_enum_to_title(severity: &Severities) -> String {
    return match severity {
        Severities::Gas => "Gas Optimizations".to_string(),
        Severities::NC => "Non-Critical Issues".to_string(),
        Severities::L => "Low Risk Issues".to_string(),
        Severities::M => "Medium Risk Issues".to_string(),
        Severities::H => "High Risk Issues".to_string(),
    };
}

fn times_found_text(times_found: &usize) -> String {
    if times_found > &1 {
        return format!("This issue was found {} times:", times_found);
    }

    return format!("This issue found {} time:", times_found);
}

fn generate_summary_tables(issues: &Vec<Issue>) {}
