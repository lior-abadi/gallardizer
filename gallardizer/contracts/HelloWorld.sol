// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

contract HelloWorld {
    struct Proposal {
        uint256 number;
        bytes32 hash;
    }

    mapping(uint256 => Proposal) _standardFundingProposals;
    mapping(bytes32 => uint256[]) _fundedProposalSlates;

    function test3(uint256 f, uint256 g) external returns (uint256) {
        Proposal memory proposal = _standardFundingProposals[f];
        Proposal memory proposal = _standardFundingProposalsss[f];

        // uint256[] memory fundingProposalIds = _fundedProposalSlates[fundedSlateHash];

        // Proposal storage proposal2 = _standardFundingProposals[fundingProposalIds[i]];

        // uint256 number = 1;

        return f / g;
    }

    function test3(uint256 f, uint256 g) external view returns (uint256) {
        Proposal memory proposal = _standardFundingProposals[f];
        Proposal memory proposal = _standardFundingProposalsss[f];

        return f / g;
    }

    function test3(uint256 f, uint256 g) external pure returns (uint256) {
        Proposal memory proposal = _standardFundingProposals[f];
        Proposal memory proposal = _standardFundingProposalsss[f];

        return f / g;
    }

    // function _fundedProposalSlates(uint256 index) internal returns (uint256[] memory) {}
}
