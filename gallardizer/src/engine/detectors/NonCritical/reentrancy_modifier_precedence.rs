use crate::engine::detectors::Detector;
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;
use solang_parser::pt::{Base, ContractPart, FunctionAttribute, FunctionTy, Loc, SourceUnitPart};
use std::any::type_name;

pub struct ReentrancyModifierPrecedence {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for ReentrancyModifierPrecedence {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        for part in &parsed_file.parsed_ast_tree.0 {
            match part {
                SourceUnitPart::ContractDefinition(def) => {
                    // println!("Found contract {:?}", def.name);
                    for part in &def.parts {
                        match part {
                            ContractPart::FunctionDefinition(def) => {
                                if def.ty == FunctionTy::Function {
                                    let attributes = &def.attributes;

                                    // Check if the function has the `nonReentrant` modifier
                                    if attributes.iter().any(|attr| match attr {
                                        FunctionAttribute::BaseOrModifier(_, base) => {
                                            base.name.to_string() == "nonReentrant"
                                        }
                                        _ => false,
                                    }) {
                                        // Check if there is another modifier before `nonReentrant`
                                        let found_non_reentrant =
                                            attributes.iter().any(|attr| match attr {
                                                FunctionAttribute::BaseOrModifier(_, base) => {
                                                    base.name.to_string() == "nonReentrant"
                                                }
                                                _ => false,
                                            });

                                        let mut non_reentrant_index: u8 = 0;
                                        let mut current_index: u8 = 0;

                                        let mut attribute_zero = &def.attributes[0];

                                        for attribute in &def.attributes {
                                            match attribute {
                                                FunctionAttribute::BaseOrModifier(_, base) => {
                                                    if (base.name.to_string() == "nonReentrant") {
                                                        non_reentrant_index = current_index;
                                                    }
                                                }
                                                _ => (),
                                            }
                                            current_index += 1;
                                        }

                                        let mut detected: bool = false;
                                        // If the index is greater than zero
                                        // and the previous element has the BaseOrModifier type, treat this as detected
                                        if current_index > 0 {
                                            if let FunctionAttribute::BaseOrModifier(_, base) =
                                                &def.attributes[(current_index - 1) as usize]
                                            {
                                                if let Base { .. } = base {
                                                    detected = true;
                                                }
                                            }
                                        }

                                        // print!("\n{:?} {:?}\n", detected, current_index);

                                        // Means that the function has more than one modifier before "nonReentrant"
                                        // position 0 will be for the visibility
                                        // position 1 should be nonReentrant
                                        if detected {
                                            // println!(
                                            //         "\n\nFunction {:?} has `nonReentrant` modifier and \
                                            //         another modifier before it",
                                            //         &def,
                                            //     );

                                            // let extracted_metadata =
                                            //     extract_line_from_content(&parsed_file, &def.loc);

                                            // let issue_appearance: IssueAppearance =
                                            //     IssueAppearance {
                                            //         file_path: (parsed_file.file_path.clone()),
                                            //         line: extracted_metadata.line,
                                            //         content: extracted_metadata.content.to_owned(),
                                            //     };
                                            // self.detected_issues.push(issue_appearance);
                                        }
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
