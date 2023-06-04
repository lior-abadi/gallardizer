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
| [L-4] | Reversals due to division by zero | 5 |
| [L-5] | Absence of array length validation | 3 |


Total: 23 instances over 5 issues

## Non-Critical Issues
| |Issue|Instances|
|-|:-|:-:|
| [NC-1] | The <code>nonReentrant</code> modifier should precede all other modifiers | 1 |
| [NC-2] | Prefer scientific notation over exponentiation | 9 |
| [NC-3] | Add descriptive revert reasons | 1 |
| [NC-4] | Avoid using magic numbers | 61 |
| [NC-5] | Time-related numeric values could employ time units | 3 |
| [NC-6] | Expressions defining constant values should employ `immutable` instead of `constant` | 4 |
| [NC-7] | Long lines of code | 91 |
| [NC-8] | Inadequate indexing of event fields | 15 |


Total: 185 instances over 8 issues

## Gas Optimizations
| |Issue|Instances|Total Gas Saved|
|-|:-|:-:|:-:|
| [G-1] | Adopt custom errors over `revert()/require()` strings | 7 | 350 |
| [G-2] | Prefer `storage` over `memory` for structs/arrays | 5 | 21000 |
| [G-3] | Use bit shifting for division by two | 12 | 240 |


Total: 24 instances over 3 issues, saving over 51230 gas units

## Overall Results
**Total: 236 instances over 18 issues, potentially saving over 51230 gas units**

# Medium Risk Issues
## [M-1] Prioritize <code>_safeMint()</code> over <code>_mint()</code> for enhanced security when minting NFTs
It's recommended to prioritize the use of <code>_safeMint()</code> over <code>_mint()</code> to reduce risk of halting or reverting at early stages of a function call.
The implementation principle of <code>_safeMint()</code> ensures the recipient is an Externally Owned Account (EOA) or correctly implements the <code>IERC721Receiver</code>
interface.<br>

