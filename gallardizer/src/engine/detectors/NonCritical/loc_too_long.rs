use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use solang_parser::pt::Loc;

pub struct LocTooLong {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for LocTooLong {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let lines: Vec<&str> = parsed_file.file_content.split('\n').collect();

        for (index, line) in lines.iter().enumerate() {
            let trimmed_line = line.trim();
            let line_length = trimmed_line.len();

            // Long line detected
            if line_length > 135 {
                // Skip inline SVGs
                if trimmed_line.contains("/>")
                    || (trimmed_line.contains("'<") && trimmed_line.contains(">'"))
                {
                    continue;
                }

                let start_offset = lines[..index].iter().map(|l| l.len() + 1).sum::<usize>();
                let end_offset = start_offset + line_length;

                let loc = Loc::File(0, start_offset, end_offset);
                self.detected_issues
                    .push(get_appearance_metadata(&loc, parsed_file));
            }
        }
    }

    fn get_detected_issues(&self) -> Vec<IssueAppearance> {
        return self.detected_issues.clone();
    }

    fn get_detector_name(&self) -> String {
        return "LocTooLong".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::NC,
            title: indoc! {"Long lines of code"}.to_string(),
            content: indoc! {
            "Traditionally, source code lines are restricted to 80 characters. 
            With contemporary screens being considerably larger, this rule can be somewhat relaxed. 
            The [Solidity style guide](https://docs.soliditylang.org/en/latest/style-guide.html#maximum-line-length), however, suggests a maximum limit of 120 characters per line. 
            Therefore, it's advisable to break up lines when they approach this length."}
            .to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}
