# Issues Summary
## Medium Risk Issues
| |Issue|Instances|
|-|:-|:-:|
| [M-1] | Prioritize <code>_safeMint()</code> over <code>_mint()</code> for enhanced security when minting NFTs | 1 |
| [M-2] | Risk of NFT loss with `transferFrom()`, use `safeTransferFrom()` instead | 3 |


Total: 4 instances over 2 issues

## Low Risk Issues
| |Issue|Instances|
|-|:-|:-:|
| [L-1] | Potential precision loss from division with large numbers | 8 |
| [L-2] | Insecure declaration of <code>pragma</code> version | 2 |
| [L-3] | Denial of service risk from unbounded for-loops with external calls | 5 |


Total: 15 instances over 3 issues

## Non-Critical Issues
| |Issue|Instances|
|-|:-|:-:|
| [NC-1] | The <code>nonReentrant</code> modifier should precede all other modifiers | 1 |
| [NC-2] | Prefer scientific notation over exponentiation | 9 |


Total: 10 instances over 2 issues

## Gas Optimizations
| |Issue|Instances|Total Gas Saved|
|-|:-|:-:|:-:|
| [G-1] | Adopt custom errors over `revert()/require()` strings | 7 | 350 |


Total: 7 instances over 1 issue, saving over 350 gas units

## Overall Results
**Total: 36 instances over 8 issues, potentially saving over 350 gas units**

# Medium Risk Issues
## [M-1] Prioritize <code>_safeMint()</code> over <code>_mint()</code> for enhanced security when minting NFTs
It's recommended to prioritize the use of <code>_safeMint()</code> over <code>_mint()</code> to reduce risk of halting or reverting at early stages of a function call.
The implementation principle of <code>_safeMint()</code> ensures the recipient is an Externally Owned Account (EOA) or correctly implements the <code>IERC721Receiver</code>
interface.<br>

The main difference resides in the checks made after minting that ensure the reception of the token (e.g. Openzeppelin's <code>_checkOnERC721Received</code>).
Not adhering to this practice can lead to tokens being locked or owned by contracts that aren't equipped to handle them.

*This issue found 1 time:*

```solidity
File: ./ajna-core/src/PositionManager.sol

238:            _mint(params_.recipient, tokenId_);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol)



## [M-2] Risk of NFT loss with `transferFrom()`, use `safeTransferFrom()` instead
The use of `transferFrom()` in transferring NFTs, as outlined in the `EIP-721` [standard](https://github.com/ethereum/EIPs/blob/78e2c297611f5e92b6a5112819ab71f74041ff25/EIPS/eip-721.md?plain=1#L103-L113), 
places the responsibility on the caller to ensure that the recipient `_to` is capable of 
receiving NFTs. Failure to ensure this could lead to permanent loss of the NFTs.

By contrast, `safeTransferFrom()` mitigates these risks by performing additional checks to ensure 
the recipient can handle the token transfer. It's highly advised to use `safeTransferFrom()` over 
`transferFrom()` to avoid the risk of permanent NFT loss.

*This issue was found 3 times:*

```solidity
File: ./ajna-core/src/ERC721Pool.sol

612:            IERC721Token(_getArgAddress(COLLATERAL_ADDRESS)).transferFrom(from_, to_, tokenId_);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol)


```solidity
File: ./ajna-core/src/RewardsManager.sol

250:            IERC721(address(positionManager)).transferFrom(msg.sender, address(this), tokenId_);


302:            IERC721(address(positionManager)).transferFrom(address(this), msg.sender, tokenId_);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/RewardsManager.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/RewardsManager.sol)



# Low Risk Issues
## [L-1] Potential precision loss from division with large numbers
Division operations with large denominators in Solidity may result in a return value of 
zero due to its lack of fractional number support. It's crucial to address this by ensuring 
the numerator is always greater than the denominator. A suggested safeguard is to set a required 
minimum value for the numerator, mitigating the risk of unexpected precision loss and improving the 
accuracy of your computations.

*This issue was found 8 times:*

```solidity
File: ./ajna-core/src/ERC721Pool.sol

