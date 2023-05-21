use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;
use solang_parser::pt::{
    ContractPart, Expression, FunctionDefinition, Identifier, Loc, Parameter, SourceUnitPart,
    Statement,
};

pub struct DivisionByZero {
    pub detected_issues: Vec<IssueAppearance>,
}
impl Detector for DivisionByZero {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let function_definitions = extract_target_from_node(
            Target::FunctionDefinition,
            parsed_file.parsed_ast_tree.clone().into(),
        );

        for function in function_definitions {
            let mut has_require: bool = false;
            // Handle both functions declared inside or outside a contract
            let some_contract_part: Option<ContractPart> = function.clone().contract_part();
            let some_source_part: Option<SourceUnitPart> = function.source_unit_part();

            if let Some(part) = some_contract_part {
                // This condition is pretty severe but ensures that no false positives might arise
                if (has_require_statements(part.clone())) {
                    continue;
                }

                let divide_detection: DivideCheckReturn = divides_by_parameter(part.clone());
                if (divide_detection.detected) {
                    let issue_appearance =
                        get_appearance_metadata(&divide_detection.loc, parsed_file);
                    self.detected_issues.push(issue_appearance);
                }
            }

            if let Some(part) = some_source_part {
                // This condition is pretty severe but ensures that no false positives might arise
                if (has_require_statements1(part.clone())) {
                    continue;
                }

                let divide_detection: DivideCheckReturn = divides_by_parameter1(part.clone());
                if (divide_detection.detected) {
                    let issue_appearance =
                        get_appearance_metadata(&divide_detection.loc, parsed_file);
                    self.detected_issues.push(issue_appearance);
                }
            }
        }
    }

    fn get_detected_issues(&self) -> Vec<IssueAppearance> {
        return self.detected_issues.clone();
    }

    fn get_detector_name(&self) -> String {
        return "DivisionByZero".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::L,
            title: indoc! {"Reversals due to division by zero"}.to_string(),
            content: indoc! {
            "A division operation lacks necessary zero-value checks on any parameter used as denominator, 
            which could result in the function reverting if zero is passed as an argument. It's crucial to implement 
            safeguards against such division by zero errors to prevent unexpected function reverts and maintain the 
            integrity of each contract's calculations."}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}

fn has_require_statements(contract_part: ContractPart) -> bool {
    match contract_part {
        ContractPart::FunctionDefinition(def) => {
            if let Some(Statement::Block {
                loc: _,
                unchecked: _,
                statements,
            }) = &def.body
            {
                for function_body_members in statements {
                    let function_calls = extract_target_from_node(
                        Target::FunctionCall,
                        function_body_members.clone().into(),
                    );

                    for function_call in function_calls {
                        let call_expression = function_call.expression().unwrap();
                        if let Expression::FunctionCall(_, body, _) = call_expression {
                            match *body {
                                Expression::Variable(identifier) => {
                                    return identifier.name == "require";
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
        _ => (),
    }
    return false;
}

fn has_require_statements1(source_part: SourceUnitPart) -> bool {
    match source_part {
        SourceUnitPart::FunctionDefinition(def) => {
            if let Some(Statement::Block {
                loc: _,
                unchecked: _,
                statements,
            }) = &def.body
            {
                for function_body_members in statements {
                    let function_calls = extract_target_from_node(
                        Target::FunctionCall,
                        function_body_members.clone().into(),
                    );

                    for function_call in function_calls {
                        let call_expression = function_call.expression().unwrap();
                        if let Expression::FunctionCall(_, body, _) = call_expression {
                            match *body {
                                Expression::Variable(identifier) => {
                                    return identifier.name == "require";
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
        _ => (),
    }
    return false;
}

struct DivideCheckReturn {
    detected: bool,
    loc: Loc,
}

fn divides_by_parameter(contract_part: ContractPart) -> DivideCheckReturn {
    let mut params: Vec<String> = vec![];
    match contract_part {
        ContractPart::FunctionDefinition(def) => {
            for (_, parameter) in &def.params {
                if let Some(Parameter {
                    name: Some(Identifier { name, .. }),
                    ..
                }) = parameter
                {
                    params.push(name.to_string());
                }
            }

            if let Some(Statement::Block {
                loc: _,
                unchecked: _,
                statements,
            }) = &def.body
            {
                for function_body_members in statements {
                    let divides_in_tree = extract_target_from_node(
                        Target::Divide,
                        function_body_members.clone().into(),
                    );

                    for divide_op in divides_in_tree {
                        let divide_expression = divide_op.expression().unwrap();
                        match divide_expression {
                            Expression::Divide(loc, _numerator, denominator) => {
                                if let Expression::Variable(Identifier { name, .. }) = *denominator
                                {
                                    return DivideCheckReturn {
                                        detected: params.contains(&name),
                                        loc,
                                    };
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

    return DivideCheckReturn {
        detected: false,
        loc: Loc::Builtin,
    };
}

fn divides_by_parameter1(contract_part: SourceUnitPart) -> DivideCheckReturn {
    let mut params: Vec<String> = vec![];
    match contract_part {
        SourceUnitPart::FunctionDefinition(def) => {
            for (_, parameter) in &def.params {
                if let Some(Parameter {
                    name: Some(Identifier { name, .. }),
                    ..
                }) = parameter
                {
                    params.push(name.to_string());
                }
            }

            if let Some(Statement::Block {
                loc: _,
                unchecked: _,
                statements,
            }) = &def.body
            {
                for function_body_members in statements {
                    let divides_in_tree = extract_target_from_node(
                        Target::Divide,
                        function_body_members.clone().into(),
                    );

                    for divide_op in divides_in_tree {
                        let divide_expression = divide_op.expression().unwrap();
                        match divide_expression {
                            Expression::Divide(loc, _numerator, denominator) => {
                                if let Expression::Variable(Identifier { name, .. }) = *denominator
                                {
                                    return DivideCheckReturn {
                                        detected: params.contains(&name),
                                        loc,
                                    };
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

    return DivideCheckReturn {
        detected: false,
        loc: Loc::Builtin,
    };
}
