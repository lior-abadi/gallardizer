use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use solang_parser::pt::Expression;

pub struct RequireInsteadOfAssert {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for RequireInsteadOfAssert {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let function_calls = extract_target_from_node(
            Target::FunctionCall,
            parsed_file.parsed_ast_tree.clone().into(),
        );

        for function_call in function_calls {
            let some_call_expression = function_call.expression();
            if let Some(call_expression) = some_call_expression {
                if let Expression::FunctionCall(loc, body, _) = call_expression {
                    match *body {
                        Expression::Variable(identifier) => {
                            if identifier.name == "assert" {
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

    fn get_detected_issues(&self) -> Vec<IssueAppearance> {
        return self.detected_issues.clone();
    }

    fn get_detector_name(&self) -> String {
        return "CentralizationRisk".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::L,
            title: indoc! {"Use `require()` over `assert()` for error avoidance and effective gas management"}.to_string(),
            content: indoc! {
            "Utilizing `assert` in versions of Solidity before `0.8.0` can exhaust a transaction's 
            remaining gas rather than conserving it, as is the case with `require()` and `revert()`. 
            Beyond `0.8.0`, `assert()` still merits caution as it introduces a `Panic` 
            [error](https://docs.soliditylang.org/en/v0.8.14/control-structures.html#panic-via-assert-and-error-via-require) when triggered, which, 
            under proper contract operation, should not happen even with incorrect external input. 
            Therefore, the occurrence of such an error could be symptomatic of an underlying contract bug. 
            Consequently, adopting `require()` over `assert()` for input and condition validation can promote better gas 
            optimization and prevent `Panic` errors, reinforcing the reliability of your contract."}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}
