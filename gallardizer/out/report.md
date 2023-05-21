# Issues Summary
## Medium Risk Issues
| |Issue|Instances|
|-|:-|:-:|
| [M-1] | Prioritize <code>_safeMint()</code> over <code>_mint()</code> for enhanced security when minting NFTs | 1 |


Total: 1 instance over 1 issue

## Low Risk Issues
| |Issue|Instances|
|-|:-|:-:|
| [L-1] | Potential precision loss from division with large numbers | 2 |
| [L-2] | Insecure declaration of <code>pragma</code> version | 2 |


Total: 4 instances over 2 issues

## Gas Optimizations
| |Issue|Instances|Total Gas Saved|
|-|:-|:-:|:-:|
| [G-1] | Adopt custom errors over `revert()/require()` strings | 2 | 100 |


Total: 2 instances over 1 issue

# Medium Risk Issues
## [M-1] Prioritize <code>_safeMint()</code> over <code>_mint()</code> for enhanced security when minting NFTs
It's recommended to prioritize the use of <code>_safeMint()</code> over <code>_mint()</code> to reduce risk of halting or reverting at early stages of a function call.
The implementation principle of <code>_safeMint()</code> ensures the recipient is an Externally Owned Account (EOA) or correctly implements the <code>IERC721Receiver</code>
interface.<br>

The main difference resides in the checks made after minting that ensure the reception of the token (e.g. Openzeppelin's <code>_checkOnERC721Received</code>).
Not adhering to this practice can lead to tokens being locked or owned by contracts that aren't equipped to handle them.

*This issue found 1 time:*

```solidity
File: ./contracts/HelloWorld.sol

23:            _mint(a, b);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol)



# Low Risk Issues
## [L-1] Potential precision loss from division with large numbers
Division operations with large denominators in Solidity may result in a return value of 
zero due to its lack of fractional number support. It's crucial to address this by ensuring 
the numerator is always greater than the denominator. A suggested safeguard is to set a required 
minimum value for the numerator, mitigating the risk of unexpected precision loss and improving the 
accuracy of your computations.

*This issue was found 2 times:*

```solidity
File: ./contracts/HelloWorld.sol

22:            uint256 num3 = number / 2e18;


28:            return num2 / 1e18;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol)



## [L-2] Insecure declaration of <code>pragma</code> version
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



# Gas Optimizations
## [G-1] Adopt custom errors over `revert()/require()` strings
From Solidity version `0.8.4`, custom errors are available which can offer gas efficiency compared to 
`revert()` or `require()` revert strings. Utilizing custom errors saves each time they're triggered, 
as it bypasses the need to allocate and store the revert string. In addition, omitting the definition of these 
strings conserves deployment gas. Switching to custom errors can be a significant optimization, enhancing the 
performance and cost-effectiveness of your smart contract.

*This issue was found 2 times:*

```solidity
File: ./contracts/HelloWorld.sol

25:            require(something, "error");


26:            revert("error2");

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol)



