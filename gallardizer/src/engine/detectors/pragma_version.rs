use super::Detector;
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;
use solang_parser::pt::SourceUnitPart;

pub struct PragmaVersionDetector {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for PragmaVersionDetector {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        for part in &parsed_file.parsed_ast_tree.0 {
            match part {
                SourceUnitPart::PragmaDirective(def, _opt, _opt_lit) => {
                    let pragma_version = _opt_lit.as_ref().map(|s| s.string.clone()).unwrap();

                    let detected: bool = check_floating_pragma(&pragma_version)
                        || check_pragma_version(&pragma_version);
                    println!("{pragma_version} - PragmaVersionIssue: {detected}");

                    if detected {
                        let issue_appearance: IssueAppearance = IssueAppearance {
                            file_path: (parsed_file.file_path.clone()),
                            line: (0),
                            content: (pragma_version),
                        };
                        self.detected_issues.push(issue_appearance);
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
            title: r#"Inconsistent floating pragma version"#.to_string(),
            content: indoc! {
            "The specified <code>pragma</code> version allows for the utilization of compiler versions
            beyond <code>0.8.0</code> to compile the source code.
            However, it's important to consider the potential risks associated with using a floating pragma version.<br>

            Employing versions <code>0.8.7</code>or earlier may result in compilation errors, as they lack support for
            functions overriding interface functions without using the <code>override</code> modifier, which is exclusively 
            available in Solidity <code>0.8.8</code> and newer versions. Similarly, the usage of abi.encodeCall, 
            which was introduced in Solidity <code>0.8.11</code>, may cause issues if the codebase relies on it.<br>

            While it is not confirmed whether these specific bugs related to override or encoding will appear in the code, 
            it is advised to be cautious. Considering the uncertainty of potential bugs related to 
            <code>override</code>, <code>encode</code>, or others, it is recommended to avoid using a floating pragma version.<br>

            Consider upgrading the pragma version to a newer release the most recent version available, 
            in order to mitigate potential risks leveraging from bug fixes introduced on newer releases. 
            Also, make the pragma version fixed."}.to_string(),
        };

        return metadata;
    }
}

fn check_floating_pragma(content: &str) -> bool {
    return content.contains("^") || content.contains(">");
}

fn check_pragma_version(content: &str) -> bool {
    let parts: Vec<&str> = content.split('.').collect();

    // We ensure that the pragma format is X.Y.Z (e.g. 0.8.9)
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
