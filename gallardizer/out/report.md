# Issues Summary
## Low Risk Issues
| |Issue|Instances|
|-|:-|:-:|
| [L-1] | Insecure declaration of <code>pragma</code> version | 2 |
| [L-2] | Use `require()` over `assert()` for error avoidance and effective gas management | 1 |


Total: 3 instances over 2 issues

## Overall Results
**Total: 3 instances over 2 issues, potentially saving over 0 gas units**

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



## [L-2] Use `require()` over `assert()` for error avoidance and effective gas management
Utilizing `assert` in versions of Solidity before `0.8.0` can exhaust a transaction's 
remaining gas rather than conserving it, as is the case with `require()` and `revert()`. 
Beyond `0.8.0`, `assert()` still merits caution as it introduces a `Panic` 
[error](https://docs.soliditylang.org/en/v0.8.14/control-structures.html#panic-via-assert-and-error-via-require) when triggered, which, 
under proper contract operation, should not happen even with incorrect external input. 
Therefore, the occurrence of such an error could be symptomatic of an underlying contract bug. 
Consequently, adopting `require()` over `assert()` for input and condition validation can promote better gas 
optimization and prevent `Panic` errors, reinforcing the reliability of your contract.

*This issue found 1 time:*

```solidity
File: ./contracts/HelloWorld.sol

19:            assert(b != 0);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/contracts/HelloWorld.sol)



