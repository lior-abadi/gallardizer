use crate::engine::detectors::{get_match_with_regex, Detector};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use regex::Regex;

pub struct CustomErrorsInsteadOfRevertStrings {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for CustomErrorsInsteadOfRevertStrings {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let pattern: Regex = Regex::new(r"(revert|require)\((.+?)(?:,\s*(.+?))?\)").unwrap();
        let detected_issues_with_regex = get_match_with_regex(parsed_file, pattern);
        for detected_issue in detected_issues_with_regex {
            self.detected_issues.push(detected_issue);
        }
    }

    fn get_detected_issues(&self) -> Vec<IssueAppearance> {
        return self.detected_issues.clone();
    }

    fn get_detector_name(&self) -> String {
        return "CustomErrorsInsteadOfRevertStrings".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::Gas,
            title: indoc! {"Adopt custom errors over `revert()/require()` strings"}.to_string(),
            content: indoc! {
            "From Solidity version `0.8.4`, custom errors are available which can offer gas efficiency compared to 
            `revert()` or `require()` revert strings. Utilizing custom errors saves each time they're triggered, 
            as it bypasses the need to allocate and store the revert string. In addition, omitting the definition of these 
            strings conserves deployment gas. Switching to custom errors can be a significant optimization, enhancing the 
            performance and cost-effectiveness of your smart contract."}.to_string(),
            gas_saved_per_instance: 50,
        };

        return metadata;
    }
}
