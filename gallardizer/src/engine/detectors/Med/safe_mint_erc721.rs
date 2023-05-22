use crate::engine::detectors::{get_appearance_metadata, Detector};
use crate::engine::parser::{extract_target_from_node, Target};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use solang_parser::pt::{Expression, SourceUnitPart};

pub struct SafeMintERC721 {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for SafeMintERC721 {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        // We will check first if the contract inherits from an ERC721
        let mut inherits_erc721: bool = false;
        for part in &parsed_file.parsed_ast_tree.0 {
            match part {
                SourceUnitPart::ContractDefinition(def) => {
                    for parent in &def.base {
                        if parent.name.identifiers[0].name.contains("ERC721") {
                            inherits_erc721 = true;
                        }
                    }
                }
                _ => (),
            }
        }
        if inherits_erc721 {
            let target_nodes = extract_target_from_node(
                Target::FunctionCall,
                parsed_file.parsed_ast_tree.clone().into(),
            );

            for node in target_nodes {
                // FunctionCall is an expression
                let expression = node.expression().unwrap();

                if let Expression::FunctionCall(loc, selector, params) = expression {
                    if let Expression::Variable(identifier) = *selector {
                        if identifier.name == "_mint" {
                            self.detected_issues
                                .push(get_appearance_metadata(&loc, parsed_file));
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
        return "SafeMintERC721".to_string();
    }

    fn get_metadata(&self) -> IssueMetadata {
        let metadata: IssueMetadata = IssueMetadata {
            severity: Severities::M,
            title: indoc! {"Prioritize <code>_safeMint()</code> over <code>_mint()</code> for enhanced security when minting NFTs"}.to_string(),
            content: indoc! {
            "It's recommended to prioritize the use of <code>_safeMint()</code> over <code>_mint()</code> to reduce risk of halting or reverting at early stages of a function call.
             The implementation principle of <code>_safeMint()</code> ensures the recipient is an Externally Owned Account (EOA) or correctly implements the <code>IERC721Receiver</code>
             interface.<br>

             The main difference resides in the checks made after minting that ensure the reception of the token (e.g. Openzeppelin's <code>_checkOnERC721Received</code>).
             Not adhering to this practice can lead to tokens being locked or owned by contracts that aren't equipped to handle them."}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}
