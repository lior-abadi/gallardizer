use std::fs::Metadata;

use self::{
    Low::pragma_version,
    NonCritical::{reentrancy_modifier_precedence, scientific_notation},
};

use super::report_generator::{
    generate_report_locally, get_line_content, get_line_number, AppearanceMetadata, Issue,
    IssueAppearance, IssueMetadata,
};
use crate::utils::file_processor::FileNameWithContent;
use solang_parser::pt::Loc;

pub trait Detector {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent);
    fn get_detector_name(&self) -> String;
    fn get_metadata(&self) -> IssueMetadata;
    fn get_detected_issues(&self) -> Vec<IssueAppearance>;
}

fn get_all_detectors() -> Vec<Box<dyn Detector>> {
    return vec![
        Box::new(pragma_version::PragmaVersionDetector {
            detected_issues: Vec::new(),
        }),
        Box::new(
            reentrancy_modifier_precedence::ReentrancyModifierPrecedence {
                detected_issues: Vec::new(),
            },
        ),
        Box::new(scientific_notation::ScientificNotation {
            detected_issues: Vec::new(),
        }),
        /* Add more detector modules */
    ];
}

pub fn run_all_detectors(parsed_files: Vec<FileNameWithContent>) -> Vec<Issue> {
    let detectors: Vec<Box<dyn Detector>> = get_all_detectors();

    let mut all_detected_issues: Vec<Issue> = Vec::new();

    for mut detector in detectors {
        let detector_name = detector.get_detector_name();

        for file in &parsed_files {
            detector.run_detector(&file);
        }

        let detector_detected_issues: Vec<IssueAppearance> = detector.get_detected_issues();

        // Store only detected issues
        if (detector_detected_issues.len() != 0) {
            let current_issue_metadata: IssueMetadata = detector.get_metadata();
            let current_issue: Issue = Issue {
                issue_appearances: (detector_detected_issues),
                metadata: (current_issue_metadata),
            };

            // Store the detected issues for the current detector
            all_detected_issues.push(current_issue);
        }
    }

    // Generate the formatted report with each issue
    for detected_issue in &all_detected_issues {
        // In here add real-time detection feed.
    }

    return all_detected_issues;
}

pub fn get_appearance_metadata(loc: &Loc, parsed_file: &FileNameWithContent) -> IssueAppearance {
    let mut issue_appearance = IssueAppearance {
        file_path: String::new(),
        metadata: vec![],
    };

    if let Loc::File(_, initial_position, final_position) = loc {
        let line_number =
            get_line_number(&parsed_file.file_content, initial_position, final_position);

        let mut metadata: Vec<AppearanceMetadata> = vec![];
        for line in line_number {
            let line_content = get_line_content(&parsed_file.file_content, line);

            metadata.push(AppearanceMetadata {
                line: (line),
                content: (line_content.to_owned()),
            })
        }

        issue_appearance = IssueAppearance {
            file_path: parsed_file.file_path.clone(),
            metadata,
        };
    }

    return issue_appearance;
}

pub mod Gas;
pub mod High;
pub mod Low;
pub mod Med;
pub mod NonCritical;
