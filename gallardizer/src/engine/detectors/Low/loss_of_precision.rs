use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;
use solang_parser::pt::Expression;

pub struct LossOfPrecision {
    pub detected_issues: Vec<IssueAppearance>,
}
impl Detector for LossOfPrecision {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let target_nodes =
            extract_target_from_node(Target::Divide, parsed_file.parsed_ast_tree.clone().into());

        for node in target_nodes {
            let expression = node.expression().unwrap();

            match expression {
                Expression::Divide(loc, _numerator, nominator) => match *nominator {
                    Expression::NumberLiteral(_, base, exp, _) => {
                        let mut exp_value: u32 = 0;
                        let base_value: u128;
                        let base_ten: u128 = 10;

                        if exp.len() > 0 {
                            exp_value = exp.parse::<u32>().unwrap();
                        }

                        base_value = base.parse::<u128>().unwrap();
                        let numerator_value = base_value * base_ten.pow(exp_value);

                        if numerator_value > base_ten.pow(4) {
                            let issue_appearance = get_appearance_metadata(&loc, parsed_file);
                            self.detected_issues.push(issue_appearance);
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    fn get_detected_issues(&self) -> Vec<IssueAppearance> {
        return self.detected_issues.clone();
    }

    fn get_detector_name(&self) -> String {
        return "LossOfPrecision".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::L,
            title: indoc! {"Potential precision loss from division with large numbers"}.to_string(),
            content: indoc! {
            "Division operations with large denominators in Solidity may result in a return value of 
            zero due to its lack of fractional number support. It's crucial to address this by ensuring 
            the numerator is always greater than the denominator. A suggested safeguard is to set a required 
            minimum value for the numerator, mitigating the risk of unexpected precision loss and improving the 
            accuracy of your computations."}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}
