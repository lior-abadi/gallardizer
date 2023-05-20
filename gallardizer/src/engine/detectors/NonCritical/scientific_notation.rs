use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use regex::Regex;
use solang_parser::pt::Loc;

pub struct ScientificNotation {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for ScientificNotation {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let file_content = &parsed_file.file_content;
        let lines: Vec<&str> = file_content.lines().collect();

        let mut byte_offset = 0;
        let pattern = Regex::new(r"10\*\*[+-]?\d+(.\d+)?").unwrap(); // 10**number

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

            // Search for the ** pattern in the line
            if let Some(start) = line.find("**") {
                let byte_start = byte_offset + start;
                let byte_end = byte_start + 2; // Assuming ** is always two characters

                let loc = Loc::File(0, byte_start, byte_end);
                let issue_appearance = get_appearance_metadata(&loc, parsed_file);
                self.detected_issues.push(issue_appearance);
            }

            // Update the byte offset for the next line
            byte_offset += line.len() + 1; // Add 1 for the newline character
        }
    }

    fn get_detected_issues(&self) -> Vec<IssueAppearance> {
        return self.detected_issues.clone();
    }

    fn get_detector_name(&self) -> String {
        return "ScientificNotation".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::NC,
            title: indoc! {"Prefer scientific notation over exponentiation"}.to_string(),
            content: indoc! {
            "Although the compiler effectively optimizes the use of exponentiation, 
            it's generally more advisable to employ scientific notation for representing large numbers. 
            By opting for idioms like <code>1e18</code> instead of <code>10**18</code>, you're using a method that
            inherently does not require additional compiler optimization.<br>
             
            This practice promotes clarity and efficiency in your code, aligning with robust coding standards."}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}
