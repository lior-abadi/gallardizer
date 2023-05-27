use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use solang_parser::pt::{Expression, Statement};

pub struct RevertStrings {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for RevertStrings {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        // As Revert is a Statement and require an expression, we need to browse both targets
        let function_calls = extract_target_from_node(
            Target::FunctionCall,
            parsed_file.parsed_ast_tree.clone().into(),
        );

        let reverts =
            extract_target_from_node(Target::Revert, parsed_file.parsed_ast_tree.clone().into());

        // Require Handling
        for function_call in function_calls {
            let some_call_expression = function_call.expression();
            if let Some(call_expression) = some_call_expression {
                let mut has_require_strings: bool = false;
                // Here we evaluate the require statements for each function
                if let Expression::FunctionCall(loc, body, parameters) = call_expression {
                    match *body {
                        Expression::Variable(identifier) => {
                            if identifier.name == "require" {
                                for parameter in parameters {
                                    match parameter {
                                        Expression::StringLiteral(_lit) => {
                                            // If enters here, means that has a revert string
                                            has_require_strings = true;
                                        }
                                        _ => (),
                                    }
                                }
                                if !has_require_strings {
                                    self.detected_issues
                                        .push(get_appearance_metadata(&loc, parsed_file));
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        // Revert handling
        for revert in reverts {
            let some_statement = revert.statement();
            if let Some(statement) = some_statement {
                let mut has_revert_strings: bool = false;
                if let Statement::Revert(loc, identifier, expressions) = statement {
                    // We filter reverts triggered with custom errors
                    if let Some(_path) = identifier {
                        continue;
                    }

                    // If we've reached this level, means that the revert found is not a custom error
                    for expression in expressions {
                        match expression {
                            Expression::StringLiteral(_lit) => {
                                // If enters here, means that has a revert string
                                has_revert_strings = true;
                            }
                            _ => (),
                        }
                    }
                    if !has_revert_strings {
                        self.detected_issues
                            .push(get_appearance_metadata(&loc, parsed_file));
                    }
                }
            }
        }
    }

    fn get_detected_issues(&self) -> Vec<IssueAppearance> {
        return self.detected_issues.clone();
    }

    fn get_detector_name(&self) -> String {
        return "RevertStrings".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::NC,
            title: indoc! {"Add descriptive revert reasons"}.to_string(),
            content: indoc! {
            "Include descriptive reason strings in `require()` and `revert()` for 
            improved error handling and user feedback. Since Solidity `0.8.4`, 
            [custom errors](https://blog.soliditylang.org/2021/04/21/custom-errors/) offer a concise, 
            detailed alternative for reversion, facilitating better contract usability and debugging 
            also providing a more efficient way of reverting."}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}
