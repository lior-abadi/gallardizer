use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, extract_targets_from_node, Node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use solang_parser::pt::{
    ContractPart, Expression, FunctionAttribute, Identifier, Loc, Mutability, SourceUnitPart,
    Statement, StorageLocation, VariableAttribute,
};

pub struct DivisionByTwoShift {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for DivisionByTwoShift {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let functions = extract_target_from_node(
            Target::FunctionDefinition,
            parsed_file.parsed_ast_tree.clone().into(),
        );

        let global_var_definitions = extract_target_from_node(
            Target::VariableDefinition,
            parsed_file.parsed_ast_tree.clone().into(),
        );

        let mut global_vars: Vec<String> = vec![];
        for var_def_node in global_var_definitions {
            let some_contract_part: Option<ContractPart> = var_def_node.clone().contract_part();
            let some_source_part: Option<SourceUnitPart> = var_def_node.source_unit_part();

            // Handle global declarations in contracts
            if let Some(contract_part) = some_contract_part {
                match contract_part {
                    ContractPart::VariableDefinition(def) => {
                        if let Some(identifier) = def.name {
                            global_vars.push(identifier.name);
                        }
                    }
                    _ => (),
                }
            }

            // Handle global declarations outside contracts
            if let Some(source_part) = some_source_part {
                match source_part {
                    SourceUnitPart::VariableDefinition(def) => {
                        if let Some(identifier) = def.name {
                            global_vars.push(identifier.name);
                        }
                    }
                    _ => (),
                }
            }
        }

        for function_node in functions {
            let some_contract_part: Option<ContractPart> = function_node.clone().contract_part();
            let some_source_part: Option<SourceUnitPart> = function_node.source_unit_part();

            // Handle elements declared inside contracts
            if let Some(contract_part) = some_contract_part {
                // Filter view functions
                let mut is_view_or_pure: bool = false;
                match &contract_part {
                    ContractPart::FunctionDefinition(def) => {
                        for attribute in &def.attributes {
                            match attribute {
                                FunctionAttribute::Mutability(_) => {
                                    is_view_or_pure = true;
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }

                // Skip view or pure functions
                if is_view_or_pure {
                    continue;
                }

                let variable_defs = extract_target_from_node(
                    Target::VariableDefinition,
                    contract_part.clone().into(),
                );

                let locs = detect_issue(variable_defs, &global_vars);
                for loc in locs {
                    self.detected_issues
                        .push(get_appearance_metadata(&loc, parsed_file));
                }
            }

            // Handle elements declared outside contracts
            if let Some(source_part) = some_source_part {}
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

fn detect_issue(variable_defs: Vec<Node>, global_decs: &Vec<String>) -> Vec<Loc> {
    let mut findings: Vec<Loc> = vec![];

    for def_node in variable_defs {
        let some_variable_def: Option<Statement> = def_node.statement();
        if let Some(variable_def) = some_variable_def {
            match variable_def {
                Statement::VariableDefinition(loc, declaration, some_expression) => {
                    // if no storage location was declared, continue
                    // otherwise, get the storage location
                    let mut is_in_memory: bool = false;

                    match declaration.storage {
                        None => {
                            continue;
                        }
                        Some(location) => match location {
                            StorageLocation::Memory(_) => is_in_memory = true,
                            _ => (),
                        },
                    }
                    let mut detected_match: bool = false;
                    if let Some(expression) = some_expression {
                        match expression {
                            Expression::ArraySubscript(_, variable, _) => {
                                if let Expression::Variable(identifier) = *variable {
                                    for var in global_decs {
                                        if &identifier.name == var {
                                            detected_match = true;
                                            break;
                                        }
                                    }
                                }
                            }
                            _ => (),
                        }
                    }

                    // If the variable we are checking was not declared globally, continue
                    if !detected_match {
                        continue;
                    }

                    // Check the type of variable and the detection condition
                    match declaration.ty {
                        Expression::ArraySubscript(_, _, _) | Expression::Variable(_) => {
                            if is_in_memory {
                                findings.push(loc);
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    }

    return findings;
}
