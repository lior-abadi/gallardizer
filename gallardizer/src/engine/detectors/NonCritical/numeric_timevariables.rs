use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use solang_parser::pt::{ContractPart, Expression, SourceUnitPart};

pub struct NumericTimeVariables {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for NumericTimeVariables {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let variable_definitions = extract_target_from_node(
            Target::VariableDefinition,
            parsed_file.parsed_ast_tree.clone().into(),
        );

        for variable_def_node in variable_definitions {
            let some_contract_part: Option<ContractPart> =
                variable_def_node.clone().contract_part();
            let some_source_part: Option<SourceUnitPart> = variable_def_node.source_unit_part();

            // Handle elements declared inside contracts
            if let Some(contract_part) = some_contract_part {
                match contract_part {
                    ContractPart::VariableDefinition(def) => {
                        if let Some(name_identifier) = &def.name {
                            let variable_name = &name_identifier.name.trim().to_lowercase();

                            if contains_time_keywords(variable_name) {
                                // Check if it is initialized with a time figure
                                if let Some(literal) = def.initializer {
                                    match literal {
                                        Expression::NumberLiteral(
                                            num_loc,
                                            _string_val,
                                            _exp,
                                            some_identifier,
                                        ) => match some_identifier {
                                            // If no time figure is found, we detected the issue
                                            None => {
                                                self.detected_issues.push(get_appearance_metadata(
                                                    &num_loc,
                                                    parsed_file,
                                                ));
                                            }
                                            // If there is a figure is found, check if it is a time
                                            Some(identifier) => {
                                                if !contains_time_keywords(&identifier.name) {
                                                    self.detected_issues.push(
                                                        get_appearance_metadata(
                                                            &num_loc,
                                                            parsed_file,
                                                        ),
                                                    );
                                                }
                                            }
                                        },
                                        _ => (),
                                    }
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
                        if let Some(name_identifier) = &def.name {
                            let variable_name = &name_identifier.name.trim().to_lowercase();

                            if contains_time_keywords(variable_name) {
                                // Check if it is initialized with a time figure
                                if let Some(literal) = def.initializer {
                                    match literal {
                                        Expression::NumberLiteral(
                                            num_loc,
                                            _string_val,
                                            _exp,
                                            some_identifier,
                                        ) => match some_identifier {
                                            // If no time figure is found, we detected the issue
                                            None => {
                                                self.detected_issues.push(get_appearance_metadata(
                                                    &num_loc,
                                                    parsed_file,
                                                ));
                                            }
                                            // If there is a figure is found, check if it is a time
                                            Some(identifier) => {
                                                if !contains_time_keywords(&identifier.name) {
                                                    self.detected_issues.push(
                                                        get_appearance_metadata(
                                                            &num_loc,
                                                            parsed_file,
                                                        ),
                                                    );
                                                }
                                            }
                                        },
                                        _ => (),
                                    }
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
        return "MissingIndexedFields".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::NC,
            title: indoc! {"Time-related numeric values could employ time units"}.to_string(),
            content: indoc! {
            "For readability and consistency, numeric values associated with time should
            utilize predefined [units](https://docs.soliditylang.org/en/latest/units-and-global-variables.html#time-units) 
            like seconds, minutes, hours, days, or weeks."}
            .to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}

fn contains_time_keywords(variable_name: &str) -> bool {
    let time_keywords = [
        "second",
        "minute",
        "hour",
        "day",
        "week",
        "month",
        "year",
        "duration",
        "interval",
        "schedule",
        "deadline",
        "expiry",
        "timeout",
        "countdown",
        "frequency",
        "recurring",
        "elapsed",
        "timing",
        "clock",
        "period",
    ];

    for keyword in time_keywords {
        if variable_name.contains(keyword) {
            return true;
        }
    }
    return false;
}
