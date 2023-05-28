use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, Node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use solang_parser::pt::{ContractPart, Expression, Loc, SourceUnitPart};

pub struct DivisionByTwoShift {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for DivisionByTwoShift {
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
                let divisions =
                    extract_target_from_node(Target::Divide, contract_part.clone().into());

                let detected_divisions = detect_issue(divisions);

                for detected in detected_divisions {
                    self.detected_issues
                        .push(get_appearance_metadata(&detected, parsed_file));
                }
            }

            // Handle elements declared outside contracts
            if let Some(source_part) = some_source_part {
                let divisions =
                    extract_target_from_node(Target::Divide, source_part.clone().into());

                let detected_divisions = detect_issue(divisions);

                for detected in detected_divisions {
                    self.detected_issues
                        .push(get_appearance_metadata(&detected, parsed_file));
                }
            }
        }
    }

    fn get_detected_issues(&self) -> Vec<IssueAppearance> {
        return self.detected_issues.clone();
    }

    fn get_detector_name(&self) -> String {
        return "DivisionByTwoShift".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::Gas,
            title: indoc! {"Use bit shifting for division by two"}.to_string(),
            content: indoc! {
            "The expression `<x> / 2` has the same result as `<x> >> 1`.
            Despite the compiler's use of the `SHR` opcode for both processes, 
            the division form involves an additional gas expense of `20` due to 
            redirects to a compiler utility function that adds checks. These 
            checks can be bypassed by incorporating `unchecked {}` when dividing by two."}
            .to_string(),
            gas_saved_per_instance: 20,
        };

        return metadata;
    }
}

fn detect_issue(division_nodes: Vec<Node>) -> Vec<Loc> {
    let mut findings: Vec<Loc> = vec![];

    for division_node in division_nodes {
        let some_division_expression = division_node.expression();
        if let Some(division_expression) = some_division_expression {
            if let Expression::Divide(loc, _, denominator) = division_expression {
                match *denominator {
                    Expression::NumberLiteral(_, string_value, _, _) => {
                        if string_value == "2" {
                            findings.push(loc);
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    return findings;
}
