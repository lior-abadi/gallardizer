use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use solang_parser::pt::{ContractPart, FunctionAttribute, IdentifierPath};

pub struct CentralizationRisk {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for CentralizationRisk {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let target_nodes = extract_target_from_node(
            Target::FunctionDefinition,
            parsed_file.parsed_ast_tree.clone().into(),
        );

        for node in target_nodes {
            // Functions could be defined either inside a contract or outside
            let some_contract_part: Option<ContractPart> = node.clone().contract_part();

            if let Some(contract_part) = some_contract_part {
                // FunctionDefinition is a contract part from now
                if let ContractPart::FunctionDefinition(def) = contract_part {
                    for attribute in &def.attributes {
                        match attribute {
                            FunctionAttribute::BaseOrModifier(loc, base) => {
                                let IdentifierPath { identifiers, .. } = &base.name;

                                for identifier in identifiers {
                                    let modifier_name = identifier.name.to_lowercase();
                                    if modifier_name.contains("owner")
                                        || modifier_name.contains("only")
                                        || modifier_name.contains("auth")
                                        || modifier_name.contains("allow")
                                    {
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
            severity: Severities::M,
            title: indoc! {"Excess of trust required due to centralization risks"}.to_string(),
            content: indoc! {
            "The existence of contract owners with administrative privileges can introduce a single 
            point of failure due to the inherent centralization risk. These privileged owners are entrusted 
            not to execute harmful updates or illicitly withdraw funds, emphasizing the need for trustworthiness 
            in these roles to maintain the security and integrity of the contract's operation."}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}
