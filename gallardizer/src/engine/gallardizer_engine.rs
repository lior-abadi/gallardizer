use super::detectors;
use super::parser;
use super::report_generator::generate_report_locally;
use crate::utils::file_processor::FileNameWithContent;

pub fn gallardize(targets: Vec<FileNameWithContent>, github_link: &str) {
    let parsed_files: Vec<FileNameWithContent> = parser::parse_targets(targets);

    let detected_issues = detectors::run_all_detectors(parsed_files);
    generate_report_locally(detected_issues, github_link);
}
