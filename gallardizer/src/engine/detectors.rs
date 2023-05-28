use super::report_generator::{
    generate_report_locally, get_line_content, get_line_number, AppearanceMetadata, Issue,
    IssueAppearance, IssueMetadata,
};
use crate::utils::file_processor::FileNameWithContent;
use regex::Regex;
use solang_parser::pt::Loc;

pub trait Detector {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent);
    fn get_detector_name(&self) -> String;
    fn get_metadata(&self) -> IssueMetadata;
    fn get_detected_issues(&self) -> Vec<IssueAppearance>;
}

pub fn run_all_detectors(parsed_files: Vec<FileNameWithContent>) -> Vec<Issue> {
    let detectors: Vec<Box<dyn Detector>> = get_all_detectors();

    let mut all_detected_issues: Vec<Issue> = Vec::new();

    // print!("Running detectors...\n");
    for mut detector in detectors {
        let detector_name = detector.get_detector_name();

        // print!("\nCurrently Running: {detector_name}");

        for file in &parsed_files {
            detector.run_detector(&file);
        }

        let detector_detected_issues: Vec<IssueAppearance> = detector.get_detected_issues();

        // Store only detected issues
        let amount_of_issues_detected = detector_detected_issues.len();
        // print!("\nIssues Detected: {amount_of_issues_detected}\n");

        if amount_of_issues_detected != 0 {
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

fn get_match_with_regex(parsed_file: &FileNameWithContent, pattern: Regex) -> Vec<IssueAppearance> {
    let file_content = &parsed_file.file_content;
    let mut byte_offset = 0;
    let lines: Vec<&str> = file_content.lines().collect();

    let mut detected_issues: Vec<IssueAppearance> = vec![];
    // Iterate over each line in the file content
    for (_index, line) in lines.iter().enumerate() {
        // Skip the lines starting with //, ///, /**, or *
        if line.trim().starts_with("//")
            || line.trim().starts_with("///")
            || line.trim().starts_with("/**")
            || line.trim().starts_with("*")
        {
            byte_offset += line.len() + 1; // Add 1 for the newline character
            continue;
        }

        if pattern.is_match(line) {
            if let Some(captures) = pattern.captures(line) {
                let entire_match = captures.get(0).unwrap(); // The entire match is the first capture group
                let byte_start = byte_offset + entire_match.start();
                let byte_end = byte_offset + entire_match.end(); // This now gives the end of the entire match

                let loc = Loc::File(0, byte_start, byte_end);
                let issue_appearance = get_appearance_metadata(&loc, parsed_file);
                detected_issues.push(issue_appearance);
            }
        }

        // Update the byte offset for the next line
        byte_offset += line.len() + 1; // Add 1 for the newline character
    }

    return detected_issues;
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

use self::{
    Gas::{division_by_two, storage_for_mapping_array, use_custom_errors},
    Low::{
        division_by_zero, ext_call_for_loop, loss_of_precision, pragma_version,
        require_instead_of_assert, unchecked_array_length,
    },
    Med::{centralization_risk, safe_mint_erc721, safe_transfer_erc721, solmate_safetransfer},
    NonCritical::{
        keccak_immutable, loc_too_long, magic_numbers, missing_indexed_fields,
        numeric_timevariables, reentrancy_modifier_precedence, revert_strings, scientific_notation,
    },
};

fn get_all_detectors() -> Vec<Box<dyn Detector>> {
    return vec![
        /* ==== MED ==== */
        Box::new(safe_mint_erc721::SafeMintERC721 {
            detected_issues: Vec::new(),
        }),
        Box::new(safe_transfer_erc721::SafeTransferERC721 {
            detected_issues: Vec::new(),
        }),
        Box::new(centralization_risk::CentralizationRisk {
            detected_issues: Vec::new(),
        }),
        Box::new(solmate_safetransfer::SolmateSafeTransfer {
            detected_issues: Vec::new(),
        }),
        /* ==== LOW ==== */
        Box::new(loss_of_precision::LossOfPrecision {
            detected_issues: Vec::new(),
        }),
        Box::new(pragma_version::PragmaVersionDetector {
            detected_issues: Vec::new(),
        }),
        Box::new(ext_call_for_loop::ExternalCallInsideForLoopDoS {
            detected_issues: Vec::new(),
        }),
        Box::new(require_instead_of_assert::RequireInsteadOfAssert {
            detected_issues: Vec::new(),
        }),
        Box::new(division_by_zero::DivisionByZero {
            detected_issues: Vec::new(),
        }),
        Box::new(unchecked_array_length::UncheckedArrayLength {
            detected_issues: Vec::new(),
        }),
        /* ==== NC ==== */
        Box::new(
            reentrancy_modifier_precedence::ReentrancyModifierPrecedence {
                detected_issues: Vec::new(),
            },
        ),
        Box::new(scientific_notation::ScientificNotation {
            detected_issues: Vec::new(),
        }),
        Box::new(revert_strings::RevertStrings {
            detected_issues: Vec::new(),
        }),
        Box::new(magic_numbers::MagicNumbers {
            detected_issues: Vec::new(),
        }),
        Box::new(numeric_timevariables::NumericTimeVariables {
            detected_issues: Vec::new(),
        }),
        Box::new(keccak_immutable::KeccakImmutable {
            detected_issues: Vec::new(),
        }),
        Box::new(loc_too_long::LocTooLong {
            detected_issues: Vec::new(),
        }),
        Box::new(missing_indexed_fields::MissingIndexedFields {
            detected_issues: Vec::new(),
        }),
        /* ==== GAS ==== */
        Box::new(use_custom_errors::CustomErrorsInsteadOfRevertStrings {
            detected_issues: Vec::new(),
        }),
        Box::new(storage_for_mapping_array::StorageForMappingArray {
            detected_issues: Vec::new(),
        }),
        Box::new(division_by_two::DivisionByTwoShift {
            detected_issues: Vec::new(),
        }),
        /* Add more detector modules */
    ];
}
