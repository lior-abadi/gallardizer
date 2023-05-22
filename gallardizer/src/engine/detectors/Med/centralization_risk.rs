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

            // Since functions defined outside contracts cannot have modifiers, we skip them.
            // let some_source_part: Option<SourceUnitPart> = node.source_unit_part();

            if let Some(contract_part) = some_contract_part {
                // FunctionDefinition is a contract part from now
                if let ContractPart::FunctionDefinition(def) = contract_part {
                    for attribute in &def.attributes {
                        match attribute {
                            FunctionAttribute::BaseOrModifier(loc, base) => {
                                let IdentifierPath { identifiers, .. } = &base.name;

                                for identifier in identifiers {
                                    if identifier.name.contains("onlyOwner") {
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
            title: indoc! {"Single point of failure due to centralization risk"}.to_string(),
            content: indoc! {
            "The presence of a single account as the sole owner of a contract presents centralization risks and introduces a 
            single point of failure. This setup leaves the contract vulnerable to loss, theft, or unavailability of the 
            private key (if the owner is an external owned account). To mitigate these risks, consider transitioning to a 
            multi-signature setup or adopting a role-based authorization model, segregating roles and their privileges as a 
            part of a defense in depth strategy. This way, responsibilities and powers can be distributed, enhancing security, 
            reducing centralization, and providing robust protection against potential adversarial events."}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}
