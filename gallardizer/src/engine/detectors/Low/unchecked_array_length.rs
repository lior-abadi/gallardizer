use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, Node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use solang_parser::pt::{
    ContractPart, EventDefinition, Expression, FunctionDefinition, SourceUnitPart, Statement,
};

pub struct UncheckedArrayLength {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for UncheckedArrayLength {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let functions = extract_target_from_node(
            Target::FunctionDefinition,
            parsed_file.parsed_ast_tree.clone().into(),
        );

        for function in functions {
            // Functions could be defined either inside a contract or outside
            let some_contract_part: Option<ContractPart> = function.clone().contract_part();
            let some_source_part: Option<SourceUnitPart> = function.source_unit_part();

            // Handle functions declared inside contracts
            if let Some(contract_part) = some_contract_part {
                match &contract_part {
                    ContractPart::FunctionDefinition(def) => {
                        let array_inputs = get_function_array_inputs(def);

                        // To have an array len mismatch, there should be at least two
                        if array_inputs.len() < 2 {
                            continue;
                        }

                        // Check if there are for loops using those inputs
                        let uses_inputs_in_loop = has_for_loops_using_inputs(def, &array_inputs);

                        // We skip if no inputs are inside for loops
                        if (!uses_inputs_in_loop) {
                            continue;
                        }

                        // If we reached this point, it means that there are more than 2 inputs
                        // And they are being used in a for loop
                        // We need to scan if their lengths are checked at any point
                        let mut inputs_checked: usize = 0; // checked need to be == array_inputs.len()
                        for input in &array_inputs {
                            if has_revert_checks(def, input) {
                                inputs_checked += 1;
                            }
                        }

                        // Detection condition
                        if inputs_checked < array_inputs.len() {
                            println!("DETECTED");

                            self.detected_issues
                                .push(get_appearance_metadata(&def.loc, parsed_file));
                        }
                    }
                    _ => (),
                }
            }

            // Handle functions declared outside contracts
            if let Some(source_part) = some_source_part {
                // if let SourceUnitPart::EventDefinition(event_definition) = source_part {
                //     if has_non_indexed_field(&event_definition) {
                //         self.detected_issues
                //             .push(get_appearance_metadata(&event_definition.loc, parsed_file));
                //     }
                // }
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
            severity: Severities::L,
            title: indoc! {"Absence of array length validation"}.to_string(),
            content: indoc! {
            "Without explicit checks for arrays to have the same length, user operations 
            might not be completely executed. This is due to the disparity between the number of items 
            involved in the iteration and the number of items in the subsequent arrays."}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}

fn get_function_array_inputs(def: &Box<FunctionDefinition>) -> Vec<&String> {
    let mut array_args: Vec<&String> = vec![];
    for param in &def.params {
        if let Some(parameter) = &param.1 {
            match parameter.ty {
                Expression::ArraySubscript(_, _, _) => {
                    if let Some(identifier) = &parameter.name {
                        array_args.push(&identifier.name);
                    }
                }
                _ => (),
            }
        }
    }
    return array_args;
}
fn has_for_loops_using_inputs(
    def: &Box<FunctionDefinition>,
    targets_to_check: &Vec<&String>,
) -> bool {
    if let Some(Statement::Block {
        loc: _,
        unchecked: _,
        statements,
    }) = &def.body
    {
        // To have a potential mismatch, at least two statements need to be used
        let mut inputs_in_for_loop: u8 = 0;
        for function_body_members in statements {
            // Require and assert are function calls
            let for_loops =
                extract_target_from_node(Target::For, function_body_members.clone().into());

            for for_loop_node in for_loops {
                let some_loop = for_loop_node.statement();

                if let Some(for_loop) = some_loop {
                    // We should look for member access (input .length) calls
                    // Return if two different inputs are invoked
                    // Require and assert are function calls
                    let variable_invocations =
                        extract_target_from_node(Target::Variable, for_loop.clone().into());

                    let mut retrieved_targets: Vec<String> = vec![];
                    for to_check in targets_to_check {
                        let retrieved_member =
                            get_variable_with_criteria(variable_invocations.clone(), *to_check);

                        retrieved_targets.push(retrieved_member);
                    }

                    for retrieved_target in retrieved_targets {
                        if targets_to_check.contains(&&retrieved_target) {
                            inputs_in_for_loop += 1;
                        }
                    }

                    return inputs_in_for_loop > 1;
                }
            }
        }
    }

    return false;
}

// Checks if the target variables are inside a statement that leads to a revert
fn has_revert_checks(def: &Box<FunctionDefinition>, target_to_check: &str) -> bool {
    if let Some(Statement::Block {
        loc: _,
        unchecked: _,
        statements,
    }) = &def.body
    {
        for function_body_members in statements {
            // Require and assert are function calls
            let function_calls = extract_target_from_node(
                Target::FunctionCall,
                function_body_members.clone().into(),
            );

            // Reverting with errors is triggered by If statements
            let if_statements =
                extract_target_from_node(Target::If, function_body_members.clone().into());

            let mut match_detected: bool = false;

            // Handle Require and Assert
            for function_call in function_calls {
                let some_call_expression = function_call.expression();
                if let Some(call_expression) = some_call_expression {
                    if let Expression::FunctionCall(_, body, expressions) = call_expression {
                        match *body {
                            Expression::Variable(identifier) => {
                                // The following condition is ideal
                                for expression in expressions {
                                    let member_access: Vec<Node> = extract_target_from_node(
                                        Target::MemberAccess,
                                        expression.clone().into(),
                                    );

                                    for node_member in member_access {
                                        let some_member = node_member.expression();
                                        if let Some(member) = some_member {
                                            match member {
                                                Expression::MemberAccess(
                                                    _,
                                                    member_expression,
                                                    length_identifier,
                                                ) => {
                                                    // Get the variable name
                                                    match *member_expression {
                                                        Expression::Variable(var_identifier) => {
                                                            if length_identifier.name == "length" {
                                                                match_detected = target_to_check
                                                                    == var_identifier.name;

                                                                // Early return as we already found the match
                                                                if (match_detected) {
                                                                    return match_detected;
                                                                }
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
                            }
                            _ => (),
                        }
                    }
                }
            }
            // Handle if statements
            for if_statement_node in if_statements {
                let some_if_statement = if_statement_node.statement();

                if let Some(if_statement) = some_if_statement {
                    let member_access: Vec<Node> =
                        extract_target_from_node(Target::MemberAccess, if_statement.clone().into());

                    for node_member in member_access {
                        let some_member = node_member.expression();
                        if let Some(member) = some_member {
                            match member {
                                Expression::MemberAccess(
                                    _,
                                    member_expression,
                                    length_identifier,
                                ) => {
                                    // Get the variable name
                                    match *member_expression {
                                        Expression::Variable(var_identifier) => {
                                            if length_identifier.name == "length" {
                                                match_detected =
                                                    target_to_check == var_identifier.name;

                                                // Early return as we already found the match
                                                if (match_detected) {
                                                    return match_detected;
                                                }
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
            }
        }
    }
    return false;
}

fn get_variable_with_criteria(variables: Vec<Node>, target_to_check: &str) -> String {
    let processed_target_to_check = target_to_check.trim().to_lowercase();
    let mut return_string = String::new();

    for variable_node in variables {
        let some_var = variable_node.expression();
        if let Some(variable) = some_var {
            match variable {
                Expression::Variable(identifier) => {
                    if identifier.name == processed_target_to_check {
                        return_string = identifier.name;
                    }
                }
                _ => (),
            }
        }
    }

    return return_string;
}
