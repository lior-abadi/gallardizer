use crate::engine::detectors::{get_match_with_regex, Detector};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use regex::Regex;

pub struct ScientificNotation {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for ScientificNotation {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let pattern: Regex = Regex::new(r"10\*\*\d+\b").unwrap(); // 10**constant
        let detected_issues_with_regex = get_match_with_regex(parsed_file, pattern);

        for detected_issue in detected_issues_with_regex {
            self.detected_issues.push(detected_issue);
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