457:                result.collateralAmount / 1e18


537:            uint256 borrowerCollateralRoundedUp = (borrowerCollateral_ + 1e18 - 1) / 1e18;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol)


```solidity
File: ./ajna-core/src/libraries/external/PoolCommons.sol

281:                mau102 = mau * PERCENT_102 / 1e18;


294:            } else if (4 * (tu - mau) > 1e18 - ((tu + mau - 1e18) ** 2) / 1e18) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol)


```solidity
File: ./ajna-core/src/libraries/external/TakerActions.sol

356:                takeableCollateral = (takeableCollateral / 1e18) * 1e18;


381:                uint256 collateralTaken = (vars_.collateralAmount / 1e18) * 1e18; // solidity rounds down, so if 2.5 it will be 2.5 / 1 = 2

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol)


```solidity
File: ./ajna-core/src/libraries/helpers/PoolHelper.sol

181:                pricePrecisionAdjustment_ = uint256(result / 1e18);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol)


```solidity
File: ./ajna-core/src/libraries/internal/Loans.sol

114:                    borrower_.t0Np = (1e18 + poolRate_) * curMomp * t0ThresholdPrice / lup_ / 1e18;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Loans.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Loans.sol)



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
File: ./ajna-grants/src/token/AjnaToken.sol

4:    pragma solidity 0.8.7;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol)


```solidity
File: ./ajna-grants/src/token/BurnWrapper.sol

4:    pragma solidity 0.8.7;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/BurnWrapper.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/BurnWrapper.sol)



## [L-3] Denial of service risk from unbounded for-loops with external calls
Unbounded for-loops making external calls pose a Denial of Service (DOS) risk due to potential gas limitations. 
This can disrupt contract operation and even lead to a halt in functionalities. To enhance contract stability and 
resilience against DOS attacks, consider limiting the number of iterations in these loops, thereby controlling gas 
consumption and ensuring smoother execution.

*This issue was found 5 times:*

```solidity
File: ./ajna-core/src/ERC721Pool.sol

561:            for (uint256 i = 0; i < tokenIds_.length;) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol)


```solidity
File: ./ajna-core/src/RewardsManager.sol

229:            for (uint256 i = 0; i < positionIndexes.length; ) {


229:            for (uint256 i = 0; i < positionIndexes.length; ) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/RewardsManager.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/RewardsManager.sol)


```solidity
File: ./ajna-grants/src/grants/base/Funding.sol

62:            for (uint256 i = 0; i < targets_.length; ++i) {


112:            for (uint256 i = 0; i < targets_.length;) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol)



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



# Gas Optimizations
## [G-1] Adopt custom errors over `revert()/require()` strings
From Solidity version `0.8.4`, custom errors are available which can offer gas efficiency compared to 
`revert()` or `require()` revert strings. Utilizing custom errors saves each time they're triggered, 
as it bypasses the need to allocate and store the revert string. In addition, omitting the definition of these 
strings conserves deployment gas. Switching to custom errors can be a significant optimization, enhancing the 
performance and cost-effectiveness of your smart contract.

*This issue was found 7 times:*

```solidity
File: ./ajna-core/src/PositionManager.sol

520:            require(_exists(tokenId_));

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol)


```solidity
File: ./ajna-core/src/base/PermitERC721.sol

80:            require(block.timestamp <= deadline_, "ajna/nft-permit-expired");


98:            require(spender_ != owner, "ERC721Permit: approval to current owner");


108:                require(recoveredAddress != address(0), "ajna/nft-invalid-signature");


109:                require(recoveredAddress == owner, "ajna/nft-unauthorized");

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/PermitERC721.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/PermitERC721.sol)


```solidity
File: ./ajna-core/src/libraries/helpers/SafeTokenNamer.sol

76:            require(len % 2 == 0 && len > 0 && len <= 40, 'SafeERC20Namer: INVALID_LEN');

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/SafeTokenNamer.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/SafeTokenNamer.sol)


```solidity
File: ./ajna-grants/src/token/AjnaToken.sol

28:            require(to_ != address(this), "Cannot transfer tokens to the contract itself");

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol)



