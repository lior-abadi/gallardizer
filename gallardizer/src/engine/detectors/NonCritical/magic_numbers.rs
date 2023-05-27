use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, extract_targets_from_node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use solang_parser::pt::{ContractPart, EventDefinition, Expression, SourceUnitPart};

pub struct MagicNumbers {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for MagicNumbers {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let functions = extract_target_from_node(
            Target::FunctionDefinition,
            parsed_file.parsed_ast_tree.clone().into(),
        );

        for function_node in functions {
            let some_contract_part: Option<ContractPart> = function_node.clone().contract_part();
            let some_source_part: Option<SourceUnitPart> = function_node.source_unit_part();

            // Handle elements declared inside contracts
            if let Some(contract_part) = some_contract_part {
                // We extract all the literals
                let literals = extract_targets_from_node(
                    vec![Target::HexNumberLiteral, Target::NumberLiteral],
                    contract_part.clone().into(),
                );

                for literal_node in literals {
                    let some_literal = literal_node.expression();
                    // If we enter this point, means that there are some literals
                    if let Some(literal) = some_literal {
                        match literal {
                            Expression::HexNumberLiteral(loc, string_hex, _) => {
                                if string_hex != "0x00" && string_hex != "0x0" {
                                    self.detected_issues
                                        .push(get_appearance_metadata(&loc, parsed_file));
                                }
                            }

                            Expression::NumberLiteral(loc, string_value, string_exp, _) => {
                                if (string_value != "0"
                                    && string_value != "1"
                                    && string_value != "2")
                                    && string_exp == ""
                                {
                                    self.detected_issues
                                        .push(get_appearance_metadata(&loc, parsed_file));
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }

            // Handle elements declared outside contracts
            if let Some(source_part) = some_source_part {
                let literals = extract_targets_from_node(
                    vec![Target::HexNumberLiteral, Target::NumberLiteral],
                    source_part.clone().into(),
                );
                for literal_node in literals {
                    let some_literal = literal_node.expression();
                    // If we enter this point, means that there are some literals
                    if let Some(literal) = some_literal {
                        match literal {
                            Expression::HexNumberLiteral(loc, string_hex, _) => {
                                if string_hex != "0x00" && string_hex != "0x0" {
                                    self.detected_issues
                                        .push(get_appearance_metadata(&loc, parsed_file));
                                }
                            }

                            Expression::NumberLiteral(loc, string_value, string_exp, _) => {
                                if (string_value != "0"
                                    && string_value != "1"
                                    && string_value != "2")
                                    && string_exp == ""
                                {
                                    self.detected_issues
                                        .push(get_appearance_metadata(&loc, parsed_file));
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    fn get_detected_issues(&self) -> Vec<IssueAppearance> {
        return self.detected_issues.clone();
    }

    fn get_detector_name(&self) -> String {
        return "MissingIndexedFields".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::NC,
            title: indoc! {"Avoid using magic numbers"}.to_string(),
            content: indoc! {
            "It is recommended to define constants instead of relying on hex or numeric literals.
            This practice enhances readability and clarity, even in assembly context, 
            thereby mitigating the potential for confusion or error."}
            .to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}
