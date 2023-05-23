use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;
use solang_parser::pt::SourceUnitPart;

pub struct PragmaVersionDetector {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for PragmaVersionDetector {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        // Checks if the pragma version is under 0.8.0 and if it is floating
        for part in &parsed_file.parsed_ast_tree.0 {
            match part {
                SourceUnitPart::PragmaDirective(loc, _opt, _opt_lit) => {
                    if let Some(pragma_literal) = _opt_lit {
                        let pragma_version = &pragma_literal.string;
                        let detected: bool = check_floating_pragma(&pragma_version)
                            || check_pragma_version(&pragma_version);

                        if detected {
                            self.detected_issues
                                .push(get_appearance_metadata(&loc, parsed_file));
                        }
                    }
                }
                _ => (),
            }
        }
    }

    fn get_detected_issues(&self) -> Vec<IssueAppearance> {
        return self.detected_issues.clone();
    }

    fn get_detector_name(&self) -> String {
        return "PragmaVersion".to_string();
    }
    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::L,
            title: indoc! {"Insecure declaration of <code>pragma</code> version"}.to_string(),
            content: indoc! {
            " The utilization of a flexible pragma version could introduce a variety of potential risks to your contract, 
            accommodating a range of compiler versions which may lack support for specific improvements and changes such as 
            <code>0.8.11</code>'s <code>abi.encodeCall</code>.<br>

            Without singling out these features as definitive concerns, it's important to acknowledge the broad 
            spectrum of unexpected complications that could occur. A recommendation would be to align with a fixed, 
            updated pragma version, providing a defense against potential compatibility issues that are tied to evolving 
            language specifications and reducing exposure to bugs fixed in recent compiler versions, all of which contributes 
            to a more stable project."}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}

fn check_floating_pragma(content: &str) -> bool {
    return content.contains("^") || content.contains(">");
}

fn check_pragma_version(content: &str) -> bool {
    let parts: Vec<&str> = content.split('.').collect();

    // We ensure that the pragma format is X.Y.Z (e.g. 0.8.11)
    if parts.len() >= 3 {
        if let (Some(second), Some(last)) =
            (parts[1].parse::<u32>().ok(), parts[2].parse::<u32>().ok())
        {
            if second < 8 || (second >= 8 && last < 8) {
                return true;
            }
        }
    }

    return false;
}
