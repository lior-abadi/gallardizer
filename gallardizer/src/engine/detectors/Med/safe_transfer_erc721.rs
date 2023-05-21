use crate::engine::detectors::{get_match_with_regex, Detector};
use crate::engine::report_generator::{IssueAppearance, IssueMetadata, Severities};
use crate::utils::file_processor::FileNameWithContent;
use indoc::indoc;

use regex::Regex;
use solang_parser::pt::SourceUnitPart;

pub struct SafeTransferERC721 {
    pub detected_issues: Vec<IssueAppearance>,
}

impl Detector for SafeTransferERC721 {
    fn run_detector(&mut self, parsed_file: &FileNameWithContent) {
        // We should check first if the contract inherits from an ERC20
        // Another case is still open: a contract that:
        // 1. Does not inherit from ERC20
        // 2. Uses both transferFrom of ERC20 and ERC721 (e.g. buy/sell NFTS with ERC20's)
        // However, it still will be flagged raising a false positive.
        let mut inherits_erc20: bool = false;
        for part in &parsed_file.parsed_ast_tree.0 {
            match part {
                SourceUnitPart::ContractDefinition(def) => {
                    for parent in &def.base {
                        if parent.name.identifiers[0].name.contains("ERC20") {
                            inherits_erc20 = true;
                        }
                    }
                }
                _ => (),
            }
        }
        if (!inherits_erc20) {
            let pattern: Regex = Regex::new(r"transferFrom\([^,]+,\s*[^,]+,\s*[^,]+\)").unwrap();
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
            title: indoc! {"Risk of NFT loss with `transferFrom()`, use `safeTransferFrom()` instead"}.to_string(),
            content: indoc! {
            "The use of `transferFrom()` in transferring NFTs, as outlined in the `EIP-721` [standard](https://github.com/ethereum/EIPs/blob/78e2c297611f5e92b6a5112819ab71f74041ff25/EIPS/eip-721.md?plain=1#L103-L113), 
            places the responsibility on the caller to ensure that the recipient `_to` is capable of 
            receiving NFTs. Failure to ensure this could lead to permanent loss of the NFTs.

            By contrast, `safeTransferFrom()` mitigates these risks by performing additional checks to ensure 
            the recipient can handle the token transfer. It's highly advised to use `safeTransferFrom()` over 
            `transferFrom()` to avoid the risk of permanent NFT loss."}.to_string(),
            gas_saved_per_instance: 0,
        };

        return metadata;
    }
}
