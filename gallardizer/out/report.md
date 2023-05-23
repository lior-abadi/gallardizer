# Issues Summary
## Medium Risk Issues
| |Issue|Instances|
|-|:-|:-:|
| [M-1] | Absence of token contract existence check in Solmate's `SafeTransferLib` | 1 |


Total: 1 instance over 1 issue

## Low Risk Issues
| |Issue|Instances|
|-|:-|:-:|
| [L-1] | Insecure declaration of <code>pragma</code> version | 2 |


Total: 2 instances over 1 issue

## Non-Critical Issues
| |Issue|Instances|
|-|:-|:-:|
| [NC-1] | Add descriptive revert reasons | 2 |


Total: 2 instances over 1 issue

## Gas Optimizations
| |Issue|Instances|Total Gas Saved|
|-|:-|:-:|:-:|
| [G-1] | Adopt custom errors over `revert()/require()` strings | 3 | 150 |


Total: 3 instances over 1 issue, saving over 150 gas units

## Overall Results
**Total: 8 instances over 4 issues, potentially saving over 150 gas units**

# Medium Risk Issues
## [M-1] Absence of token contract existence check in Solmate's `SafeTransferLib`
Safe-wrapped methods could be called to non existent token contracts without reverting.
A key distinction exists between Solmate's `SafeTransferLib` and OpenZeppelin's
`SafeERC20`. While `SafeERC20` validates whether the token is a contract, 
SafeTransferLib does not perform this check. As stated in Solmate's `SafeTransferLib`, 
the responsibility to ensure the presence of token code is delegated to the caller. 
Hence, it's crucial to be aware of this subtlety when implementing and interacting with these 
libraries to ensure the correct operation and security of your smart contracts.

*This issue was found 1 time:*

```solidity
File: ./contracts/HelloWorld.sol

3:    import "solmate/utils/SafeTransferLib.sol";

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
## [NC-1] Add descriptive revert reasons
Include descriptive reason strings in `require()` and `revert()` for 
improved error handling and user feedback. Since Solidity `0.8.4`, 
[custom errors](https://blog.soliditylang.org/2021/04/21/custom-errors/) offer a concise, 
detailed alternative for reversion, facilitating better contract usability and debugging 
also providing a more efficient way of reverting.

*This issue was found 2 times:*

```solidity
File: ./contracts/HelloWorld.sol

21:            revert();


24:            require(a != b);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol)



# Gas Optimizations
## [G-1] Adopt custom errors over `revert()/require()` strings
From Solidity version `0.8.4`, custom errors are available which can offer gas efficiency compared to 
`revert()` or `require()` revert strings. Utilizing custom errors saves each time they're triggered, 
as it bypasses the need to allocate and store the revert string. In addition, omitting the definition of these 
strings conserves deployment gas. Switching to custom errors can be a significant optimization, enhancing the 
performance and cost-effectiveness of your smart contract.

*This issue was found 3 times:*

```solidity
File: ./contracts/HelloWorld.sol

22:            revert("Someting");


23:            require(a != b, "perro");


24:            require(a != b);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol)



