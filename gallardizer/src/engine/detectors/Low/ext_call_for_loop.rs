use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;
use solang_parser::pt::{Expression, Identifier, Loc};

pub struct ExternalCallInsideForLoopDoS {
    pub detected_issues: Vec<IssueAppearance>,
}
impl Detector for ExternalCallInsideForLoopDoS {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let target_nodes =
            extract_target_from_node(Target::For, parsed_file.parsed_ast_tree.clone().into());

        for for_loop in target_nodes {
            // We need to detect if array.length is used to bound the loop iterator:
            let member_calls = extract_target_from_node(Target::Less, for_loop.clone());

            let mut detected_unbound_length: bool = false;
            let mut cached_less_loc: Loc = Loc::File(0, 0, 0);

            for part in member_calls {
                let for_loop_less = part.clone().expression().unwrap();

                match for_loop_less {
                    Expression::Less(loc, _right_hand, left_hand) => {
                        cached_less_loc = loc;

                        print!("\n{:?}\n", &left_hand);
                        if let Expression::MemberAccess(_, _, identifier) = *left_hand {
                            detected_unbound_length = &identifier.name == "length";
                        }
                    }

                    _ => (),
                }
            }

            // If the loop is bounded, we assume that this was considered
            if (!detected_unbound_length) {
                continue;
            }

            let external_calls = extract_target_from_node(Target::FunctionCall, for_loop);
            for function_call in external_calls {
                let calls = function_call.clone().expression().unwrap();

                match &calls {
                    Expression::FunctionCall(loc, expr, _params) => {
                        if let Expression::MemberAccess(_, _, _) = **expr {
                            print!("\nDETECTED EXTERNAL CALL\n",);

                            let issue_appearance =
                                get_appearance_metadata(&cached_less_loc, parsed_file);
                            self.detected_issues.push(issue_appearance);
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
        return "ExternalCallInsideForLoopDoS".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::L,
            title: indoc! {"Denial of service risk from unbounded for-loops with external calls"}.to_string(),
            content: indoc! {
            "Unbounded for-loops making external calls pose a Denial of Service (DOS) risk due to potential gas limitations. 
            This can disrupt contract operation and even lead to a halt in functionalities. To enhance contract stability and 
            resilience against DOS attacks, consider limiting the number of iterations in these loops, thereby controlling gas 
            consumption and ensuring smoother execution."}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}
