use crate::utils::markdown_generator::{AsMarkdown, Markdown};
use std::cmp::Ordering;
use std::fs::{self, File};
use std::io::{self};
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
pub struct AppearanceMetadata {
    pub line: usize,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct IssueAppearance {
    pub file_path: String,
    pub metadata: Vec<AppearanceMetadata>,
}

#[derive(Debug, Clone)]
pub struct IssueMetadata {
    pub title: String,
    pub content: String,
    pub severity: Severities,
    pub gas_saved_per_instance: u64,
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

pub fn generate_report_locally(issues: Vec<Issue>, github_link: &str) {
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

    generate_report(report_file, issues, github_link);
    let removing_op = remove_auto_generated_backslashes(report_path);
    match removing_op {
        Ok(_removing_op) => {}
        Err(_error) => {
            print!("Error: Failed to remove backslashes from file")
        }
    }
}

pub fn generate_report_at_dir(dir: &str, issues: Vec<Issue>, github_link: &str) {
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
    generate_report(report_file, issues, github_link);
    let full_dir: &str = &format!("{}/report.md", dir);

    let removing_op = remove_auto_generated_backslashes(full_dir);
    match removing_op {
        Ok(_removing_op) => {}
        Err(_error) => {
            print!("Error: Failed to remove backslashes from file")
        }
    }
}

fn generate_report(file: File, mut issues: Vec<Issue>, github_link: &str) {
    let mut md = Markdown::new(file);

    // Sort inner appearances by file and line
    for issue in issues.iter_mut() {
        sort_appearances_by_file_and_line(&mut issue.issue_appearances);
    }

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

    // Generate the report's summary
    let mut global_instance_amount: usize = 0;
    let mut global_issue_amount: usize = 0;
    let mut global_gas_savings: usize = 0;

    for severity in &[
        Severities::H,
        Severities::M,
        Severities::L,
        Severities::NC,
        Severities::Gas,
    ] {
        // Filter the issues for the current severity
        let severity_issues: Vec<&Issue> = sorted_issues
            .iter()
            .filter(|issue| issue.metadata.severity == *severity)
            .copied()
            .collect();

        if !severity_issues.is_empty() {
            // Add each severity heading
            let severity_header = severity_enum_to_title(severity);
            md.write(severity_header.heading(2)).unwrap();

            // Generate the summary table for each severity
            generate_summary_table(&mut md, &severity_issues, &severity);

            let issues_found: usize = severity_issues.len();
            let mut total_issue_instances: usize = 0;

            for issue in severity_issues {
                total_issue_instances += issue.issue_appearances.len();

                // Total gas saved accounting
                if severity == &Severities::Gas {
                    global_gas_savings +=
                        (issue.metadata.gas_saved_per_instance as usize) * total_issue_instances;
                }
            }

            let instance_word: String;

            if total_issue_instances != 1 {
                instance_word = "instances".to_string();
            } else {
                instance_word = "instance".to_string();
            };

            let issue_word = if issues_found != 1 {
                "issues".to_string()
            } else {
                "issue".to_string()
            };

            let instances_detail: String;
            if severity != &Severities::Gas {
                instances_detail = format!(
                    "Total: {} {instance_word} over {} {issue_word}",
                    &total_issue_instances.to_string(),
                    &issues_found.to_string()
                );
            } else {
                instances_detail = format!(
                    "Total: {} {instance_word} over {} {issue_word}, saving over {} gas units",
                    &total_issue_instances.to_string(),
                    &issues_found.to_string(),
                    &global_gas_savings.to_string()
                );
            }

            md.write(instances_detail.paragraph()).unwrap();

            // Global amounts accounting
            global_instance_amount += &total_issue_instances;
            global_issue_amount += &issues_found;
        }
    }

    md.write("Overall Results".heading(2)).unwrap();
    let global_detail = format!(
        "Total: {} instances over {} issues, potentially saving over {} gas units",
        &global_instance_amount.to_string(),
        &global_issue_amount.to_string(),
        &global_gas_savings.to_string()
    );
    md.write(global_detail.bold().paragraph()).unwrap();

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

            // Add the findings for each issue
            let mut position_id: u32 = 1;
            for issue in severity_issues {
                let issue_instances: usize = issue.issue_appearances.len();
                format_issue(&mut md, issue, &position_id, &issue_instances, github_link);
                position_id += 1;
            }
        }
    }
}

