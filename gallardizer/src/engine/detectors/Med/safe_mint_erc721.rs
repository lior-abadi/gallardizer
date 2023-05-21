use crate::engine::detectors::{get_match_with_regex, Detector};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use regex::Regex;
use solang_parser::pt::{IdentifierPath, SourceUnitPart};

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
            let pattern: Regex = Regex::new(r"_mint\([^,]+,\s*[^,]+\)").unwrap();
            let detected_issues_with_regex = get_match_with_regex(parsed_file, pattern);
            for detected_issue in detected_issues_with_regex {
                self.detected_issues.push(detected_issue);
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
