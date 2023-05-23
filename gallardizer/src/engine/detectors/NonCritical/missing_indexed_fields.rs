use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use solang_parser::pt::{
    ContractPart, EventDefinition, FunctionAttribute, IdentifierPath, SourceUnitPart,
};

pub struct MissingIndexedFields {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for MissingIndexedFields {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let events = extract_target_from_node(
            Target::EventDefinition,
            parsed_file.parsed_ast_tree.clone().into(),
        );

        for event in events {
            // Events could be defined either inside a contract or outside
            let some_contract_part: Option<ContractPart> = event.clone().contract_part();
            let some_source_part: Option<SourceUnitPart> = event.source_unit_part();

            // Handle events declared inside contracts
            if let Some(contract_part) = some_contract_part {
                if let ContractPart::EventDefinition(event_definition) = contract_part {
                    if has_non_indexed_field(&event_definition) {
                        self.detected_issues
                            .push(get_appearance_metadata(&event_definition.loc, parsed_file));
                    }
                }
            }

            // Handle events declared outside contracts
            if let Some(source_part) = some_source_part {
                if let SourceUnitPart::EventDefinition(event_definition) = source_part {
                    if has_non_indexed_field(&event_definition) {
                        self.detected_issues
                            .push(get_appearance_metadata(&event_definition.loc, parsed_file));
                    }
                }
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
            severity: Severities::NC,
            title: indoc! {"Inadequate indexing of event fields"}.to_string(),
            content: indoc! {
            "Indexed event fields enhance accessibility for off-chain tools parsing events, 
            proving particularly beneficial for address-based filtering. However, gas costs increase with each 
            indexed field during emission, posing a challenge in maximizing the use of the allowable three fields per event. 
            Events with three or more fields should ideally utilize all three indexed fields, provided that gas usage is not a 
            significant concern. In events with fewer than three fields, it's advisable to index all applicable fields, balancing 
            quick accessibility and efficient gas consumption"}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}

fn has_non_indexed_field(event_definition: &Box<EventDefinition>) -> bool {
    let mut has_non_indexed_param: bool = false;
    let mut amount_of_indexed_fields: u16 = 0;
    for field in &event_definition.fields {
        if field.indexed {
            amount_of_indexed_fields += 1;
            // We keep looping
            continue;
        }
        // If we reach this point, means that we haven't entered the if branch (as it continues)
        has_non_indexed_param = true;
    }

    // If the event has at least one non indexed field and has less than 3, flag it.
    return has_non_indexed_param && amount_of_indexed_fields < 3;
}
