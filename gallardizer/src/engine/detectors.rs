pub mod pragma_version;

use super::report_generator::{Issue, IssueAppearance, IssueMetadata};
use crate::utils::file_processor::FileNameWithContent;

pub trait Detector {
    fn run_detector(&self, parsed_file: &FileNameWithContent);
    fn get_detector_name(&self) -> String;
    fn get_metadata(&self) -> IssueMetadata;
    fn get_detected_issues(&self) -> Vec<IssueAppearance>;
}

pub fn run_all_detectors(parsed_files: Vec<FileNameWithContent>) {
    let detectors: Vec<Box<dyn Detector>> = vec![
        Box::new(pragma_version::PragmaVersionDetector {
            detected_issues: Vec::new(),
        }),
        /* Add more detector modules */
    ];

    let mut all_detected_issues: Vec<Issue> = Vec::new();

    for detector in detectors {
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

    // Use the collected detected issues for further processing
    // (e.g., generating a report)
    for detected_issue in &all_detected_issues {
        // Process the detected issues as needed
        println!("Detector: {}", detected_issue.metadata.title);
        println!("Contract: {:?}", detected_issue.metadata.severity);
        for issue_appearance in &detected_issue.issue_appearances {
            println!("Issue: {:?}", issue_appearance);
        }
        println!("---");
    }
}
