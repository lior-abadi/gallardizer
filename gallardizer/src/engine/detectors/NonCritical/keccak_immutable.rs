use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use solang_parser::pt::{ContractPart, Expression, SourceUnitPart};

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

            // Handle elements declared inside contracts
            if let Some(contract_part) = some_contract_part {
                match contract_part {
                    ContractPart::VariableDefinition(def) => {}
                    _ => (),
                }
            }

            // Handle elements declared outside contracts
            if let Some(source_part) = some_source_part {
                match source_part {
                    SourceUnitPart::VariableDefinition(def) => {}
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
            title: indoc! {"Expressions for constant values should employ `immutable` instead of `constant`."}.to_string(),
            content: indoc! {
            "It's important to distinguish between `constant` and `immutable` variables, 
            using each in their appropriate situations. Constants are suitable for literal values 
            hard-coded into the contracts, while `immutables` should be used for expression-based values, such as a call to `keccak256()`,  
            or those calculated or introduced in the `constructor`."}
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