// fn generate_report_summary(md: &Markdown<File>)

// Generate the details for an individual issue
fn format_issue(
    md: &mut Markdown<File>,
    issue: &Issue,
    position: &u32,
    times_found: &usize,
    github_link: &str,
) {
    let full_header = format!("[{}-{}]", &issue.metadata.severity.to_string(), &position)
        + " "
        + &issue.metadata.title;
    let times_found_prompt = times_found_text(times_found);

    // Add the issue title
    md.write(full_header.heading(2)).unwrap();

    // Add the issue description
    md.write(issue.metadata.content.paragraph()).unwrap();

    // Start the details dropdown
    md.write("<details>".paragraph()).unwrap();

    // Add the instances
    let times_found_summary = format!("<summary><i>{}</i></summary>", times_found_prompt);
    md.write(times_found_summary.italic().paragraph()).unwrap();

    // Add the code blocks for each occurrence
    let mut previous_file_path: &String = &mut "".to_string(); // Initialize to empty string

    // Get the first appearance lines of code numbers
    let mut appearance_lines_to_refer: (&usize, &usize) = (
        &issue.issue_appearances[0].metadata[0].line,
        &issue.issue_appearances[0].metadata[&issue.issue_appearances[0].metadata.len() - 1].line,
    );

    for appearance in &issue.issue_appearances {
        let formatted_appearance = format_appearance(
            appearance,
            &appearance.file_path,
            previous_file_path,
            github_link,
            appearance_lines_to_refer,
        );

        md.write(formatted_appearance.paragraph()).unwrap();

        if (&appearance.file_path != previous_file_path) && previous_file_path != "" {
            appearance_lines_to_refer = (
                &appearance.metadata[0].line,
                &appearance.metadata[&appearance.metadata.len() - 1].line,
            );
        }

        previous_file_path = &appearance.file_path;
    }

    // Close last issue appearance
    md.write("```".paragraph()).unwrap();
    // Add the location link for the last appearance
    if github_link != "" {
        let file_path_without_prefix = previous_file_path
            .strip_prefix("./")
            .unwrap_or(previous_file_path);

        // We refer to the first line by default
        let mut full_link: String = format!(
            "{}blob/main/{}#L{}",
            github_link, file_path_without_prefix, appearance_lines_to_refer.0
        );

        // If there are more lines, append them in the link
        if appearance_lines_to_refer.0 != appearance_lines_to_refer.1 {
            full_link += &format!("-L{}", appearance_lines_to_refer.1).to_string();
        }

        let full_formatted_link = &format!("**Location link:** [{}]({})\n\n", full_link, full_link);
        md.write(full_formatted_link.paragraph()).unwrap();
    }

    // Close details dropdown
    md.write("</details>".paragraph()).unwrap();
}

// Sorts all appearances using the first one as reference
fn sort_appearances_by_file_and_line(instances: &mut Vec<IssueAppearance>) {
    instances.sort_by(|a, b| {
        let file_comparison = a.file_path.cmp(&b.file_path);
        if file_comparison == Ordering::Equal {
            a.metadata[0].line.cmp(&b.metadata[0].line)
        } else {
            file_comparison
        }
    });
}

fn format_appearance(
    issue_appearance: &IssueAppearance,
    current_file_path: &str,
    previous_file_path: &str,
    github_link: &str,
    appearance_lines_to_refer: (&usize, &usize),
) -> String {
    let mut formatted_appearance: String = "".to_string();

    if current_file_path != previous_file_path {
        // If this is not the first finding, it means that we need to close the previous codeblock
        if previous_file_path != "" {
            formatted_appearance += "```\n";

            // If a Github link to the project was provided, reference that after closing the block.
            if github_link != "" {
                let file_path_without_prefix = previous_file_path
                    .strip_prefix("./")
                    .unwrap_or(previous_file_path);

                // We refer to the first line by default
                let mut full_link: String = format!(
                    "{}blob/main/{}#L{}",
                    github_link, file_path_without_prefix, appearance_lines_to_refer.0
                );

                // If there are more lines, append them in the link
                if appearance_lines_to_refer.0 != appearance_lines_to_refer.1 {
                    full_link += &format!("-L{}", appearance_lines_to_refer.1).to_string();
                }

                formatted_appearance +=
                    &format!("\n**Location link:** [{}]({})\n\n", full_link, full_link);
            }
            formatted_appearance += "\n";
        }
        formatted_appearance += "```solidity";
        formatted_appearance += &format!("\nFile: {}\n", issue_appearance.file_path);
    }

    for line_of_code in &issue_appearance.metadata {
        let line_number_with_content = &format!(
            "\n{}:    {}",
            &line_of_code.line.to_string(),
            &line_of_code.content
        );
        formatted_appearance += line_number_with_content;
    }

    return formatted_appearance;
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

    return format!("This issue was found {} time:", times_found);
}

