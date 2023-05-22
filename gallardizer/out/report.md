# Issues Summary
## Medium Risk Issues
| |Issue|Instances|
|-|:-|:-:|
| [M-1] | Single point of failure due to centralization risk | 1 |


Total: 1 instance over 1 issue

## Low Risk Issues
| |Issue|Instances|
|-|:-|:-:|
| [L-1] | Insecure declaration of <code>pragma</code> version | 2 |


Total: 2 instances over 1 issue

## Non-Critical Issues
| |Issue|Instances|
|-|:-|:-:|
| [NC-1] | The <code>nonReentrant</code> modifier should precede all other modifiers | 1 |


Total: 1 instance over 1 issue

## Gas Optimizations
| |Issue|Instances|Total Gas Saved|
|-|:-|:-:|:-:|
| [G-1] | Adopt custom errors over `revert()/require()` strings | 1 | 50 |


Total: 1 instance over 1 issue, saving over 50 gas units

## Overall Results
**Total: 5 instances over 4 issues, potentially saving over 50 gas units**

# Medium Risk Issues
## [M-1] Single point of failure due to centralization risk
The presence of a single account as the sole owner of a contract presents centralization risks and introduces a 
single point of failure. This setup leaves the contract vulnerable to loss, theft, or unavailability of the 
private key (if the owner is an external owned account). To mitigate these risks, consider transitioning to a 
multi-signature setup or adopting a role-based authorization model, segregating roles and their privileges as a 
part of a defense in depth strategy. This way, responsibilities and powers can be distributed, enhancing security, 
reducing centralization, and providing robust protection against potential adversarial events.

*This issue found 1 time:*

```solidity
File: ./contracts/HelloWorld.sol

18:        function test(uint256 a, uint256 b) external onlyOwner nonReentrant returns (uint256) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol)



# Low Risk Issues
## [L-1] Insecure declaration of <code>pragma</code> version
 The utilization of a flexible pragma version could introduce a variety of potential risks to your contract, 
accommodating a range of compiler versions which may lack support for specific improvements and changes such as 
those found in <code>0.8.8</code>'s <code>override</code> modifier or <code>0.8.11</code>'s <code>abi.encodeCall</code>.<br>

Without singling out these features as definitive concerns, it's important to acknowledge the broad 
spectrum of unexpected complications that could occur. A recommendation would be to align with a fixed, 
updated pragma version, providing a defense against potential compatibility issues that are tied to evolving 
language specifications and reducing exposure to bugs fixed in recent compiler versions, all of which contributes 
to a more stable project.

*This issue was found 2 times:*

```solidity
File: ./contracts/Test.sol

2:    pragma solidity ^0.8.0;


3:    pragma solidity ^0.8.1;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/Test.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/Test.sol)



# Non-Critical Issues
## [NC-1] The <code>nonReentrant</code> modifier should precede all other modifiers
Prioritizing reentrancy checks before any other calculations or validations within modifiers 
is a recommended practice for enhancing the security of the protected function.

*This issue found 1 time:*

```solidity
File: ./contracts/HelloWorld.sol

18:        function test(uint256 a, uint256 b) external onlyOwner nonReentrant returns (uint256) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol)



# Gas Optimizations
## [G-1] Adopt custom errors over `revert()/require()` strings
From Solidity version `0.8.4`, custom errors are available which can offer gas efficiency compared to 
`revert()` or `require()` revert strings. Utilizing custom errors saves each time they're triggered, 
as it bypasses the need to allocate and store the revert string. In addition, omitting the definition of these 
strings conserves deployment gas. Switching to custom errors can be a significant optimization, enhancing the 
performance and cost-effectiveness of your smart contract.

*This issue found 1 time:*

```solidity
File: ./contracts/HelloWorld.sol

19:            require(b != 0);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol)



