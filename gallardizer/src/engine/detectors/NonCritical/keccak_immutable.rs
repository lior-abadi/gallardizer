use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use solang_parser::pt::{ContractPart, Expression, SourceUnitPart, VariableAttribute};

pub struct KeccakImmutable {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for KeccakImmutable {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let variable_definitions = extract_target_from_node(
            Target::VariableDefinition,
            parsed_file.parsed_ast_tree.clone().into(),
        );

        for variable_def_node in variable_definitions {
            let some_contract_part: Option<ContractPart> =
                variable_def_node.clone().contract_part();
            let some_source_part: Option<SourceUnitPart> = variable_def_node.source_unit_part();
            let mut is_constant: bool = false;

            // Handle elements declared inside contracts
            if let Some(contract_part) = some_contract_part {
                match contract_part {
                    ContractPart::VariableDefinition(def) => {
                        for attribute in &def.attrs {
                            match attribute {
                                VariableAttribute::Constant(_) => {
                                    is_constant = true;
                                    break;
                                }
                                _ => (),
                            }
                        }
                        if is_constant {
                            if let Some(expression) = &def.initializer {
                                match expression {
                                    Expression::FunctionCall(_, variable, _) => {
                                        match *variable.clone() {
                                            Expression::Variable(variable_identifier) => {
                                                // Detection condition
                                                if variable_identifier.name == "keccak256" {
                                                    self.detected_issues.push(
                                                        get_appearance_metadata(
                                                            &def.loc,
                                                            parsed_file,
                                                        ),
                                                    );
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }

            // Handle elements declared outside contracts
            if let Some(source_part) = some_source_part {
                match source_part {
                    SourceUnitPart::VariableDefinition(def) => {
                        for attribute in &def.attrs {
                            match attribute {
                                VariableAttribute::Constant(_) => {
                                    is_constant = true;
                                    break;
                                }
                                _ => (),
                            }
                        }
                        if is_constant {
                            if let Some(expression) = &def.initializer {
                                match expression {
                                    Expression::FunctionCall(_, variable, _) => {
                                        match *variable.clone() {
                                            Expression::Variable(variable_identifier) => {
                                                // Detection condition
                                                if variable_identifier.name == "keccak256" {
                                                    self.detected_issues.push(
                                                        get_appearance_metadata(
                                                            &def.loc,
                                                            parsed_file,
                                                        ),
                                                    );
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    fn get_detected_issues(&self) -> Vec<IssueAppearance> {
        return self.detected_issues.clone();
    }

    fn get_detector_name(&self) -> String {
        return "KeccakImmutable".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::NC,
            title: indoc! {"Expressions defining constant values should employ `immutable` instead of `constant`"}.to_string(),
            content: indoc! {
            "It's important to distinguish between `constant` and `immutable` variables, 
            using each in their appropriate situations. Constants are suitable for literal values 
            hard-coded into the contracts, while `immutables` should be used for expression-based values, such as a call to `keccak256()`, 
            or those calculated/introduced in the `constructor`."}
            .to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}
