# Issues Summary
## Low Risk Issues
| |Issue|Instances|
|-|:-|:-:|
| [L-1] | Insecure declaration of <code>pragma</code> version | 2 |


Total: 2 instances over 1 issue

## Non-Critical Issues
| |Issue|Instances|
|-|:-|:-:|
| [NC-1] | The <code>nonReentrant</code> modifier should precede all other modifiers | 1 |
| [NC-2] | Prefer scientific notation over exponentiation | 9 |


Total: 10 instances over 2 issues

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
File: ./ajna-grants/src/token/AjnaToken.sol

4:    pragma solidity 0.8.7;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol)


```solidity
File: ./ajna-grants/src/token/BurnWrapper.sol

4:    pragma solidity 0.8.7;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/BurnWrapper.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/BurnWrapper.sol)



# Non-Critical Issues
## [NC-1] The <code>nonReentrant</code> modifier should precede all other modifiers
Prioritizing reentrancy checks before any other calculations or validations within modifiers 
is a recommended practice for enhancing the security of the protected function.

*This issue found 1 time:*

```solidity
File: ./ajna-core/src/PositionManager.sol

262:        function moveLiquidity(
263:            MoveLiquidityParams calldata params_
264:        ) external override mayInteract(params_.pool, params_.tokenId) nonReentrant {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol)



## [NC-2] Prefer scientific notation over exponentiation
Although the compiler effectively optimizes the use of exponentiation, 
it's generally more advisable to employ scientific notation for representing large numbers. 
By opting for idioms like <code>1e18</code> instead of <code>10**18</code>, you're using a method that
inherently does not require additional compiler optimization.<br>
 
This practice promotes clarity and efficiency in your code, aligning with robust coding standards.

*This issue was found 9 times:*

```solidity
File: ./ajna-core/src/libraries/helpers/PoolHelper.sol

102:                minDebtAmount_ = Maths.wdiv(Maths.wdiv(debt_, Maths.wad(loansCount_)), 10**19);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol)


```solidity
File: ./ajna-core/src/libraries/internal/Maths.sol

42:            return (x * y + 10**27 / 2) / 10**27;


46:            z = n % 2 != 0 ? x : 10**27;


58:            return (x + 10**9 / 2) / 10**9;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Maths.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Maths.sol)


```solidity
File: ./ajna-grants/src/grants/libraries/Maths.sol

6:        uint256 public constant WAD = 10**18;


30:            z = z * 10**9;


34:            return (x * y + 10**18 / 2) / 10**18;


38:            return (x * 10**18 + y / 2) / y;


47:            z = n % 2 != 0 ? x : 10**18;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol)



