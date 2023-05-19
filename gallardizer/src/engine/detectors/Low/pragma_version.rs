use crate::engine::detectors::Detector;
use crate::engine::report_generator::{
    get_line_content, get_line_number, IssueAppearance, IssueMetadata, Severities,
};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;
use solang_parser::pt::{Loc, SourceUnitPart};

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

                    if detected {
                        if let Loc::File(_, initial_position, _) = def {
                            let line_number =
                                get_line_number(&parsed_file.file_content, initial_position);

                            let line_content =
                                get_line_content(&parsed_file.file_content, line_number);

                            let issue_appearance: IssueAppearance = IssueAppearance {
                                file_path: (parsed_file.file_path.clone()),
                                line: line_number,
                                content: line_content.to_owned(),
                            };
                            self.detected_issues.push(issue_appearance);
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
            title: r#"Insecure declaration of pragma version"#.to_string(),
            content: indoc! {
            "The specified <code>pragma</code> version allows for the utilization of different compiler versions to compile the source code.
            It's important to consider the potential risks associated with using a floating or flexible pragma version. 
            For instance, employing versions <code>0.8.7</code> or earlier may result in compilation errors, as they lack support for 
            functions overriding interface functions without using the <code>override</code> modifier, 
            which is exclusively available in Solidity <code>0.8.8</code> and newer versions.<br> 
            
            Similarly, the usage of <code>abi.encodeCall</code>, which was introduced in Solidity <code>0.8.11</code>, 
            may cause issues if the codebase relies on it. Although it is uncertain whether these specific bugs related to <code>override</code> 
            or <code>encode</code> will manifest in the code, exercising caution is advised to avoid potential unexpected scenarios or compatibility
            issues that may arise with the inclusion of new features or implementations.
            Considering the uncertainty of potential bugs related to <code>override</code>, <code>encode</code>, or others, using a floating (flexible)
            <code>pragma</code> version might lead to the project compiling with uncertain versions within that range.<br>
            
            Consider upgrading the pragma version to a newer release, preferably the most recent version available, 
            in order to mitigate potential risks stemming from bug fixes introduced in previous releases. 
            Additionally, it is recommended to make the pragma version fixed to ensure consistency and stability in the project."}.to_string(),
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
