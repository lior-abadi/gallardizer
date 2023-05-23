use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use solang_parser::pt::{Import, SourceUnitPart};

pub struct SolmateSafeTransfer {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for SolmateSafeTransfer {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        let import_directives = extract_target_from_node(
            Target::ImportDirective,
            parsed_file.parsed_ast_tree.clone().into(),
        );

        for import_directive in import_directives {
            let some_source_unit = import_directive.source_unit_part();

            if let Some(source_unit) = some_source_unit {
                match source_unit {
                    SourceUnitPart::ImportDirective(import) => {
                        if let Import::Plain(literal, loc) = import {
                            let import_route = literal.string.to_lowercase();
                            if import_route.contains("solmate")
                                && import_route.contains("safetransferlib.sol")
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

    fn get_detected_issues(&self) -> Vec<IssueAppearance> {
        return self.detected_issues.clone();
    }

    fn get_detector_name(&self) -> String {
        return "SolmateSafeTransfer".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::M,
            title: indoc! {"Absence of token contract existence check in Solmate's `SafeTransferLib`"}.to_string(),
            content: indoc! {
            "Safe-wrapped methods could be called to non existent token contracts without reverting.
            A key distinction exists between Solmate's `SafeTransferLib` and OpenZeppelin's
            `SafeERC20`. While `SafeERC20` validates whether the token is a contract, 
            SafeTransferLib does not perform this check. As stated in Solmate's `SafeTransferLib`, 
            the responsibility to ensure the presence of token code is delegated to the caller. 
            Hence, it's crucial to be aware of this subtlety when implementing and interacting with these 
            libraries to ensure the correct operation and security of your smart contracts."}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}