fn remove_auto_generated_backslashes(file_path: &str) -> io::Result<()> {
    let file_content = fs::read_to_string(file_path)?;

    let cleared_content = file_content.replace("\\", "");

    fs::write(file_path, cleared_content)?;
    Ok(())
}

fn generate_summary_table(md: &mut Markdown<File>, issues: &Vec<&Issue>, severity: &Severities) {
    // We want to different types: only for Gas add a new column with the amount saved.
    let header_with_placeholder: &str;

    if severity != &Severities::Gas {
        header_with_placeholder = "| |Issue|Instances|\n|-|:-|:-:|";
        let mut position_id: u32 = 1;

        let mut rows: String = "".to_string();

        for issue in issues {
            let issue_instances: usize = issue.issue_appearances.len();
            let finding_indicator = format!(
                "[{}-{}]",
                &issue.metadata.severity.to_string(),
                &position_id
            );

            let current_full_row: String = format!(
                "| {} | {} | {} |\n",
                finding_indicator, issue.metadata.title, issue_instances
            );
            rows += &current_full_row;
            position_id += 1;
        }

        let table = format!("{}\n{}", header_with_placeholder, rows);
        md.write(table.as_str()).unwrap();
    } else {
        header_with_placeholder = "| |Issue|Instances|Total Gas Saved|\n|-|:-|:-:|:-:|";
        let mut position_id: u32 = 1;

        let mut rows: String = "".to_string();

        for issue in issues {
            let issue_instances: u64 = issue.issue_appearances.len().try_into().unwrap();
            let finding_indicator = format!(
                "[{}-{}]",
                &issue.metadata.severity.to_string(),
                &position_id
            );

            let current_full_row: String = format!(
                "| {} | {} | {} | {} |\n",
                finding_indicator,
                issue.metadata.title,
                issue_instances,
                issue.metadata.gas_saved_per_instance * &issue_instances
            );
            rows += &current_full_row;
            position_id += 1;
        }

        let table = format!("{}\n{}", header_with_placeholder, rows);
        md.write(table.as_str()).unwrap();
    };
}

pub fn get_line_content(content: &str, line_number: usize) -> &str {
    let mut current_line_number = 1;
    let mut line_start = 0;

    for (byte_index, byte) in content.bytes().enumerate() {
        if byte == b'\n' {
            if current_line_number == line_number {
                return &content[line_start..byte_index];
            }

            current_line_number += 1;
            line_start = byte_index + 1;
        }
    }

    // Check if the desired line is the last line
    if current_line_number == line_number {
        return &content[line_start..];
    } else {
        return "";
    }
}

pub fn get_line_number(
    content: &str,
    initial_position: &usize,
    final_position: &usize,
) -> Vec<usize> {
    let mut line_count = 0;
    let mut bytes_count = 0;
    let mut line_numbers: Vec<usize> = Vec::new();
    let mut current_line_number = 1;

    for (_byte_index, byte) in content.bytes().enumerate() {
        if byte == b'\n' {
            line_count += 1;

            if &bytes_count >= initial_position && &bytes_count < final_position {
                line_numbers.push(current_line_number);
            }

            current_line_number += 1;
        }

        bytes_count += 1;

        if &bytes_count >= final_position {
            break;
        }
    }

    // Check if the desired line is the last line
    if &bytes_count >= initial_position && current_line_number == line_count + 1 {
        line_numbers.push(current_line_number);
    }

    // println!(
    //     "Line numbers between the offset [{}:{}] = {:?}",
    //     initial_position, final_position, line_numbers
    // );

    return line_numbers;
}
