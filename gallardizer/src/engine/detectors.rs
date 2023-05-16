use super::report_generator::{generate_report_locally, Issue, IssueAppearance, IssueMetadata};
use crate::utils::file_processor::FileNameWithContent;

pub trait Detector {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent);
    fn get_detector_name(&self) -> String;
    fn get_metadata(&self) -> IssueMetadata;
    fn get_detected_issues(&self) -> Vec<IssueAppearance>;
}

pub fn run_all_detectors(parsed_files: Vec<FileNameWithContent>) {
    let detectors: Vec<Box<dyn Detector>> = get_all_detectors();

    let mut all_detected_issues: Vec<Issue> = Vec::new();

    for mut detector in detectors {
        let detector_name = detector.get_detector_name();

        for file in &parsed_files {
            detector.run_detector(&file);
        }

        let detector_detected_issues: Vec<IssueAppearance> = detector.get_detected_issues();
        let current_issue_metadata: IssueMetadata = detector.get_metadata();

        let current_issue: Issue = Issue {
            issue_appearances: (detector_detected_issues),
            metadata: (current_issue_metadata),
        };

        // Store the detected issues for the current detector
        all_detected_issues.push(current_issue);
    }

    // Generate the formatted report with each issue
    for detected_issue in &all_detected_issues {
        // In here add real-time detection feed.
    }

    generate_report_locally(all_detected_issues);
}

fn get_all_detectors() -> Vec<Box<dyn Detector>> {
    return vec![
        Box::new(pragma_version::PragmaVersionDetector {
            detected_issues: Vec::new(),
        }),
        /* Add more detector modules */
    ];
}

pub mod pragma_version;