The main difference resides in the checks made after minting that ensure the reception of the token (e.g. Openzeppelin's <code>_checkOnERC721Received</code>).
Not adhering to this practice can lead to tokens being locked or owned by contracts that aren't equipped to handle them.

<details>

*<summary><i>This issue was found 1 time:</i></summary>*

```solidity
File: ./ajna-core/src/PositionManager.sol

238:            _mint(params_.recipient, tokenId_);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol#L238](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol#L238)



</details>

## [M-2] Risk of NFT loss with `transferFrom()`, use `safeTransferFrom()` instead
The use of `transferFrom()` in transferring NFTs, as outlined in the `EIP-721` [standard](https://github.com/ethereum/EIPs/blob/78e2c297611f5e92b6a5112819ab71f74041ff25/EIPS/eip-721.md?plain=1#L103-L113), 
places the responsibility on the caller to ensure that the recipient `_to` is capable of 
receiving NFTs. Failure to ensure this could lead to permanent loss of the NFTs.

By contrast, `safeTransferFrom()` mitigates these risks by performing additional checks to ensure 
the recipient can handle the token transfer. It's highly advised to use `safeTransferFrom()` over 
`transferFrom()` to avoid the risk of permanent NFT loss.

<details>

*<summary><i>This issue was found 3 times:</i></summary>*

```solidity
File: ./ajna-core/src/ERC721Pool.sol

612:            IERC721Token(_getArgAddress(COLLATERAL_ADDRESS)).transferFrom(from_, to_, tokenId_);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol#L612](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol#L612)


```solidity
File: ./ajna-core/src/RewardsManager.sol

250:            IERC721(address(positionManager)).transferFrom(msg.sender, address(this), tokenId_);


302:            IERC721(address(positionManager)).transferFrom(address(this), msg.sender, tokenId_);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/RewardsManager.sol#L250](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/RewardsManager.sol#L250)



</details>

# Low Risk Issues
## [L-1] Potential precision loss from division with large numbers
Division operations with large denominators in Solidity may result in a return value of 
zero due to its lack of fractional number support. It's crucial to address this by ensuring 
the numerator is always greater than the denominator. A suggested safeguard is to set a required 
minimum value for the numerator, mitigating the risk of unexpected precision loss and improving the 
accuracy of your computations.

<details>

*<summary><i>This issue was found 8 times:</i></summary>*

```solidity
File: ./ajna-core/src/ERC721Pool.sol

457:                result.collateralAmount / 1e18


537:            uint256 borrowerCollateralRoundedUp = (borrowerCollateral_ + 1e18 - 1) / 1e18;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol#L457](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol#L457)


```solidity
File: ./ajna-core/src/libraries/external/PoolCommons.sol

281:                mau102 = mau * PERCENT_102 / 1e18;


294:            } else if (4 * (tu - mau) > 1e18 - ((tu + mau - 1e18) ** 2) / 1e18) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol#L281](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol#L281)


```solidity
File: ./ajna-core/src/libraries/external/TakerActions.sol

356:                takeableCollateral = (takeableCollateral / 1e18) * 1e18;


381:                uint256 collateralTaken = (vars_.collateralAmount / 1e18) * 1e18; // solidity rounds down, so if 2.5 it will be 2.5 / 1 = 2

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol#L356](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol#L356)


```solidity
File: ./ajna-core/src/libraries/helpers/PoolHelper.sol

181:                pricePrecisionAdjustment_ = uint256(result / 1e18);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol#L181](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol#L181)


```solidity
File: ./ajna-core/src/libraries/internal/Loans.sol

114:                    borrower_.t0Np = (1e18 + poolRate_) * curMomp * t0ThresholdPrice / lup_ / 1e18;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Loans.sol#L114](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Loans.sol#L114)



</details>

## [L-2] Insecure declaration of <code>pragma</code> version
 The utilization of a flexible pragma version could introduce a variety of potential risks to your contract, 
accommodating a range of compiler versions which may lack support for specific improvements and changes such as 
<code>0.8.11</code>'s <code>abi.encodeCall</code>.<br>

Without singling out these features as definitive concerns, it's important to acknowledge the broad 
spectrum of unexpected complications that could occur. A recommendation would be to align with a fixed, 
updated pragma version, providing a defense against potential compatibility issues that are tied to evolving 
language specifications and reducing exposure to bugs fixed in recent compiler versions, all of which contributes 
to a more stable project.

<details>

*<summary><i>This issue was found 2 times:</i></summary>*

```solidity
File: ./ajna-grants/src/token/AjnaToken.sol

4:    pragma solidity 0.8.7;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol#L4](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol#L4)


```solidity
File: ./ajna-grants/src/token/BurnWrapper.sol

4:    pragma solidity 0.8.7;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/BurnWrapper.sol#L4](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/BurnWrapper.sol#L4)



</details>

## [L-3] Denial of service risk from unbounded for-loops with external calls
Unbounded for-loops making external calls pose a Denial of Service (DOS) risk due to potential gas limitations. 
This can disrupt contract operation and even lead to a halt in functionalities. To enhance contract stability and 
resilience against DOS attacks, consider limiting the number of iterations in these loops, thereby controlling gas 
consumption and ensuring smoother execution.

<details>

*<summary><i>This issue was found 5 times:</i></summary>*

```solidity
File: ./ajna-core/src/ERC721Pool.sol

561:            for (uint256 i = 0; i < tokenIds_.length;) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol#L561](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol#L561)


```solidity
File: ./ajna-core/src/RewardsManager.sol

229:            for (uint256 i = 0; i < positionIndexes.length; ) {


229:            for (uint256 i = 0; i < positionIndexes.length; ) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/RewardsManager.sol#L229](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/RewardsManager.sol#L229)


```solidity
File: ./ajna-grants/src/grants/base/Funding.sol

62:            for (uint256 i = 0; i < targets_.length; ++i) {


112:            for (uint256 i = 0; i < targets_.length;) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol#L62](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol#L62)



</details>

## [L-4] Reversals due to division by zero
A division operation lacks necessary zero-value checks on any parameter used as denominator, 
which could result in the function reverting if zero is passed as an argument. It's crucial to implement 
safeguards against such division by zero errors to prevent unexpected function reverts and maintain the 
integrity of each contract's calculations.

<details>

*<summary><i>This issue was found 5 times:</i></summary>*

```solidity
File: ./ajna-core/src/libraries/helpers/PoolHelper.sol

248:            scaledAmount_ = (amount_ / tokenScale_) * tokenScale_;


389:            uint256 thresholdPrice = borrowerDebt_  * Maths.WAD / collateral_;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol#L248](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol#L248)


```solidity
File: ./ajna-core/src/libraries/internal/Maths.sol

22:            return (x * WAD + y / 2) / y;


26:            return (x * WAD) / y;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Maths.sol#L22](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Maths.sol#L22)


```solidity
File: ./ajna-grants/src/grants/libraries/Maths.sol

38:            return (x * 10**18 + y / 2) / y;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol#L38](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol#L38)



</details>

## [L-5] Absence of array length validation
Without explicit checks for arrays to have the same length, user operations 
might not be completely executed. This is due to the disparity between the number of items 
involved in the iteration and the number of items in the subsequent arrays.

<details>

*<summary><i>This issue was found 3 times:</i></summary>*

```solidity
File: ./ajna-core/src/libraries/external/LPActions.sol

55:        function increaseLPAllowance(
56:            mapping(uint256 => uint256) storage allowances_,
57:            address spender_,
58:            uint256[] calldata indexes_,
59:            uint256[] calldata amounts_
60:        ) external {


91:        function decreaseLPAllowance(
92:            mapping(uint256 => uint256) storage allowances_,
93:            address spender_,
94:            uint256[] calldata indexes_,
95:            uint256[] calldata amounts_
96:        ) external {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LPActions.sol#L55-L60](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LPActions.sol#L55-L60)


```solidity
File: ./ajna-grants/src/grants/base/Funding.sol

52:        function _execute(
53:            uint256 proposalId_,
54:            address[] memory targets_,
55:            uint256[] memory values_,
56:            bytes[] memory calldatas_
57:        ) internal {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol#L52-L57](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol#L52-L57)



</details>

# Non-Critical Issues
## [NC-1] The <code>nonReentrant</code> modifier should precede all other modifiers
Prioritizing reentrancy checks before any other calculations or validations within modifiers 
is a recommended practice for enhancing the security of the protected function.

<details>

*<summary><i>This issue was found 1 time:</i></summary>*

```solidity
File: ./ajna-core/src/PositionManager.sol

262:        function moveLiquidity(
263:            MoveLiquidityParams calldata params_
264:        ) external override mayInteract(params_.pool, params_.tokenId) nonReentrant {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol#L262-L264](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol#L262-L264)



</details>

## [NC-2] Prefer scientific notation over exponentiation
Although the compiler effectively optimizes the use of exponentiation, 
it's generally more advisable to employ scientific notation for representing large numbers. 
By opting for idioms like <code>1e18</code> instead of <code>10**18</code>, you're using a method that
inherently does not require additional compiler optimization.<br>
 
This practice promotes clarity and efficiency in your code, aligning with robust coding standards.

<details>

*<summary><i>This issue was found 9 times:</i></summary>*

```solidity
File: ./ajna-core/src/libraries/helpers/PoolHelper.sol

102:                minDebtAmount_ = Maths.wdiv(Maths.wdiv(debt_, Maths.wad(loansCount_)), 10**19);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol#L102](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol#L102)


```solidity
File: ./ajna-core/src/libraries/internal/Maths.sol

42:            return (x * y + 10**27 / 2) / 10**27;


46:            z = n % 2 != 0 ? x : 10**27;


58:            return (x + 10**9 / 2) / 10**9;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Maths.sol#L42](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Maths.sol#L42)


```solidity
File: ./ajna-grants/src/grants/libraries/Maths.sol

6:        uint256 public constant WAD = 10**18;


30:            z = z * 10**9;


34:            return (x * y + 10**18 / 2) / 10**18;


38:            return (x * 10**18 + y / 2) / y;


47:            z = n % 2 != 0 ? x : 10**18;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol#L6](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol#L6)



</details>

## [NC-3] Add descriptive revert reasons
Include descriptive reason strings in `require()` and `revert()` for 
improved error handling and user feedback. Since Solidity `0.8.4`, 
[custom errors](https://blog.soliditylang.org/2021/04/21/custom-errors/) offer a concise, 
detailed alternative for reversion, facilitating better contract usability and debugging 
also providing a more efficient way of reverting.

<details>

*<summary><i>This issue was found 1 time:</i></summary>*

```solidity
File: ./ajna-core/src/PositionManager.sol

520:            require(_exists(tokenId_));

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol#L520](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol#L520)



</details>

## [NC-4] Avoid using magic numbers
It is recommended to define constants instead of relying on hex or numeric literals.
This practice enhances readability and clarity, even in assembly context, 
thereby mitigating the potential for confusion or error.

<details>

*<summary><i>This issue was found 61 times:</i></summary>*

```solidity
File: ./ajna-core/src/ERC721PoolFactory.sol

62:            try IERC165(collateral_).supportsInterface(0x80ac58cd) returns (bool supportsERC721Interface) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721PoolFactory.sol#L62](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721PoolFactory.sol#L62)


```solidity
File: ./ajna-core/src/PoolInfoUtils.sol

254:            timeRemaining_              = 3 days - Maths.min(3 days, block.timestamp - auctionKickTime);


254:            timeRemaining_              = 3 days - Maths.min(3 days, block.timestamp - auctionKickTime);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PoolInfoUtils.sol#L254](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PoolInfoUtils.sol#L254)


```solidity
File: ./ajna-core/src/base/PermitERC721.sol

59:                        0x8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f,


103:                    IERC1271(owner).isValidSignature(digest, abi.encodePacked(r_, s_, v_)) == 0x1626ba7e,

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/PermitERC721.sol#L59](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/PermitERC721.sol#L59)


```solidity
File: ./ajna-core/src/libraries/external/KickerActions.sol

277:            if (block.timestamp < lastBurnTimestamp + 2 weeks || block.timestamp - reserveAuction_.kicked <= 72 hours) {


407:            vars.t0KickPenalty = Maths.wdiv(Maths.wmul(kickResult_.t0KickedDebt, poolState_.rate), 4 * 1e18);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol#L277](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol#L277)


```solidity
File: ./ajna-core/src/libraries/external/PoolCommons.sol

180:            else if (block.timestamp - interestParams_.interestRateUpdate > 12 hours) {


227:            uint256 pendingFactor = PRBMathUD60x18.exp((poolState_.rate * elapsed_) / 365 days);


291:            if (4 * (tu - mau102) < (((tu + mau102 - 1e18) / 1e9) ** 2) - 1e18) {


294:            } else if (4 * (tu - mau) > 1e18 - ((tu + mau - 1e18) ** 2) / 1e18) {


299:            newInterestRate_ = Maths.min(500 * 1e18, Maths.max(0.001 * 1e18, newInterestRate_));


331:            uint256 base = 1_000_000 * 1e18 - Maths.wmul(Maths.min(mau_, 1e18), 1_000_000 * 1e18);


331:            uint256 base = 1_000_000 * 1e18 - Maths.wmul(Maths.min(mau_, 1e18), 1_000_000 * 1e18);


384:            return PRBMathUD60x18.exp((interestRate_ * elapsed_) / 365 days);


401:                PRBMathUD60x18.exp((interestRate_ * (block.timestamp - inflatorUpdate)) / 365 days)

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol#L180](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol#L180)


```solidity
File: ./ajna-core/src/libraries/external/PositionNFTSVG.sol

40:            string memory ownerHexString = (uint256(uint160(params_.owner))).toHexString(20);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PositionNFTSVG.sol#L40](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PositionNFTSVG.sol#L40)


```solidity
File: ./ajna-core/src/libraries/external/SettlerActions.sol

112:            if ((block.timestamp - kickTime < 72 hours) && (borrower.collateral != 0)) revert AuctionNotClearable();

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/SettlerActions.sol#L112](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/SettlerActions.sol#L112)


```solidity
File: ./ajna-core/src/libraries/external/TakerActions.sol

288:            if (kicked != 0 && block.timestamp - kicked <= 72 hours) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol#L288](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol#L288)


```solidity
File: ./ajna-core/src/libraries/helpers/PoolHelper.sol

82:                return uint256(4157 - PRBMathSD59x18.toInt(ceilIndex));


84:            return uint256(4156 - PRBMathSD59x18.toInt(ceilIndex));


116:            return Maths.max(Maths.wdiv(interestRate_, 52 * 1e18), 0.0005 * 1e18);


128:            return Maths.wdiv(interestRate_, 365 * 1e18);


178:            if (bucketIndex_ > 3900) {


179:                int256 bucketOffset = int256(bucketIndex_ - 3900);


180:                int256 result = PRBMathSD59x18.sqrt(PRBMathSD59x18.div(bucketOffset * 1e18, int256(36 * 1e18)));


304:                uint256 hoursComponent   = 1e27 >> secondsElapsed / 3600;


305:                uint256 minutesComponent = Maths.rpow(MINUTE_HALF_LIFE, secondsElapsed % 3600 / 60);


305:                uint256 minutesComponent = Maths.rpow(MINUTE_HALF_LIFE, secondsElapsed % 3600 / 60);


307:                price_ = Maths.rayToWad(1_000_000_000 * Maths.rmul(hoursComponent, minutesComponent));


334:            price_ = 32 * Maths.wmul(referencePrice, uint256(PRBMathSD59x18.exp2(timeAdjustment)));

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol#L82](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol#L82)


```solidity
File: ./ajna-core/src/libraries/helpers/RevertsHelper.sol

57:                if (block.timestamp - kickTime > 72 hours) revert AuctionNotCleared();


107:                if (loansCount >= 10)

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/RevertsHelper.sol#L57](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/RevertsHelper.sol#L57)


```solidity
File: ./ajna-core/src/libraries/helpers/SafeTokenNamer.sol

15:            symbol_ = _callAndParseStringReturn(token, 0x95d89b41);


18:                return _toAsciiString(token, 6);


25:            name_ = _callAndParseStringReturn(token, 0x06fdde03);


28:                return _toAsciiString(token, 40);


44:            if (data.length == 32) {


47:            } else if (data.length > 64) {


58:            bytes memory bytesString = new bytes(32);


60:            for (uint256 j = 0; j < 32; j++) {


76:            require(len % 2 == 0 && len > 0 && len <= 40, 'SafeERC20Namer: INVALID_LEN');


82:                uint8 b = uint8(addrNum >> (8 * (19 - i)));


82:                uint8 b = uint8(addrNum >> (8 * (19 - i)));


84:                uint8 hi = b >> 4;


86:                uint8 lo = b - (hi << 4);


97:            if (b < 10) {


98:                return bytes1(b + 0x30);


100:                return bytes1(b + 0x37);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/SafeTokenNamer.sol#L15](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/SafeTokenNamer.sol#L15)


```solidity
File: ./ajna-core/src/libraries/internal/Deposits.sol

82:            uint256 i  = 4096; // 1 << (_numBits - 1) = 1 << (13 - 1) = 4096


148:                lsb_ = i_ & ((i_ ^ 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff) + 1);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Deposits.sol#L82](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Deposits.sol#L82)


```solidity
File: ./ajna-grants/src/grants/base/Funding.sol

125:                if (selector != bytes4(0xa9059cbb)) revert InvalidProposal();

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol#L125](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol#L125)


```solidity
File: ./ajna-grants/src/grants/base/StandardFunding.sol

292:            ) / 10;


391:            if (newProposal.tokensRequested > (currentDistribution.fundsAvailable * 9 / 10)) revert InvalidProposal();


391:            if (newProposal.tokensRequested > (currentDistribution.fundsAvailable * 9 / 10)) revert InvalidProposal();


448:                if (totalTokensRequested > (gbc * 9 / 10)) {


448:                if (totalTokensRequested > (gbc * 9 / 10)) {


719:            if (screenedProposalsLength < 10 && indexInArray == -1) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol#L292](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol#L292)


```solidity
File: ./ajna-grants/src/grants/libraries/Maths.sol

19:            if (y > 3) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol#L19](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol#L19)


```solidity
File: ./ajna-grants/src/token/AjnaToken.sol

13:            _mint(tokenReceiver_, 2_000_000_000 * 10 ** decimals());

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol#L13](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol#L13)


```solidity
File: ./ajna-grants/src/token/BurnWrapper.sol

50:            return 18;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/BurnWrapper.sol#L50](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/BurnWrapper.sol#L50)



</details>

## [NC-5] Time-related numeric values could employ time units
For readability and consistency, numeric values associated with time should
utilize predefined [units](https://docs.soliditylang.org/en/latest/units-and-global-variables.html#time-units) 
like seconds, minutes, hours, days, or weeks.

<details>

*<summary><i>This issue was found 3 times:</i></summary>*

```solidity
File: ./ajna-grants/src/grants/base/StandardFunding.sol

34:        uint256 internal constant CHALLENGE_PERIOD_LENGTH = 50400;


40:        uint48 internal constant DISTRIBUTION_PERIOD_LENGTH = 648000;


46:        uint256 internal constant FUNDING_PERIOD_LENGTH = 72000;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol#L34](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol#L34)



</details>

## [NC-6] Expressions defining constant values should employ `immutable` instead of `constant`
It's important to distinguish between `constant` and `immutable` variables, 
using each in their appropriate situations. Constants are suitable for literal values 
hard-coded into the contracts, while `immutables` should be used for expression-based values, such as a call to `keccak256()`, 
or those calculated/introduced in the `constructor`.

<details>

*<summary><i>This issue was found 4 times:</i></summary>*

```solidity
File: ./ajna-core/src/ERC20PoolFactory.sol

28:        bytes32 public constant ERC20_NON_SUBSET_HASH = keccak256("ERC20_NON_SUBSET_HASH");

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC20PoolFactory.sol#L28](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC20PoolFactory.sol#L28)


```solidity
File: ./ajna-core/src/ERC721PoolFactory.sol

30:        bytes32 public constant ERC721_NON_SUBSET_HASH = keccak256("ERC721_NON_SUBSET_HASH");

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721PoolFactory.sol#L30](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721PoolFactory.sol#L30)


```solidity
File: ./ajna-grants/src/grants/base/ExtraordinaryFunding.sol

28:        bytes32 internal constant DESCRIPTION_PREFIX_HASH_EXTRAORDINARY = keccak256(bytes("Extraordinary Funding: "));

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/ExtraordinaryFunding.sol#L28](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/ExtraordinaryFunding.sol#L28)


```solidity
File: ./ajna-grants/src/grants/base/StandardFunding.sol

51:        bytes32 internal constant DESCRIPTION_PREFIX_HASH_STANDARD = keccak256(bytes("Standard Funding: "));

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol#L51](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol#L51)



</details>

## [NC-7] Long lines of code
Traditionally, source code lines are restricted to 80 characters. 
With contemporary screens being considerably larger, this rule can be somewhat relaxed. 
The [Solidity style guide](https://docs.soliditylang.org/en/latest/style-guide.html#maximum-line-length), however, suggests a maximum limit of 120 characters per line. 
Therefore, it's advisable to break up lines when they approach this length.

<details>

*<summary><i>This issue was found 91 times:</i></summary>*

```solidity
File: ./ajna-core/src/ERC20Pool.sol

127:         *  @dev    - update `t0Debt2ToCollateral` ratio only if loan not in auction, debt and collateral pre action are considered 0 if auction settled


201:         *  @dev    - update `t0Debt2ToCollateral` ratio only if loan not in auction, debt and collateral pre action are considered 0 if auction settled


351:         *  @dev    - no update of `t0Debt2ToCollateral` ratio as debt and collateral pre settle are not taken into account (pre debt and pre collateral = 0)

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC20Pool.sol#L127](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC20Pool.sol#L127)


```solidity
File: ./ajna-core/src/ERC721Pool.sol

134:         *  @dev    - update `t0Debt2ToCollateral` ratio only if loan not in auction, debt and collateral pre action are considered 0 if auction settled


209:         *  @dev    - update `t0Debt2ToCollateral` ratio only if loan not in auction, debt and collateral pre action are considered 0 if auction settled


394:         *  @dev    - no update of `t0Debt2ToCollateral` ratio as debt and collateral pre settle are not taken into account (pre debt and pre collateral = 0)


472:            // transfer from taker to pool the amount of quote tokens needed to cover collateral auctioned (including excess for rounded collateral)


552:         *  @param  poolTokens_ Array in pool that tracks `NFT` ids (could be tracking `NFT`s pledged by borrower or `NFT`s added by a lender in a specific bucket).


576:         *  @param  poolTokens_     Array in pool that tracks `NFT` ids (could be tracking `NFT`s pledged by borrower or `NFT`s added by a lender in a specific bucket).


604:         *  @notice Helper function to transfer an `NFT` from owner to target address (reused in code to reduce contract deployment bytecode size).


605:         *  @dev    Since `transferFrom` is used instead of `safeTransferFrom`, calling smart contracts must be careful to check that they support any received `NFT`s.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol#L134](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol#L134)


```solidity
File: ./ajna-core/src/ERC721PoolFactory.sol

17:     *  @notice Pool factory contract for creating `ERC721` pools. If a list with token ids is provided then a subset `ERC721` pool is created for the `NFT`.


18:     *  @notice Pool factory contract for creating `ERC20` pools.  If a list with token ids is provided then a subset `ERC721` pool is created for the `NFT`. Actors actions:


19:     *          - `Pool creators`: create pool by providing a fungible token for quote, a non fungible token for collateral and an interest rate between `1%-10%`.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721PoolFactory.sol#L17](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721PoolFactory.sol#L17)


```solidity
File: ./ajna-core/src/RewardsManager.sol

379:         *  @dev    Rewards are calculated as the difference in exchange rates between the last interaction burn event and the current burn event.


418:         *  @dev    Rewards are calculated as the difference in exchange rates between the last interaction burn event and the current burn event.


666:         *  @dev    Caller can claim `5%` of the rewards that have accumulated to each bucket since the last burn event, if it hasn't already been updated.


808:         *  @dev   It is used to ensure that rewards claimers will be able to claim some portion of the remaining tokens if a claim would exceed the remaining contract balance.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/RewardsManager.sol#L379](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/RewardsManager.sol#L379)


```solidity
File: ./ajna-core/src/base/Pool.sol

402:            // start a new claimable reserve auction, passing in relevant parameters such as the current pool size, debt, balance, and inflator value


554:            poolState_.collateral -= (result_.collateralAmount + result_.compensatedCollateral); // deduct collateral taken plus collateral compensated if NFT auction settled

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/Pool.sol#L402](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/Pool.sol#L402)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolBorrowerActions.sol

11:         *  @notice Called by fully colalteralized borrowers to restamp the `Neutral Price` of the loan (only if loan is fully collateralized and not in auction).


12:         *          The reason for stamping the neutral price on the loan is to provide some certainty to the borrower as to at what price they can expect to be liquidated.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolBorrowerActions.sol#L11](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolBorrowerActions.sol#L11)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolErrors.sol

44:         *  @notice Borrower is attempting to create or modify a loan such that their loan's quote token would be less than the pool's minimum debt amount.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolErrors.sol#L44](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolErrors.sol#L44)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolInternals.sol

46:        uint256 excessQuoteToken;      // [WAD] (NFT only) amount of quote tokens to be paid by taker to borrower for fractional collateral, used in take action

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolInternals.sol#L46](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolInternals.sol#L46)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolState.sol

45:         *  @return t0Debt2ToCollateral_ t0debt accross all borrowers divided by their collateral, used in determining a collateralization weighted debt.


107:         *  @dev    If a reserve auction is active, it refers to the current reserve auction. If no reserve auction is active, it refers to the last reserve auction.


434:        uint256 unclaimed;                         // [WAD] Amount of claimable reserves which has not been taken in the Claimable Reserve Auction.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolState.sol#L45](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolState.sol#L45)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolTakerActions.sol

13:         *  @param  depositTake_      If `true` then the take will happen at an auction price equal with bucket price. Auction price is used otherwise.


25:         *  @param  maxAmount_        Max amount of collateral that will be taken from the auction (max number of `NFT`s in case of `ERC721` pool).

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolTakerActions.sol#L13](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolTakerActions.sol#L13)


```solidity
File: ./ajna-core/src/interfaces/pool/erc20/IERC20PoolBorrowerActions.sol

12:         *  @dev    Can be called by borrowers with either `0` `amountToBorrow_` or `0` `collateralToPledge_`, if borrower only wants to take a single action. 


15:         *  @param  limitIndex_         Lower bound of `LUP` change (if any) that the borrower will tolerate from a creating or modifying position.


27:         *  @dev    Can be called by borrowers with either `0` `maxQuoteTokenAmountToRepay_` or `0` `collateralAmountToPull_`, if borrower only wants to take a single action. 

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolBorrowerActions.sol#L12](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolBorrowerActions.sol#L12)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721PoolBorrowerActions.sol

12:         *  @dev    Can be called by borrowers with either `0` `amountToBorrow_` or `0` `collateralToPledge`_, if borrower only wants to take a single action. 


15:         *  @param  limitIndex_       Lower bound of `LUP` change (if any) that the borrower will tolerate from a creating or modifying position.


27:         *  @dev    Can be called by borrowers with either `0` `maxQuoteTokenAmountToRepay_` or `0` `collateralAmountToPull_`, if borrower only wants to take a single action. 

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolBorrowerActions.sol#L12](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolBorrowerActions.sol#L12)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721PoolEvents.sol

28:         *  @param  toIndexLps              If non-zero, amount of LP in toIndex when collateral is merged into bucket. If 0, no collateral is merged.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolEvents.sol#L28](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolEvents.sol#L28)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721PoolLenderActions.sol

26:         *  @param  noOfNFTsToRemove_ Intergral number of `NFT`s to remove if collateral amount is met `noOfNFTsToRemove_`, else merge at bucket index, `toIndex_`.


29:         *  @return bucketLP_         If non-zero, amount of `LP` in `toIndex_` when collateral is merged into bucket. If `0`, no collateral is merged.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolLenderActions.sol#L26](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolLenderActions.sol#L26)


```solidity
File: ./ajna-core/src/interfaces/position/IPositionManagerOwnerActions.sol

22:         *  @dev    The NFT must have already been created, and the number of buckets to be memorialized at a time determined by function caller.


24:         *  @dev    `Pool.increaseLPAllowance` must be called prior to calling this method in order to allow Position manager contract to transfer LP to be memorialized.


33:         *  @dev    Position `NFT`s can only be minited with an association to pools that have been deployed by the `Ajna` `ERC20PoolFactory` or `ERC721PoolFactory`.


52:         *  @dev    The `NFT` must have already been created, and the number of buckets to be memorialized at a time determined by function caller.


54:         *  @dev    `Pool.approveLPTransferors` must be called prior to calling this method in order to allow `Position manager` contract to transfer redeemed `LP`.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerOwnerActions.sol#L22](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerOwnerActions.sol#L22)


```solidity
File: ./ajna-core/src/interfaces/rewards/IRewardsManagerOwnerActions.sol

24:         *  @dev    Automatically claims any available rewards in all existing buckets. Updates exchange rates for each new bucket the `NFT` is associated with.


25:         *  @dev    `fromBuckets_` and `toBuckets_` must be the same array length. Liquidity is moved from the `fromBuckets_` to the `toBuckets_` in the same index.


58:         *  @dev    Caller can claim `5%` of the rewards that have accumulated to each bucket since the last burn event, if it hasn't already been updated.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerOwnerActions.sol#L24](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerOwnerActions.sol#L24)


```solidity
File: ./ajna-core/src/libraries/external/BorrowerActions.sol

376:                    borrower.t0Debt != 0 && encumberedCollateral == 0 || // case when small amount of debt at a high LUP results in encumbered collateral calculated as 0

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/BorrowerActions.sol#L376](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/BorrowerActions.sol#L376)


```solidity
File: ./ajna-core/src/libraries/external/KickerActions.sol

176:            if (vars.amountToDebitFromDeposit > vars.bucketDeposit) vars.amountToDebitFromDeposit = vars.bucketDeposit; // cap the amount to remove at bucket deposit


194:                vars.amountToDebitFromDeposit = kickResult_.amountToCoverBond;                                 // cap amount to remove from deposit at amount to cover bond


196:                kickResult_.lup = Deposits.getLup(deposits_, poolState_.debt + vars.amountToDebitFromDeposit); // recalculate the LUP with the amount to cover bond


197:                kickResult_.amountToCoverBond = 0;                                                             // entire bond is covered from deposit, no additional amount to be send by lender


199:                kickResult_.amountToCoverBond -= vars.amountToDebitFromDeposit;                                // lender should send additional amount to cover bond


276:            // check that at least two weeks have passed since the last reserve auction completed, and that the auction was not kicked within the past 72 hours


371:            // check if NP is not less than price at the limit index provided by the kicker - done to prevent frontrunning kick auction call with a large amount of loan

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol#L176](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol#L176)


```solidity
File: ./ajna-core/src/libraries/external/LenderActions.sol

72:        event MoveQuoteToken(address indexed lender, uint256 indexed from, uint256 indexed to, uint256 amount, uint256 lpRedeemedFrom, uint256 lpAwardedTo, uint256 lup);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LenderActions.sol#L72](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LenderActions.sol#L72)


```solidity
File: ./ajna-core/src/libraries/external/SettlerActions.sol

134:                uint256 assets      = Maths.wmul(poolState_.t0Debt - result_.t0DebtSettled + borrower.t0Debt, poolState_.inflator) + params_.poolBalance;


194:         *  @param  borrowerCollateral_    Borrower collateral amount before auction exit (in `NFT` could be fragmented as result of partial takes).


196:         *  @return remainingCollateral_   Collateral remaining after auction is settled (same amount for `ERC20` pool, rounded collateral for `ERC721` pool).


197:         *  @return compensatedCollateral_ Amount of collateral compensated (`ERC721` settle only), to be deducted from pool pledged collateral accumulator. Always `0` for `ERC20` pools.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/SettlerActions.sol#L134](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/SettlerActions.sol#L134)


```solidity
File: ./ajna-core/src/libraries/external/TakerActions.sol

85:            bool    isRewarded;               // True if kicker is rewarded (auction price lower than neutral price), false if penalized (auction price greater than neutral price).


127:         *  @notice Performs bucket take collateral on an auction, rewards taker and kicker (if case) and updates loan info (settles auction if case).


215:                (poolState_.poolType == uint8(PoolType.ERC721) && borrower.collateral < 1e18) || // revert in case of NFT take when there isn't a full token to be taken


216:                (poolState_.poolType == uint8(PoolType.ERC20)  && borrower.collateral == 0)      // revert in case of ERC20 take when no collateral to be taken


386:                        // taker should send additional quote tokens to cover difference between collateral needed to be taken and rounded collateral, at auction price


483:         *  @return settledAuction_        True if auction is settled by the take action. (`NFT` take: rebalance borrower collateral in pool if true)


484:         *  @return remainingCollateral_   Borrower collateral remaining after take action. (`NFT` take: collateral to be rebalanced in case of `NFT` settlement)


597:         *  @param  depositTake_  If `true` then the take will happen at an auction price equal with bucket price. Auction price is used otherwise.


737:            // If there is no unscaled quote token bound, then we pass in max, but that cannot be scaled without an overflow.  So we check in the line below.


738:            vars.quoteTokenAmount = (vars.unscaledDeposit != type(uint256).max) ? Maths.wmul(vars.unscaledDeposit, vars.bucketScale) : type(uint256).max;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol#L85](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol#L85)


```solidity
File: ./ajna-core/src/libraries/helpers/RevertsHelper.sol

47:         *  @notice Check if head auction is clearable (auction is kicked and `72` hours passed since kick time or auction still has debt but no remaining collateral).

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/RevertsHelper.sol#L47](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/RevertsHelper.sol#L47)


```solidity
File: ./ajna-core/src/libraries/internal/Loans.sol

21:        @dev    The `Loans` heap is a `Max Heap` data structure (complete binary tree), the root node is the loan with the highest threshold price (`TP`)

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Loans.sol#L21](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Loans.sol#L21)


```solidity
File: ./ajna-grants/src/grants/base/ExtraordinaryFunding.sol

62:            proposalId_ = _hashProposal(targets_, values_, calldatas_, keccak256(abi.encode(DESCRIPTION_PREFIX_HASH_EXTRAORDINARY, descriptionHash_)));


92:            proposalId_ = _hashProposal(targets_, values_, calldatas_, keccak256(abi.encode(DESCRIPTION_PREFIX_HASH_EXTRAORDINARY, keccak256(bytes(description_)))));

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/ExtraordinaryFunding.sol#L62](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/ExtraordinaryFunding.sol#L62)


```solidity
File: ./ajna-grants/src/grants/base/Funding.sol

70:         * @dev    Voting power is the minimum of the amount of votes available at a snapshot block 33 blocks prior to voting start, and at the vote starting block.


72:         * @param snapshot_       One of the block numbers to retrieve the voting power at. 33 blocks prior to the block at which a proposal is available for voting.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol#L70](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol#L70)


```solidity
File: ./ajna-grants/src/grants/base/StandardFunding.sol

269:         * @dev    Voter must have voted in both the screening and funding stages, and is proportional to their share of votes across the stages.


310:            uint256 sum = _validateSlate(distributionId_, currentDistribution.endBlock, currentDistribution.fundsAvailable, proposalIds_, numProposalsInSlate);


372:            proposalId_ = _hashProposal(targets_, values_, calldatas_, keccak256(abi.encode(DESCRIPTION_PREFIX_HASH_STANDARD, keccak256(bytes(description_)))));


388:            newProposal.tokensRequested = _validateCallDatas(targets_, values_, calldatas_); // check proposal parameters are valid and update tokensRequested


421:        function _validateSlate(uint24 distributionId_, uint256 endBlock, uint256 distributionPeriodFundsAvailable_, uint256[] calldata proposalIds_, uint256 numProposalsInSlate_) internal view returns (uint256 sum_) {


444:                sum_ += uint128(proposal.fundingVotesReceived); // since we are converting from int128 to uint128, we can safely assume that the value will not overflow


541:                uint128 newVotingPower = SafeCast.toUint128(_getVotesFunding(msg.sender, votingPower, voter.remainingVotingPower, screeningStageEndBlock));


578:            if (block.number < currentDistribution.startBlock || block.number > _getScreeningStageEndBlock(currentDistribution.endBlock)) revert InvalidVote();


668:            // calculate the change in voting power used by the voter in this vote in order to accurately track the total voting power used in the funding stage


706:            if (screeningVotesCast[distributionId][account_] + votes_ > _getVotesScreening(distributionId, account_)) revert InsufficientVotingPower();


823:                _standardFundingProposals[proposals_[targetProposalId_]].votesReceived > _standardFundingProposals[proposals_[targetProposalId_ - 1]].votesReceived


886:         * @param  votingPower_            The voter's voting power in the funding round. Equal to the square of their tokens in the voting snapshot.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol#L269](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol#L269)


```solidity
File: ./ajna-grants/src/grants/interfaces/IStandardFunding.sol

152:            uint128 votingPower;           // amount of votes originally available to the voter, equal to the sum of the square of their initial votes


190:         * @param  targets_         List of contracts the proposal calldata will interact with. Should be the Ajna token contract for all proposals.


358:         * @return votingPower          The voter's voting power in the funding round. Equal to the square of their tokens in the voting snapshot.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IStandardFunding.sol#L152](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IStandardFunding.sol#L152)


```solidity
File: ./ajna-grants/src/token/AjnaToken.sol

45:         *  @notice Called by an owner of AJNA tokens to enable their tokens to be transferred by a spender address without making a seperate permit call

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol#L45](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol#L45)



</details>

## [NC-8] Inadequate indexing of event fields
Indexed event fields enhance accessibility for off-chain tools parsing events, 
proving particularly beneficial for address-based filtering. However, gas costs increase with each 
indexed field during emission, posing a challenge in maximizing the use of the allowable three fields per event. 
Events with three or more fields should ideally utilize all three indexed fields, provided that gas usage is not a 
significant concern. In events with fewer than three fields, it's advisable to index all applicable fields, balancing 
quick accessibility and efficient gas consumption

<details>

*<summary><i>This issue was found 15 times:</i></summary>*

```solidity
File: ./ajna-core/src/interfaces/pool/IPoolFactory.sol

42:        event PoolCreated(address pool_);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/IPoolFactory.sol#L42](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/IPoolFactory.sol#L42)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolEvents.sol

219:        event KickReserveAuction(
220:            uint256 claimableReservesRemaining,
221:            uint256 auctionPrice,
222:            uint256 currentBurnEpoch
223:        );


231:        event ReserveAuction(
232:            uint256 claimableReservesRemaining,
233:            uint256 auctionPrice,
234:            uint256 currentBurnEpoch
235:        );


309:        event TransferLP(
310:            address owner,
311:            address newOwner,
312:            uint256[] indexes,
313:            uint256 lp
314:        );


355:        event ResetInterestRate(
356:            uint256 oldRate,
357:            uint256 newRate
358:        );


365:        event UpdateInterestRate(
366:            uint256 oldRate,
367:            uint256 newRate
368:        );

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolEvents.sol#L219-L223](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolEvents.sol#L219-L223)


```solidity
File: ./ajna-core/src/interfaces/rewards/IRewardsManagerEvents.sol

32:        event MoveStakedLiquidity(
33:            uint256 tokenId,
34:            uint256[] fromIndexes,
35:            uint256[] toIndexes
36:        );

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerEvents.sol#L32-L36](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerEvents.sol#L32-L36)


```solidity
File: ./ajna-core/src/libraries/external/KickerActions.sol

87:        event KickReserveAuction(uint256 claimableReservesRemaining, uint256 auctionPrice, uint256 currentBurnEpoch);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol#L87](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol#L87)


```solidity
File: ./ajna-core/src/libraries/external/LPActions.sol

28:        event TransferLP(address owner, address newOwner, uint256[] indexes, uint256 lp);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LPActions.sol#L28](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LPActions.sol#L28)


```solidity
File: ./ajna-core/src/libraries/external/PoolCommons.sol

43:        event ResetInterestRate(uint256 oldRate, uint256 newRate);


44:        event UpdateInterestRate(uint256 oldRate, uint256 newRate);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol#L43](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol#L43)


```solidity
File: ./ajna-core/src/libraries/external/TakerActions.sol

103:        event ReserveAuction(uint256 claimableReservesRemaining, uint256 auctionPrice, uint256 currentBurnEpoch);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol#L103](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol#L103)


```solidity
File: ./ajna-grants/src/grants/interfaces/IFunding.sol

49:        event ProposalExecuted(uint256 proposalId);


54:        event ProposalCreated(
55:            uint256 proposalId,
56:            address proposer,
57:            address[] targets,
58:            uint256[] values,
59:            string[] signatures,
60:            bytes[] calldatas,
61:            uint256 startBlock,
62:            uint256 endBlock,
63:            string description
64:        );

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IFunding.sol#L49](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IFunding.sol#L49)


```solidity
File: ./ajna-grants/src/grants/interfaces/IGrantFund.sol

24:        event FundTreasury(uint256 amount, uint256 treasuryBalance);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IGrantFund.sol#L24](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IGrantFund.sol#L24)



</details>

# Gas Optimizations
## [G-1] Adopt custom errors over `revert()/require()` strings
From Solidity version `0.8.4`, custom errors are available which can offer gas efficiency compared to 
`revert()` or `require()` revert strings. Utilizing custom errors saves each time they're triggered, 
as it bypasses the need to allocate and store the revert string. In addition, omitting the definition of these 
strings conserves deployment gas. Switching to custom errors can be a significant optimization, enhancing the 
performance and cost-effectiveness of your smart contract.

<details>

*<summary><i>This issue was found 7 times:</i></summary>*

```solidity
File: ./ajna-core/src/PositionManager.sol

520:            require(_exists(tokenId_));

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol#L520](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol#L520)


```solidity
File: ./ajna-core/src/base/PermitERC721.sol

80:            require(block.timestamp <= deadline_, "ajna/nft-permit-expired");


98:            require(spender_ != owner, "ERC721Permit: approval to current owner");


108:                require(recoveredAddress != address(0), "ajna/nft-invalid-signature");


109:                require(recoveredAddress == owner, "ajna/nft-unauthorized");

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/PermitERC721.sol#L80](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/PermitERC721.sol#L80)


```solidity
File: ./ajna-core/src/libraries/helpers/SafeTokenNamer.sol

76:            require(len % 2 == 0 && len > 0 && len <= 40, 'SafeERC20Namer: INVALID_LEN');

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/SafeTokenNamer.sol#L76](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/SafeTokenNamer.sol#L76)


```solidity
File: ./ajna-grants/src/token/AjnaToken.sol

28:            require(to_ != address(this), "Cannot transfer tokens to the contract itself");

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol#L28](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol#L28)



</details>

## [G-2] Prefer `storage` over `memory` for structs/arrays
Retrieving data from `storage` and assigning it to a `memory` variable leads to every element of the 
`struct` or array being loaded from storage, which comes with a gas cost (`Gcoldsload`) of `2100` per element. 
If the elements are accessed from this memory variable, it incurs an additional `MLOAD` cost, bypassing a 
more affordable stack read. A more efficient approach is to declare the variable with the `storage` keyword 
and cache any elements that will be accessed multiple times in stack variables, as this only incurs the 
`Gcoldsload` cost for the elements that are actually accessed. The strategy of loading the entire struct or 
array into a memory variable is only beneficial if the function is returning the entire struct or array, 
if it's being passed to a function that needs a memory parameter, or if it's being accessed from another 
memory struct or array.

<details>

*<summary><i>This issue was found 5 times:</i></summary>*

```solidity
File: ./ajna-grants/src/grants/base/StandardFunding.sol

203:            uint256[] memory fundingProposalIds = _fundedProposalSlates[fundedSlateHash];


209:                Proposal memory proposal = _standardFundingProposals[fundingProposalIds[i]];


242:            QuarterlyDistribution memory currentDistribution = _distributions[distributionId_];


379:            QuarterlyDistribution memory currentDistribution = _distributions[_currentDistributionId];


575:            QuarterlyDistribution memory currentDistribution = _distributions[_currentDistributionId];

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol#L203](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol#L203)



</details>

## [G-3] Use bit shifting for division by two
The expression `<x> / 2` has the same result as `<x> >> 1`.
Despite the compiler's use of the `SHR` opcode for both processes, 
the division form involves an additional gas expense of `20` due to 
redirects to a compiler utility function that adds checks. These 
checks can be bypassed by incorporating `unchecked {}` when dividing by two.

<details>

*<summary><i>This issue was found 12 times:</i></summary>*

```solidity
File: ./ajna-core/src/libraries/helpers/SafeTokenNamer.sol

80:            for (uint256 i = 0; i < len / 2; i++) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/SafeTokenNamer.sol#L80](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/SafeTokenNamer.sol#L80)


```solidity
File: ./ajna-core/src/libraries/internal/Loans.sol

136:            if (index_ == ROOT_INDEX || loan_.thresholdPrice <= loans_.loans[index_ / 2].thresholdPrice){


139:              _insert(loans_, loans_.loans[index_ / 2], index_, count);


140:              _bubbleUp(loans_, loan_, index_ / 2);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Loans.sol#L136](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Loans.sol#L136)


```solidity
File: ./ajna-core/src/libraries/internal/Maths.sol

14:            return (x * y + WAD / 2) / WAD;


22:            return (x * WAD + y / 2) / y;


42:            return (x * y + 10**27 / 2) / 10**27;


58:            return (x + 10**9 / 2) / 10**9;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Maths.sol#L14](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Maths.sol#L14)


```solidity
File: ./ajna-grants/src/grants/libraries/Maths.sol

21:                uint256 x = y / 2 + 1;


24:                    x = (y / x + x) / 2;


34:            return (x * y + 10**18 / 2) / 10**18;


38:            return (x * 10**18 + y / 2) / y;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol#L21](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol#L21)



</details>

