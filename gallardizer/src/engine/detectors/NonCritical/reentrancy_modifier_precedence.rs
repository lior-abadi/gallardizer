use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;
use solang_parser::pt::{ContractPart, FunctionAttribute, FunctionTy, SourceUnitPart};

pub struct ReentrancyModifierPrecedence {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for ReentrancyModifierPrecedence {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        for part in &parsed_file.parsed_ast_tree.0 {
            match part {
                SourceUnitPart::ContractDefinition(def) => {
                    for part in &def.parts {
                        match part {
                            ContractPart::FunctionDefinition(def) => {
                                if def.ty == FunctionTy::Function {
                                    let attributes = &def.attributes;

                                    let mut detected: bool = false;
                                    for (index, attribute) in attributes.iter().enumerate() {
                                        if match attribute {
                                            FunctionAttribute::BaseOrModifier(_, base) => {
                                                base.name.to_string() == "nonReentrant"
                                            }
                                            _ => false,
                                        } {
                                            if index == 0 {
                                                break;
                                            };

                                            let prev_attribute = &attributes[index - 1];
                                            if match prev_attribute {
                                                FunctionAttribute::BaseOrModifier(_, base) => {
                                                    base.name.to_string().len() != 0
                                                }
                                                _ => false,
                                            } {
                                                detected = true;
                                            }
                                        }
                                    }

                                    if detected {
                                        let issue_appearance =
                                            get_appearance_metadata(&def.loc, parsed_file);
                                        self.detected_issues.push(issue_appearance);
                                    }
                                }
                            }
                            _ => (),
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
        return "ReentrancyModifierPrecedence".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::NC,
            title: indoc! {"The <code>nonReentrant</code> modifier should precede all other modifiers"}.to_string(),
            content: indoc! {
            "Prioritizing reentrancy checks before any other calculations or validations within modifiers 
            is a recommended practice for enhancing the security of the protected function."}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}
