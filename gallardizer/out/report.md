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
| [NC-8] | Typos | 1047 |
| [NC-9] | Inadequate indexing of event fields | 15 |


Total: 1232 instances over 9 issues

## Gas Optimizations
| |Issue|Instances|Total Gas Saved|
|-|:-|:-:|:-:|
| [G-1] | Adopt custom errors over `revert()/require()` strings | 7 | 350 |


Total: 7 instances over 1 issue, saving over 350 gas units

## Overall Results
**Total: 1266 instances over 17 issues, potentially saving over 350 gas units**

# Medium Risk Issues
## [M-1] Prioritize <code>_safeMint()</code> over <code>_mint()</code> for enhanced security when minting NFTs
It's recommended to prioritize the use of <code>_safeMint()</code> over <code>_mint()</code> to reduce risk of halting or reverting at early stages of a function call.
The implementation principle of <code>_safeMint()</code> ensures the recipient is an Externally Owned Account (EOA) or correctly implements the <code>IERC721Receiver</code>
interface.<br>

The main difference resides in the checks made after minting that ensure the reception of the token (e.g. Openzeppelin's <code>_checkOnERC721Received</code>).
Not adhering to this practice can lead to tokens being locked or owned by contracts that aren't equipped to handle them.

*This issue was found 1 time:*

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
<code>0.8.11</code>'s <code>abi.encodeCall</code>.<br>

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



## [L-4] Reversals due to division by zero
A division operation lacks necessary zero-value checks on any parameter used as denominator, 
which could result in the function reverting if zero is passed as an argument. It's crucial to implement 
safeguards against such division by zero errors to prevent unexpected function reverts and maintain the 
integrity of each contract's calculations.

*This issue was found 5 times:*

```solidity
File: ./ajna-core/src/libraries/helpers/PoolHelper.sol

248:            scaledAmount_ = (amount_ / tokenScale_) * tokenScale_;


389:            uint256 thresholdPrice = borrowerDebt_  * Maths.WAD / collateral_;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol)


```solidity
File: ./ajna-core/src/libraries/internal/Maths.sol

22:            return (x * WAD + y / 2) / y;


26:            return (x * WAD) / y;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Maths.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Maths.sol)


```solidity
File: ./ajna-grants/src/grants/libraries/Maths.sol

38:            return (x * 10**18 + y / 2) / y;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol)



## [L-5] Absence of array length validation
Without explicit checks for arrays to have the same length, user operations 
might not be completely executed. This is due to the disparity between the number of items 
involved in the iteration and the number of items in the subsequent arrays.

*This issue was found 3 times:*

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

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LPActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LPActions.sol)


```solidity
File: ./ajna-grants/src/grants/base/Funding.sol

52:        function _execute(
53:            uint256 proposalId_,
54:            address[] memory targets_,
55:            uint256[] memory values_,
56:            bytes[] memory calldatas_
57:        ) internal {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol)



# Non-Critical Issues
## [NC-1] The <code>nonReentrant</code> modifier should precede all other modifiers
Prioritizing reentrancy checks before any other calculations or validations within modifiers 
is a recommended practice for enhancing the security of the protected function.

*This issue was found 1 time:*

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



## [NC-3] Add descriptive revert reasons
Include descriptive reason strings in `require()` and `revert()` for 
improved error handling and user feedback. Since Solidity `0.8.4`, 
[custom errors](https://blog.soliditylang.org/2021/04/21/custom-errors/) offer a concise, 
detailed alternative for reversion, facilitating better contract usability and debugging 
also providing a more efficient way of reverting.

*This issue was found 1 time:*

```solidity
File: ./ajna-core/src/PositionManager.sol

520:            require(_exists(tokenId_));

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol)



## [NC-4] Avoid using magic numbers
It is recommended to define constants instead of relying on hex or numeric literals.
This practice enhances readability and clarity, even in assembly context, 
thereby mitigating the potential for confusion or error.

*This issue was found 61 times:*

```solidity
File: ./ajna-core/src/ERC721PoolFactory.sol

62:            try IERC165(collateral_).supportsInterface(0x80ac58cd) returns (bool supportsERC721Interface) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721PoolFactory.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721PoolFactory.sol)


```solidity
File: ./ajna-core/src/PoolInfoUtils.sol

254:            timeRemaining_              = 3 days - Maths.min(3 days, block.timestamp - auctionKickTime);


254:            timeRemaining_              = 3 days - Maths.min(3 days, block.timestamp - auctionKickTime);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PoolInfoUtils.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PoolInfoUtils.sol)


```solidity
File: ./ajna-core/src/base/PermitERC721.sol

59:                        0x8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f,


103:                    IERC1271(owner).isValidSignature(digest, abi.encodePacked(r_, s_, v_)) == 0x1626ba7e,

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/PermitERC721.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/PermitERC721.sol)


```solidity
File: ./ajna-core/src/libraries/external/KickerActions.sol

277:            if (block.timestamp < lastBurnTimestamp + 2 weeks || block.timestamp - reserveAuction_.kicked <= 72 hours) {


407:            vars.t0KickPenalty = Maths.wdiv(Maths.wmul(kickResult_.t0KickedDebt, poolState_.rate), 4 * 1e18);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol)


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

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol)


```solidity
File: ./ajna-core/src/libraries/external/PositionNFTSVG.sol

40:            string memory ownerHexString = (uint256(uint160(params_.owner))).toHexString(20);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PositionNFTSVG.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PositionNFTSVG.sol)


```solidity
File: ./ajna-core/src/libraries/external/SettlerActions.sol

112:            if ((block.timestamp - kickTime < 72 hours) && (borrower.collateral != 0)) revert AuctionNotClearable();

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/SettlerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/SettlerActions.sol)


```solidity
File: ./ajna-core/src/libraries/external/TakerActions.sol

288:            if (kicked != 0 && block.timestamp - kicked <= 72 hours) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol)


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

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol)


```solidity
File: ./ajna-core/src/libraries/helpers/RevertsHelper.sol

57:                if (block.timestamp - kickTime > 72 hours) revert AuctionNotCleared();


107:                if (loansCount >= 10)

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/RevertsHelper.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/RevertsHelper.sol)


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

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/SafeTokenNamer.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/SafeTokenNamer.sol)


```solidity
File: ./ajna-core/src/libraries/internal/Deposits.sol

82:            uint256 i  = 4096; // 1 << (_numBits - 1) = 1 << (13 - 1) = 4096


148:                lsb_ = i_ & ((i_ ^ 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff) + 1);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Deposits.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Deposits.sol)


```solidity
File: ./ajna-grants/src/grants/base/Funding.sol

125:                if (selector != bytes4(0xa9059cbb)) revert InvalidProposal();

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol)


```solidity
File: ./ajna-grants/src/grants/base/StandardFunding.sol

292:            ) / 10;


391:            if (newProposal.tokensRequested > (currentDistribution.fundsAvailable * 9 / 10)) revert InvalidProposal();


391:            if (newProposal.tokensRequested > (currentDistribution.fundsAvailable * 9 / 10)) revert InvalidProposal();


448:                if (totalTokensRequested > (gbc * 9 / 10)) {


448:                if (totalTokensRequested > (gbc * 9 / 10)) {


719:            if (screenedProposalsLength < 10 && indexInArray == -1) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol)


```solidity
File: ./ajna-grants/src/grants/libraries/Maths.sol

19:            if (y > 3) {

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol)


```solidity
File: ./ajna-grants/src/token/AjnaToken.sol

13:            _mint(tokenReceiver_, 2_000_000_000 * 10 ** decimals());

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol)


```solidity
File: ./ajna-grants/src/token/BurnWrapper.sol

50:            return 18;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/BurnWrapper.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/BurnWrapper.sol)



## [NC-5] Time-related numeric values could employ time units
For readability and consistency, numeric values associated with time should
utilize predefined [units](https://docs.soliditylang.org/en/latest/units-and-global-variables.html#time-units) 
like seconds, minutes, hours, days, or weeks.

*This issue was found 3 times:*

```solidity
File: ./ajna-grants/src/grants/base/StandardFunding.sol

34:        uint256 internal constant CHALLENGE_PERIOD_LENGTH = 50400;


40:        uint48 internal constant DISTRIBUTION_PERIOD_LENGTH = 648000;


46:        uint256 internal constant FUNDING_PERIOD_LENGTH = 72000;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol)



## [NC-6] Expressions defining constant values should employ `immutable` instead of `constant`
It's important to distinguish between `constant` and `immutable` variables, 
using each in their appropriate situations. Constants are suitable for literal values 
hard-coded into the contracts, while `immutables` should be used for expression-based values, such as a call to `keccak256()`, 
or those calculated/introduced in the `constructor`.

*This issue was found 4 times:*

```solidity
File: ./ajna-core/src/ERC20PoolFactory.sol

28:        bytes32 public constant ERC20_NON_SUBSET_HASH = keccak256("ERC20_NON_SUBSET_HASH");

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC20PoolFactory.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC20PoolFactory.sol)


```solidity
File: ./ajna-core/src/ERC721PoolFactory.sol

30:        bytes32 public constant ERC721_NON_SUBSET_HASH = keccak256("ERC721_NON_SUBSET_HASH");

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721PoolFactory.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721PoolFactory.sol)


```solidity
File: ./ajna-grants/src/grants/base/ExtraordinaryFunding.sol

28:        bytes32 internal constant DESCRIPTION_PREFIX_HASH_EXTRAORDINARY = keccak256(bytes("Extraordinary Funding: "));

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/ExtraordinaryFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/ExtraordinaryFunding.sol)


```solidity
File: ./ajna-grants/src/grants/base/StandardFunding.sol

51:        bytes32 internal constant DESCRIPTION_PREFIX_HASH_STANDARD = keccak256(bytes("Standard Funding: "));

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol)



## [NC-7] Long lines of code
Traditionally, source code lines are restricted to 80 characters. 
With contemporary screens being considerably larger, this rule can be somewhat relaxed. 
The [Solidity style guide](https://docs.soliditylang.org/en/latest/style-guide.html#maximum-line-length), however, suggests a maximum limit of 120 characters per line. 
Therefore, it's advisable to break up lines when they approach this length.

*This issue was found 91 times:*

```solidity
File: ./ajna-core/src/ERC20Pool.sol

127:         *  @dev    - update `t0Debt2ToCollateral` ratio only if loan not in auction, debt and collateral pre action are considered 0 if auction settled


201:         *  @dev    - update `t0Debt2ToCollateral` ratio only if loan not in auction, debt and collateral pre action are considered 0 if auction settled


351:         *  @dev    - no update of `t0Debt2ToCollateral` ratio as debt and collateral pre settle are not taken into account (pre debt and pre collateral = 0)

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC20Pool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC20Pool.sol)


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

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol)


```solidity
File: ./ajna-core/src/ERC721PoolFactory.sol

17:     *  @notice Pool factory contract for creating `ERC721` pools. If a list with token ids is provided then a subset `ERC721` pool is created for the `NFT`.


18:     *  @notice Pool factory contract for creating `ERC20` pools.  If a list with token ids is provided then a subset `ERC721` pool is created for the `NFT`. Actors actions:


19:     *          - `Pool creators`: create pool by providing a fungible token for quote, a non fungible token for collateral and an interest rate between `1%-10%`.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721PoolFactory.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721PoolFactory.sol)


```solidity
File: ./ajna-core/src/RewardsManager.sol

379:         *  @dev    Rewards are calculated as the difference in exchange rates between the last interaction burn event and the current burn event.


418:         *  @dev    Rewards are calculated as the difference in exchange rates between the last interaction burn event and the current burn event.


666:         *  @dev    Caller can claim `5%` of the rewards that have accumulated to each bucket since the last burn event, if it hasn't already been updated.


808:         *  @dev   It is used to ensure that rewards claimers will be able to claim some portion of the remaining tokens if a claim would exceed the remaining contract balance.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/RewardsManager.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/RewardsManager.sol)


```solidity
File: ./ajna-core/src/base/Pool.sol

402:            // start a new claimable reserve auction, passing in relevant parameters such as the current pool size, debt, balance, and inflator value


554:            poolState_.collateral -= (result_.collateralAmount + result_.compensatedCollateral); // deduct collateral taken plus collateral compensated if NFT auction settled

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/Pool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/Pool.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolBorrowerActions.sol

11:         *  @notice Called by fully colalteralized borrowers to restamp the `Neutral Price` of the loan (only if loan is fully collateralized and not in auction).


12:         *          The reason for stamping the neutral price on the loan is to provide some certainty to the borrower as to at what price they can expect to be liquidated.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolBorrowerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolBorrowerActions.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolErrors.sol

44:         *  @notice Borrower is attempting to create or modify a loan such that their loan's quote token would be less than the pool's minimum debt amount.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolErrors.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolErrors.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolInternals.sol

46:        uint256 excessQuoteToken;      // [WAD] (NFT only) amount of quote tokens to be paid by taker to borrower for fractional collateral, used in take action

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolInternals.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolInternals.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolState.sol

45:         *  @return t0Debt2ToCollateral_ t0debt accross all borrowers divided by their collateral, used in determining a collateralization weighted debt.


107:         *  @dev    If a reserve auction is active, it refers to the current reserve auction. If no reserve auction is active, it refers to the last reserve auction.


434:        uint256 unclaimed;                         // [WAD] Amount of claimable reserves which has not been taken in the Claimable Reserve Auction.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolState.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolState.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolTakerActions.sol

13:         *  @param  depositTake_      If `true` then the take will happen at an auction price equal with bucket price. Auction price is used otherwise.


25:         *  @param  maxAmount_        Max amount of collateral that will be taken from the auction (max number of `NFT`s in case of `ERC721` pool).

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolTakerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolTakerActions.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc20/IERC20PoolBorrowerActions.sol

12:         *  @dev    Can be called by borrowers with either `0` `amountToBorrow_` or `0` `collateralToPledge_`, if borrower only wants to take a single action. 


15:         *  @param  limitIndex_         Lower bound of `LUP` change (if any) that the borrower will tolerate from a creating or modifying position.


27:         *  @dev    Can be called by borrowers with either `0` `maxQuoteTokenAmountToRepay_` or `0` `collateralAmountToPull_`, if borrower only wants to take a single action. 

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolBorrowerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolBorrowerActions.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721PoolBorrowerActions.sol

12:         *  @dev    Can be called by borrowers with either `0` `amountToBorrow_` or `0` `collateralToPledge`_, if borrower only wants to take a single action. 


15:         *  @param  limitIndex_       Lower bound of `LUP` change (if any) that the borrower will tolerate from a creating or modifying position.


27:         *  @dev    Can be called by borrowers with either `0` `maxQuoteTokenAmountToRepay_` or `0` `collateralAmountToPull_`, if borrower only wants to take a single action. 

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolBorrowerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolBorrowerActions.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721PoolEvents.sol

28:         *  @param  toIndexLps              If non-zero, amount of LP in toIndex when collateral is merged into bucket. If 0, no collateral is merged.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolEvents.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolEvents.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721PoolLenderActions.sol

26:         *  @param  noOfNFTsToRemove_ Intergral number of `NFT`s to remove if collateral amount is met `noOfNFTsToRemove_`, else merge at bucket index, `toIndex_`.


29:         *  @return bucketLP_         If non-zero, amount of `LP` in `toIndex_` when collateral is merged into bucket. If `0`, no collateral is merged.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolLenderActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolLenderActions.sol)


```solidity
File: ./ajna-core/src/interfaces/position/IPositionManagerOwnerActions.sol

22:         *  @dev    The NFT must have already been created, and the number of buckets to be memorialized at a time determined by function caller.


24:         *  @dev    `Pool.increaseLPAllowance` must be called prior to calling this method in order to allow Position manager contract to transfer LP to be memorialized.


33:         *  @dev    Position `NFT`s can only be minited with an association to pools that have been deployed by the `Ajna` `ERC20PoolFactory` or `ERC721PoolFactory`.


52:         *  @dev    The `NFT` must have already been created, and the number of buckets to be memorialized at a time determined by function caller.


54:         *  @dev    `Pool.approveLPTransferors` must be called prior to calling this method in order to allow `Position manager` contract to transfer redeemed `LP`.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerOwnerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerOwnerActions.sol)


```solidity
File: ./ajna-core/src/interfaces/rewards/IRewardsManagerOwnerActions.sol

24:         *  @dev    Automatically claims any available rewards in all existing buckets. Updates exchange rates for each new bucket the `NFT` is associated with.


25:         *  @dev    `fromBuckets_` and `toBuckets_` must be the same array length. Liquidity is moved from the `fromBuckets_` to the `toBuckets_` in the same index.


58:         *  @dev    Caller can claim `5%` of the rewards that have accumulated to each bucket since the last burn event, if it hasn't already been updated.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerOwnerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerOwnerActions.sol)


```solidity
File: ./ajna-core/src/libraries/external/BorrowerActions.sol

376:                    borrower.t0Debt != 0 && encumberedCollateral == 0 || // case when small amount of debt at a high LUP results in encumbered collateral calculated as 0

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/BorrowerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/BorrowerActions.sol)


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

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol)


```solidity
File: ./ajna-core/src/libraries/external/LenderActions.sol

72:        event MoveQuoteToken(address indexed lender, uint256 indexed from, uint256 indexed to, uint256 amount, uint256 lpRedeemedFrom, uint256 lpAwardedTo, uint256 lup);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LenderActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LenderActions.sol)


```solidity
File: ./ajna-core/src/libraries/external/SettlerActions.sol

134:                uint256 assets      = Maths.wmul(poolState_.t0Debt - result_.t0DebtSettled + borrower.t0Debt, poolState_.inflator) + params_.poolBalance;


194:         *  @param  borrowerCollateral_    Borrower collateral amount before auction exit (in `NFT` could be fragmented as result of partial takes).


196:         *  @return remainingCollateral_   Collateral remaining after auction is settled (same amount for `ERC20` pool, rounded collateral for `ERC721` pool).


197:         *  @return compensatedCollateral_ Amount of collateral compensated (`ERC721` settle only), to be deducted from pool pledged collateral accumulator. Always `0` for `ERC20` pools.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/SettlerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/SettlerActions.sol)


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

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol)


```solidity
File: ./ajna-core/src/libraries/helpers/RevertsHelper.sol

47:         *  @notice Check if head auction is clearable (auction is kicked and `72` hours passed since kick time or auction still has debt but no remaining collateral).

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/RevertsHelper.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/RevertsHelper.sol)


```solidity
File: ./ajna-core/src/libraries/internal/Loans.sol

21:        @dev    The `Loans` heap is a `Max Heap` data structure (complete binary tree), the root node is the loan with the highest threshold price (`TP`)

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Loans.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Loans.sol)


```solidity
File: ./ajna-grants/src/grants/base/ExtraordinaryFunding.sol

62:            proposalId_ = _hashProposal(targets_, values_, calldatas_, keccak256(abi.encode(DESCRIPTION_PREFIX_HASH_EXTRAORDINARY, descriptionHash_)));


92:            proposalId_ = _hashProposal(targets_, values_, calldatas_, keccak256(abi.encode(DESCRIPTION_PREFIX_HASH_EXTRAORDINARY, keccak256(bytes(description_)))));

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/ExtraordinaryFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/ExtraordinaryFunding.sol)


```solidity
File: ./ajna-grants/src/grants/base/Funding.sol

70:         * @dev    Voting power is the minimum of the amount of votes available at a snapshot block 33 blocks prior to voting start, and at the vote starting block.


72:         * @param snapshot_       One of the block numbers to retrieve the voting power at. 33 blocks prior to the block at which a proposal is available for voting.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol)


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

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol)


```solidity
File: ./ajna-grants/src/grants/interfaces/IStandardFunding.sol

152:            uint128 votingPower;           // amount of votes originally available to the voter, equal to the sum of the square of their initial votes


190:         * @param  targets_         List of contracts the proposal calldata will interact with. Should be the Ajna token contract for all proposals.


358:         * @return votingPower          The voter's voting power in the funding round. Equal to the square of their tokens in the voting snapshot.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IStandardFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IStandardFunding.sol)


```solidity
File: ./ajna-grants/src/token/AjnaToken.sol

45:         *  @notice Called by an owner of AJNA tokens to enable their tokens to be transferred by a spender address without making a seperate permit call

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol)



## [NC-8] Typos


*This issue was found 1047 times:*

```solidity
File: ./ajna-core/src/ERC20Pool.sol

/// [Audit Comment] Entrypoint undercollateralized dev FlashloanablePool dev dev PoolCommons LenderActions BorrowerActions
69:     *  @dev    Calls logic from external `PoolCommons`, `LenderActions`, `BorrowerActions` and `Auction` actions libraries.


/// [Audit Comment] dev
78:        /// @dev Immutable collateral scale arg offset.


/// [Audit Comment] Initialize
82:        /*** Initialize Functions ***/


/// [Audit Comment] inheritdoc
85:        /// @inheritdoc IERC20Pool


/// [Audit Comment] initializations
99:            // increment initializations count to ensure these values can't be updated


/// [Audit Comment] Immutables
104:        /*** Immutables ***/


/// [Audit Comment] inheritdoc PoolImmutables
107:        /// @inheritdoc IERC20PoolImmutables


/// [Audit Comment] inheritdoc
112:        /// @inheritdoc IERC20Pool


/// [Audit Comment] inheritdoc PoolBorrowerActions dev dev poolBalances DebtInAuction dev poolBalances pledgedCollateral dev poolBalances dev ToCollateral pre dev dev DrawDebt
124:         *  @dev    - decrement `poolBalances.t0DebtInAuction` accumulator


/// [Audit Comment] ToCollateral
162:            // adjust t0Debt2ToCollateral ratio if loan not in auction


/// [Audit Comment] pre
165:                    result.settledAuction ? 0 : result.debtPreAction,       // debt pre settle (for loan in auction) not taken into account


/// [Audit Comment] pre
167:                    result.settledAuction ? 0 : result.collateralPreAction, // collateral pre settle (for loan in auction) not taken into account


/// [Audit Comment] inheritdoc PoolBorrowerActions dev dev poolBalances dev poolBalances DebtInAuction dev poolBalances pledgedCollateral dev ToCollateral pre dev dev RepayDebt
199:         *  @dev    - decrement `poolBalances.t0DebtInAuction accumulator`


/// [Audit Comment] ToCollateral
238:            // adjust t0Debt2ToCollateral ratio if loan not in auction


/// [Audit Comment] pre
241:                    result.settledAuction ? 0 : result.debtPreAction,       // debt pre settle (for loan in auction) not taken into account


/// [Audit Comment] pre
243:                    result.settledAuction ? 0 : result.collateralPreAction, // collateral pre settle (for loan in auction) not taken into account


/// [Audit Comment] inheritdoc PoolLenderActions dev dev DustAmountNotExceeded dev dev AddCollateral
277:         *  @dev    - `DustAmountNotExceeded()`


/// [Audit Comment] inheritdoc IPoolLenderActions dev dev RemoveCollateral
312:         *  @dev    - `RemoveCollateral`


/// [Audit Comment] inheritdoc IPoolSettlerActions dev dev poolBalances dev poolBalances DebtInAuction dev poolBalances pledgedCollateral dev ToCollateral pre pre pre dev
349:         *  @dev    - decrement `poolBalances.t0DebtInAuction` accumulator


/// [Audit Comment] inheritdoc IPoolTakerActions dev dev poolBalances dev poolBalances DebtInAuction dev poolBalances pledgedCollateral dev ToCollateral pre
381:         *  @dev    - decrement `poolBalances.t0DebtInAuction` accumulator


/// [Audit Comment] inheritdoc IPoolTakerActions dev dev poolBalances dev poolBalances DebtInAuction dev poolBalances pledgedCollateral dev ToCollateral pre
430:         *  @dev    - decrement `poolBalances.t0DebtInAuction` accumulator


/// [Audit Comment] Flashloan
458:        /*** Flashloan Functions ***/


/// [Audit Comment] inheritdoc FlashloanablePool dev flashloans
462:         *  @inheritdoc FlashloanablePool


/// [Audit Comment] param param
477:         *  @param  from_    Sender address.


/// [Audit Comment] param param
486:         *  @param  to_     Receiver address.


/// [Audit Comment] param bucketIndex
495:         *  @param  bucketIndex_  Bucket index.


/// [Audit Comment] normalized
501:            // difference between the normalized scale and the collateral token's scale

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC20Pool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC20Pool.sol)


```solidity
File: ./ajna-core/src/ERC20PoolFactory.sol

/// [Audit Comment] dev params
18:     *  @dev    Reverts if pool is already created or if params to deploy new pool are invalid.


/// [Audit Comment] dev clonable
24:        /// @dev `ERC20` clonable pool contract used to deploy the new pool.


/// [Audit Comment] dev NFTSubset
27:        /// @dev Default `bytes32` hash used by `ERC20` `Non-NFTSubset` pool types


/// [Audit Comment] inheritdoc PoolFactory dev ajna dev dev deployedPools dev deployedPoolsList dev dev DeployWithZeroAddress dev PoolAlreadyExists dev PoolInterestRateInvalid dev dev PoolCreated
43:         *  @dev    - `deployedPoolsList` array

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC20PoolFactory.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC20PoolFactory.sol)


```solidity
File: ./ajna-core/src/ERC721Pool.sol

/// [Audit Comment] Entrypoint undercollateralized dev FlashloanablePool flashloan dev dev PoolCommons LenderActions BorrowerActions
59:     *  @dev    Calls logic from external `PoolCommons`, `LenderActions`, `BorrowerActions` and `Auction` actions libraries.


/// [Audit Comment] dev NFT
67:        /// @dev Immutable NFT subset pool arg offset.


/// [Audit Comment] dev tokenIds NFT
74:        /// @dev Mapping of `tokenIds` allowed in `NFT` Subset type pool.


/// [Audit Comment] dev tokenIds
76:        /// @dev Borrower `address => array` of tokenIds pledged by borrower mapping.


/// [Audit Comment] dev tokenIds
78:        /// @dev Array of `tokenIds` in pool buckets (claimable from pool).


/// [Audit Comment] Initialize
82:        /*** Initialize Functions ***/


/// [Audit Comment] inheritdoc
85:        /// @inheritdoc IERC721Pool


/// [Audit Comment] tokenIds
101:                // add subset of tokenIds allowed in the pool


/// [Audit Comment] initializations
111:            // increment initializations count to ensure these values can't be updated


/// [Audit Comment] Immutables
116:        /*** Immutables ***/


/// [Audit Comment] inheritdoc PoolImmutables
119:        /// @inheritdoc IERC721PoolImmutables


/// [Audit Comment] inheritdoc PoolBorrowerActions dev dev poolBalances DebtInAuction dev poolBalances pledgedCollateral dev poolBalances dev ToCollateral pre dev borrowerTokenIds bucketTokenIds dev dev DrawDebtNFT
131:         *  @dev    - decrement `poolBalances.t0DebtInAuction` accumulator


/// [Audit Comment] ToCollateral
167:            // adjust t0Debt2ToCollateral ratio if loan not in auction


/// [Audit Comment] pre
170:                    result.settledAuction ? 0 : result.debtPreAction,       // debt pre settle (for loan in auction) not taken into account


/// [Audit Comment] pre
172:                    result.settledAuction ? 0 : result.collateralPreAction, // collateral pre settle (for loan in auction) not taken into account


/// [Audit Comment] inheritdoc PoolBorrowerActions dev dev poolBalances dev poolBalances DebtInAuction dev poolBalances pledgedCollateral dev ToCollateral pre dev borrowerTokenIds bucketTokenIds dev dev RepayDebt
207:         *  @dev    - decrement `poolBalances.t0DebtInAuction accumulator`


/// [Audit Comment] ToCollateral
245:            // adjust t0Debt2ToCollateral ratio if loan not in auction


/// [Audit Comment] pre
248:                    result.settledAuction ? 0 : result.debtPreAction,       // debt pre settle (for loan in auction) not taken into account


/// [Audit Comment] pre
250:                    result.settledAuction ? 0 : result.collateralPreAction, // collateral pre settle (for loan in auction) not taken into account


/// [Audit Comment] inheritdoc PoolLenderActions dev dev bucketTokenIds dev dev AddCollateralNFT
284:         *  @dev    - update `bucketTokenIds` arrays


/// [Audit Comment] inheritdoc PoolLenderActions dev dev bucketTokenIds dev dev MergeOrRemoveCollateralNFT
315:         *  @dev    - update `bucketTokenIds` arrays


/// [Audit Comment] noOfNFTsToRemove
346:                // Total collateral in buckets meets the requested removal amount, noOfNFTsToRemove_


/// [Audit Comment] inheritdoc IPoolLenderActions dev dev bucketTokenIds dev dev RemoveCollateral param noOfNFTsToRemove NFT
355:         *  @dev    - update `bucketTokenIds` arrays


/// [Audit Comment] inheritdoc IPoolSettlerActions dev dev poolBalances dev poolBalances DebtInAuction dev poolBalances pledgedCollateral dev ToCollateral pre pre pre
392:         *  @dev    - decrement `poolBalances.t0DebtInAuction` accumulator


/// [Audit Comment] inheritdoc IPoolTakerActions dev dev poolBalances dev poolBalances DebtInAuction dev poolBalances pledgedCollateral dev ToCollateral pre
428:         *  @dev    - decrement `poolBalances.t0DebtInAuction` accumulator


/// [Audit Comment] inheritdoc IPoolTakerActions dev dev poolBalances dev poolBalances DebtInAuction dev poolBalances pledgedCollateral dev ToCollateral pre
483:         *  @dev    - decrement `poolBalances.t0DebtInAuction` accumulator


/// [Audit Comment] Rebalance NFT dev dev borrowerTokens bucketTokenIds dev dev RemoveCollateral param borrowerAddress param borrowerCollateral rebalanced
519:         *  @dev    - update `borrowerTokens` and `bucketTokenIds` arrays


/// [Audit Comment] rebalance
529:            // rebalance borrower's collateral, transfer difference to floor collateral from borrower to pool claimable array


/// [Audit Comment] eg borrowerCollateral noOfTokensPledged noOfTokensToTransfer eg borrowerCollateral noOfTokensPledged noOfTokensToTransfer
534:                eg1. borrowerCollateral_ = 4.1, noOfTokensPledged = 6; noOfTokensToTransfer = 1


/// [Audit Comment] NFT msg dev param poolTokens NFT NFT NFT param tokenIds NFT msg
553:         *  @param  tokenIds_   Array of `NFT` token ids to transfer from `msg.sender` to pool.


/// [Audit Comment] NFT dev NFT NFT param toAddress param poolTokens NFT NFT NFT param amountToRemove NFT
576:         *  @param  poolTokens_     Array in pool that tracks `NFT` ids (could be tracking `NFT`s pledged by borrower or `NFT`s added by a lender in a specific bucket).


/// [Audit Comment] NFT bytecode dev transferFrom safeTransferFrom NFT param NFT param NFT param tokenId NFT
608:         *  @param  tokenId_ `NFT` token id to be transferred.


/// [Audit Comment] inheritdoc PoolState
619:        /// @inheritdoc IERC721PoolState


/// [Audit Comment] inheritdoc PoolState
624:        /// @inheritdoc IERC721PoolState

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol)


```solidity
File: ./ajna-core/src/ERC721PoolFactory.sol

/// [Audit Comment] NFT NFT dev params
17:     *  @notice Pool factory contract for creating `ERC721` pools. If a list with token ids is provided then a subset `ERC721` pool is created for the `NFT`.


/// [Audit Comment] dev clonable
26:        /// @dev `ERC721` clonable pool contract used to deploy the new pool.


/// [Audit Comment] dev NFTSubset
29:        /// @dev Default `bytes32` hash used by `ERC721` `Non-NFTSubset` pool types


/// [Audit Comment] inheritdoc PoolFactory dev ajna dev dev deployedPools dev deployedPoolsList dev dev DeployWithZeroAddress dev PoolAlreadyExists dev PoolInterestRateInvalid dev NFT NFTNotSupported dev dev PoolCreated
45:         *  @dev    - `deployedPoolsList` array


/// [Audit Comment] NFT dev tokenIds param tokenIds NFT
96:         *  @dev    If no `tokenIds` are provided, the default `ERC721_NON_SUBSET_HASH` is returned.


/// [Audit Comment] tokenIds
107:                // hash the sorted array of tokenIds


/// [Audit Comment] dev param tokenIds
115:         *  @param  tokenIds_ The array of token ids to check for sorting.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721PoolFactory.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721PoolFactory.sol)


```solidity
File: ./ajna-core/src/PoolInfoUtils.sol

/// [Audit Comment] Utils dev
29:     *  @title  Pool Info Utils contract


/// [Audit Comment] param ajnaPool Ajna param kickTime debtToCover isCollateralized collateralized neutralPrice penalized
37:         *  @param  ajnaPool_         Address of `Ajna` pool.


/// [Audit Comment] Ajna param ajnaPool Ajna param
74:         *  @param  ajnaPool_   Address of `Ajna` pool.


/// [Audit Comment] param ajnaPool Ajna param quoteTokens bucketLP exchangeRate
108:         *  @param  ajnaPool_     Address of `Ajna` pool.


/// [Audit Comment] param ajnaPool Ajna poolSize loansCount maxBorrower TP pendingInflator inflator pendingInterestFactor inflator
139:         *  @param  ajnaPool_              Address of `Ajna` pool.


/// [Audit Comment] param ajnaPool Ajna hpb HPB hpbIndex HPB htp HTP htpIndex HTP lup Utilized LUP lupIndex Utilized LUP
181:         *  @return lupIndex_ The index of the current `Lowest Utilized Price` (`LUP`) bucket, in `WAD` units.


/// [Audit Comment] Claimaible param ajnaPool Ajna claimableReserves claimableReservesRemaining auctionPrice Ajna timeRemaining
212:         *  @param  ajnaPool_                   Address of `Ajna` pool.


/// [Audit Comment] slighly
239:            // due to rounding issues, especially in Auction.settle, this can be slighly negative


/// [Audit Comment] utilization param ajnaPool Ajna poolMinDebtAmount poolCollateralization collateralization poolActualUtilization utilization utilization
262:         *  @return poolActualUtilization_ The current pool actual utilization, in `WAD` units.


/// [Audit Comment] param ajnaPool Ajna
295:         *  @param  ajnaPool_             Address of `Ajna` pool.


/// [Audit Comment] LUP
331:         *  @notice Returns current `LUP` for a given pool.


/// [Audit Comment] LUP
345:         *  @notice Returns current `LUP` index for a given pool.


/// [Audit Comment] HPB
358:         *  @notice Returns current `HPB` for a given pool.


/// [Audit Comment] HPB
371:         *  @notice Returns current `HPB` index for a given pool.


/// [Audit Comment] HTP
382:         *  @notice Returns current `HTP` for a given pool.


/// [Audit Comment] MOMP
391:         *  @notice Returns current `MOMP` for a given pool.


/// [Audit Comment] HPB
401:                // if there are no borrowers, return the HPB


/// [Audit Comment] MOMP
404:                // otherwise, calculate the MOMP


/// [Audit Comment] annualized
412:         *  @notice Calculated as greater of the current annualized interest rate divided by `52` (one week of interest) or `5` bps.


/// [Audit Comment] unutilized annualized
423:         *  @notice Calculates unutilized deposit fee rate for a pool.


/// [Audit Comment] param lp param quoteAmount
436:         *  @param  lp_          The number of `LP` to calculate amounts for.


/// [Audit Comment] param lp param collateralAmount
459:         *  @param  lp_               The number of `LP` to calculate amounts for.


/// [Audit Comment] encumberance param encumberance param encumberance encumberance Encumberance
486:         *  @param  debt_         The debt amount to calculate encumberance for.


/// [Audit Comment] collateralization param param param collateralization Collateralization
501:         *  @param  price_      The price to calculate collateralization at.


/// [Audit Comment] utilization EMA param debtColEma EMA param lupt DebtEma EMA LUP utilization
516:         *  @param  lupt0DebtEma_ The `EMA` of `LUP * t0 debt`.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PoolInfoUtils.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PoolInfoUtils.sol)


```solidity
File: ./ajna-core/src/PositionManager.sol

/// [Audit Comment] NFT NFT memorialize NFT
34:     *  @notice Used by Pool lenders to optionally mint `NFT` that represents their positions.


/// [Audit Comment] dev ajna
51:        /// @dev Mapping of `token id => ajna pool address` for which token was minted.


/// [Audit Comment] dev ajna
54:        /// @dev Mapping of `token id => ajna pool address` for which token was minted.


/// [Audit Comment] dev
56:        /// @dev Mapping of `token id => nonce` value used for permit.


/// [Audit Comment] dev
58:        /// @dev Mapping of `token id => bucket indexes` associated with position.


/// [Audit Comment] dev
61:        /// @dev Id of the next token that will be minted. Skips `0`.


/// [Audit Comment] Immutables
65:        /*** Immutables ***/


/// [Audit Comment] dev Ajna
68:        /// @dev The `ERC20` pools factory contract, used to check if address is an `Ajna` pool.


/// [Audit Comment] dev Ajna
70:        /// @dev The `ERC721` pools factory contract, used to check if address is an `Ajna` pool.


/// [Audit Comment] Structs
74:        /*** Local Var Structs ***/


/// [Audit Comment] dev Struct moveLiquidity
77:        /// @dev Struct used for `moveLiquidity` function local vars.


/// [Audit Comment] bucekt
83:            uint256 depositTime;      // lender deposit time in from bucekt


/// [Audit Comment] dev param Ajna param tokenId NFT
96:         *  @param tokenId_ Id of positions `NFT`.


/// [Audit Comment] inheritdoc IPositionManagerOwnerActions dev dev nonces tokenId dev poolKey tokenId dev dev mayInteract dev dev NoAuth dev WrongPool dev LiquidityNotRemoved dev dev
131:         *  @dev    `nonces`: remove `tokenId` nonce


/// [Audit Comment] nonces
148:            // remove permit nonces and pool mapping for burned token


/// [Audit Comment] inheritdoc IPositionManagerOwnerActions dev dev lenderInfo dev transferLP PositionManager dev dev positionIndexes dev tokenId dev dev LiquidityNotRemoved dev dev MemorializePosition
161:         *  @dev    - `transferLP()`: transfer `LP` ownership to `PositionManager` contract


/// [Audit Comment] memorialization
194:                    // check that bucket didn't go bankrupt after prior memorialization


/// [Audit Comment] PositionManager
212:            // update pool LP accounting and transfer ownership of LP to PositionManager contract


/// [Audit Comment] inheritdoc IPositionManagerOwnerActions dev dev poolKey tokenId dev dev NotAjnaPool dev dev
221:         *  @dev    `poolKey`: update `tokenId => pool` mapping


/// [Audit Comment] Ajna
232:            // revert if the address is not a valid Ajna pool


/// [Audit Comment] tokenId
235:            // record which pool the tokenId was minted in


/// [Audit Comment] inheritdoc IPositionManagerOwnerActions dev dev bucketInfo dev moveQuoteToken dev dev positionIndexes dev positionIndexes dev dev dev dev mayInteract dev dev NoAuth dev WrongPool dev LiquidityNotRemoved dev dev MoveLiquidity
249:         *  @dev    `positionIndexes`: remove from bucket index


/// [Audit Comment] bucketDeposit
273:            // ensure bucketDeposit accounts for accrued interest


/// [Audit Comment] memorialization
284:            // check that bucket hasn't gone bankrupt since memorialization


/// [Audit Comment] inheritdoc IPositionManagerOwnerActions dev dev increaseLPAllowance dev transferLP PositionManager dev dev positionIndexes dev dev dev mayInteract dev dev NoAuth dev WrongPool dev RemoveLiquidityFailed dev dev RedeemPosition
339:         *  @dev    `transferLP()`: transfer `LP` ownership from `PositionManager` contract


/// [Audit Comment] memorialization
371:                // check that bucket didn't go bankrupt after memorialization


/// [Audit Comment] transferLP
387:            // approve owner to take over the LP ownership (required for transferLP pool call)


/// [Audit Comment] lps lps PositionManager
389:            // update pool lps accounting and transfer ownership of lps from PositionManager contract


/// [Audit Comment] param tokenId Ajna
401:         *  @param  tokenId_ Address of the `Ajna` pool to retrieve accumulators of.


/// [Audit Comment] Ajna param Ajna param subsetHash Ajna
412:         *  @param  pool_       Address of the `Ajna` pool.


/// [Audit Comment] NFT memorialization param memorialized param param depositTime memorialzied
431:         *  @param  pool_        The address of the pool of memorialized position.


/// [Audit Comment] inheritdoc IPositionManagerDerivedState
449:        /// @inheritdoc IPositionManagerDerivedState


/// [Audit Comment] inheritdoc IPositionManagerDerivedState
458:        /// @inheritdoc IPositionManagerDerivedState


/// [Audit Comment] inheritdoc IPositionManagerDerivedState
465:        /// @inheritdoc IPositionManagerDerivedState


/// [Audit Comment] inheritdoc IPositionManagerDerivedState
487:        /// @inheritdoc IPositionManagerDerivedState


/// [Audit Comment] inheritdoc IPositionManagerDerivedState
498:        /// @inheritdoc IPositionManagerDerivedState


/// [Audit Comment] inheritdoc IPositionManagerDerivedState
506:        /// @inheritdoc IPositionManagerDerivedState


/// [Audit Comment] tokenURI
515:         * @dev See {IERC721Metadata-tokenURI}.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol)


```solidity
File: ./ajna-core/src/RewardsManager.sol

/// [Audit Comment] NFT NFT Ajna NFT unstake
28:     *          The Rewards contract allows pool lenders with positions `NFT` to stake and earn `Ajna` tokens. 


/// [Audit Comment] Ajna NFT
44:         * @notice Maximum percentage of tokens burned that can be claimed as `Ajna` token `LP` `NFT` rewards.


/// [Audit Comment] Ajna
48:         * @notice Maximum percentage of tokens burned that can be claimed as `Ajna` token update rewards.


/// [Audit Comment] Ajna
53:         * @dev ensures that rewards issued to staked lenders in a given pool are less than the `Ajna` tokens burned in that pool.


/// [Audit Comment] dev tokenID bool
69:        /// @dev `tokenID => epoch => bool has claimed` mapping.


/// [Audit Comment] dev
71:        /// @dev `epoch => rewards claimed` mapping.


/// [Audit Comment] dev
73:        /// @dev `epoch => update bucket rate rewards claimed` mapping.


/// [Audit Comment] dev poolAddress bucketIndex
76:        /// @dev Mapping of per pool bucket exchange rates at a given burn event `poolAddress => bucketIndex => epoch => bucket exchange rate`.


/// [Audit Comment] dev tokenID
79:        /// @dev Mapping `tokenID => Stake info`.


/// [Audit Comment] Immutables
83:        /*** Immutables ***/


/// [Audit Comment] dev Ajna
86:        /// @dev Address of the `Ajna` token.


/// [Audit Comment] dev PositionManager
88:        /// @dev The `PositionManager` contract


/// [Audit Comment] inheritdoc IRewardsManagerOwnerActions dev dev NotOwnerOfDeposit dev AlreadyClaimed dev dev ClaimRewards
109:         *  @dev    not owner `NotOwnerOfDeposit()`


/// [Audit Comment] inheritdoc IRewardsManagerOwnerActions dev dev NotOwnerOfDeposit dev params MoveStakedLiquidityInvalid dev dev MoveStakedLiquidity
131:         *  @dev    invalid index params `MoveStakedLiquidityInvalid()`


/// [Audit Comment] aready
189:            // update to bucket list exchange rates, from buckets are aready updated on claim


/// [Audit Comment] inheritdoc IRewardsManagerOwnerActions dev dev NotOwnerOfDeposit dev dev
203:         *  @dev    not owner `NotOwnerOfDeposit()`


/// [Audit Comment] msg tokenId
212:            // check that msg.sender is owner of tokenId


/// [Audit Comment] initialize
224:            // initialize last time interaction at staking epoch


/// [Audit Comment] lps
235:                // record the number of lps in bucket at the time of staking


/// [Audit Comment] NFT
249:            // transfer LP NFT to this contract


/// [Audit Comment] inheritdoc IRewardsManagerOwnerActions dev dev NotOwnerOfDeposit dev dev ClaimRewards dev Unstake
265:         *  @dev    not owner `NotOwnerOfDeposit()`


/// [Audit Comment] BucketState
291:                delete stakeInfo.snapshot[positionIndexes[i]]; // reset BucketState struct for current position


/// [Audit Comment] NFT
301:            // transfer LP NFT from contract to sender


/// [Audit Comment] inheritdoc IRewardsManagerOwnerActions dev dev UpdateExchangeRates
308:         *  @dev    - `UpdateExchangeRates`


/// [Audit Comment] inheritdoc IRewardsManagerDerivedState
324:        /// @inheritdoc IRewardsManagerDerivedState


/// [Audit Comment] inheritdoc IRewardsManagerState
351:        /// @inheritdoc IRewardsManagerState


/// [Audit Comment] inheritdoc IRewardsManagerState
362:        /// @inheritdoc IRewardsManagerState


/// [Audit Comment] NFT dev param tokenId NFT param epochToClaim NFT
380:         *  @param  tokenId_      `ID` of the staked `LP` `NFT`.


/// [Audit Comment] NFT dev param tokenId NFT param param stakingEpoch param ajnaPool param positionIndexes NFT epochRewards
419:         *  @param  tokenId_         `ID` of the staked `LP` `NFT`.


/// [Audit Comment] param param nextEventEpoch param bucketIndex param bucketLP param exchangeRate interestEarned
481:         *  @param  nextEventEpoch_ The next event epoch to check the exchange rate for.


/// [Audit Comment] lp
502:                    // calculate the equivalent amount of quote tokens given the stakes lp balance,


/// [Audit Comment] param ajnaPool param interestEarned param nextEpoch param param rewardsClaimedInEpoch newRewards
512:         *  @param  ajnaPool_              Address of the pool.


/// [Audit Comment] NFT param stakeInfo StakeInfo param tokenId NFT param epochToClaim param validateEpoch param ajnaPool Ajna
556:         *  @param  tokenId_       `ID` of the staked `LP` `NFT`.


/// [Audit Comment] param lastClaimedEpoch param burnEpochToStartClaim burnEpochsClaimed
602:         *  @param  lastClaimedEpoch_      The last burn period in which a depositor claimed rewards.


/// [Audit Comment] ajna param Ajna param currentBurnEventEpoch param lastBurnEventEpoch Ajna
629:         *  @param  pool_                  Address of the `Ajna` pool to retrieve accumulators of.


/// [Audit Comment] dev unstake claimRewards updateBucketExchangeRatesAndClaim dev param param updatedRewards
665:         *  @dev    Called as part of `stake`, `unstake`, and `claimRewards`, as well as `updateBucketExchangeRatesAndClaim`.


/// [Audit Comment] param param bucketIndex param burnEpoch
740:         *  @param  bucketIndex_ Bucket index to update exchange rate.


/// [Audit Comment] param param bucketIndex param burnEpoch param totalBurned Ajna param interestEarned
764:         *  @param  totalBurned_    Total `Ajna` tokens burned in pool.


/// [Audit Comment] interestFactor
788:                // prevents excess rewards from being provided from using a 0 value as an input to the interestFactor calculation below.


/// [Audit Comment] Ajna dev msg dev param rewardsEarned
807:         *  @dev   This method is used to transfer rewards to the `msg.sender` after a successful claim or update.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/RewardsManager.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/RewardsManager.sol)


```solidity
File: ./ajna-core/src/base/FlashloanablePool.sol

/// [Audit Comment] Flashloanable flashloans flashloans Flashloans
13:     *  @notice Pool contract with `IERC3156` flashloans capabilities.


/// [Audit Comment] flashloan param param param denormalized param calldata flashloan
24:         *  @param  amount_   The denormalized amount (dependent upon token precision) of tokens to borrow.


/// [Audit Comment] flashloans
62:         *  @notice Returns `0`, as no fee is charged for flashloans.


/// [Audit Comment] param maxLoan
74:         *  @param  token_   Address of the `ERC20` token to be lent.


/// [Audit Comment] flashloans dev flashloans overriden flashloans param flashloaned
85:         *  @dev    Allows flashloans for quote token, overriden in pool implementation to allow flashloans for other tokens.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/FlashloanablePool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/FlashloanablePool.sol)


```solidity
File: ./ajna-core/src/base/PermitERC721.sol

/// [Audit Comment] dev
11:     *  @dev Interface for token permits for ERC-721


/// [Audit Comment] EIP
15:        *  @notice `EIP-4494` permit to approve by way of owner signature.


/// [Audit Comment] EIP NFT dev
24:     *  @notice Functionality to enable `EIP-4494` permit calls as part of interactions with Position `NFT`s


/// [Audit Comment] dev
29:        /** @dev Gets the current nonce for a token ID and then increments it, returning the original value */


/// [Audit Comment] dev
32:        /** @dev The hash of the name used in the permit signature verification */


/// [Audit Comment] dev
35:        /** @dev The hash of the version string used in the permit signature verification */


/// [Audit Comment] dev keccak uint tokenId uint uint
38:        /** @dev Value is equal to keccak256("Permit(address spender,uint256 tokenId,uint256 nonce,uint256 deadline)"); */


/// [Audit Comment] nameHash versionHash
42:        /** @notice Computes the `nameHash` and `versionHash` based upon constructor input */


/// [Audit Comment] EIP SEPERATOR ledgible NFT
51:         *  @notice Calculate the `EIP-712` compliant `DOMAIN_SEPERATOR` for ledgible signature encoding.


/// [Audit Comment] keccak EIP uint chainId verifyingContract
58:                        // keccak256('EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)')


/// [Audit Comment] NFT NFT param NFT param tokenId NFT param unix param secp param secp param secp
71:         *  @param tokenId_  The id of the `NFT` being interacted with.


/// [Audit Comment] keccak isValidSignature ba
101:                // bytes4(keccak256("isValidSignature(bytes32,bytes)") == 0x1626ba7e


/// [Audit Comment] dev chainId
116:         *  @dev Gets the current chain id

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/PermitERC721.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/PermitERC721.sol)


```solidity
File: ./ajna-core/src/base/Pool.sol

/// [Audit Comment] dev entrypoint commong
69:     *  @dev    Base contract and entrypoint for commong logic of both `ERC20` and `ERC721` pools.


/// [Audit Comment] dev
84:        /// @dev Immutable pool type arg offset.


/// [Audit Comment] dev Ajna
86:        /// @dev Immutable `Ajna` token address arg offset.


/// [Audit Comment] dev
88:        /// @dev Immutable collateral token address arg offset.


/// [Audit Comment] dev
90:        /// @dev Immutable quote token address arg offset.


/// [Audit Comment] dev
92:        /// @dev Immutable quote token scale arg offset.


/// [Audit Comment] dev
114:        /// @dev deposit index -> bucket mapping


/// [Audit Comment] dev
119:        /// @dev owner address -> new owner address -> deposit index -> allowed amount mapping


/// [Audit Comment] dev transferor
122:        /// @dev owner address -> transferor address -> approved flag mapping


/// [Audit Comment] Immutables
129:         * Immutables **


/// [Audit Comment] inheritdoc IPoolImmutables
135:        /// @inheritdoc IPoolImmutables


/// [Audit Comment] inheritdoc IPoolImmutables
140:        /// @inheritdoc IPoolImmutables


/// [Audit Comment] inheritdoc IPoolImmutables
145:        /// @inheritdoc IPoolImmutables


/// [Audit Comment] inheritdoc IPoolImmutables
150:        /// @inheritdoc IPoolImmutables


/// [Audit Comment] inheritdoc IPoolLenderActions
169:        /// @inheritdoc IPoolLenderActions


/// [Audit Comment] inheritdoc IPoolLenderActions
193:        /// @inheritdoc IPoolLenderActions


/// [Audit Comment] inheritdoc IPoolLenderActions
222:        /// @inheritdoc IPoolLenderActions


/// [Audit Comment] inheritdoc IPoolLenderActions
250:        /// @inheritdoc IPoolLenderActions


/// [Audit Comment] inheritdoc IPoolBorrowerActions
266:        /// @inheritdoc IPoolBorrowerActions


/// [Audit Comment] inheritdoc IPoolKickerActions dev dev poolBalances DebtInAuction poolBalances dev ToCollateral
288:         *  @dev    increment `poolBalances.t0DebtInAuction` and `poolBalances.t0Debt` accumulators


/// [Audit Comment] ToCollateral
302:            // adjust t0Debt2ToCollateral ratio


/// [Audit Comment] inheritdoc IPoolKickerActions dev dev poolBalances DebtInAuction poolBalances dev ToCollateral
322:         *  @dev    increment `poolBalances.t0DebtInAuction` and `poolBalances.t0Debt` accumulators


/// [Audit Comment] ToCollateral
337:            // adjust t0Debt2ToCollateral ratio


/// [Audit Comment] inheritdoc IPoolKickerActions dev dev dev totalBondEscrowed
360:         *  @dev    decrease auctions `totalBondEscrowed` accumulator


/// [Audit Comment] escrowed
372:            // decrement total bond escrowed


/// [Audit Comment] inheritdoc IPoolKickerActions dev dev latestBurnEpoch dev reserveAuction latestBurnEventEpoch dev dev ReserveAuctionTooSoon dev dev KickReserveAuction
395:         *  @dev    update `reserveAuction.latestBurnEventEpoch` and burn event `timestamp` state


/// [Audit Comment] inflator
402:            // start a new claimable reserve auction, passing in relevant parameters such as the current pool size, debt, balance, and inflator value


/// [Audit Comment] msg
414:            // transfer kicker award to msg.sender


/// [Audit Comment] inheritdoc IPoolTakerActions dev dev reserveAuction totalAjnaBurned dev totalInterest totalBurned
421:         *  @dev    increment `reserveAuction.totalAjnaBurned` accumulator


/// [Audit Comment] ajna
428:            // burn required number of ajna tokens to take quote from reserves


/// [Audit Comment] inheritdoc IPoolLPActions
447:        /// @inheritdoc IPoolLPActions


/// [Audit Comment] inheritdoc IPoolLPActions
456:        /// @inheritdoc IPoolLPActions


/// [Audit Comment] inheritdoc IPoolLPActions
465:        /// @inheritdoc IPoolLPActions


/// [Audit Comment] inheritdoc IPoolLPActions
470:        /// @inheritdoc IPoolLPActions


/// [Audit Comment] inheritdoc IPoolLPActions
475:        /// @inheritdoc IPoolLPActions


/// [Audit Comment] inheritdoc IPoolLPActions
480:        /// @inheritdoc IPoolLPActions


/// [Audit Comment] dev PoolCommons accrueInterest dev dev PoolCommons accrueInterest mult Fenwick dev dev reserveAuction totalInterestEarned poolState Struct
503:         *  @dev    - `PoolCommons.accrueInterest` - `Deposits.mult` (scale `Fenwick` tree with new interest accrued):


/// [Audit Comment] inflator
522:                // calculate elapsed time since inflator was last updated


/// [Audit Comment] isNewInterestAccrued
525:                // set isNewInterestAccrued field to true if elapsed time is not 0, indicating that new interest may have accrued


/// [Audit Comment] accrueInterest inflator poolState
528:                // if new interest may have accrued, call accrueInterest function and update inflator and debt fields of poolState_ struct


/// [Audit Comment] param Struct param poolState Struct
546:         *  @param poolState_ Struct containing pool details.


/// [Audit Comment] NFT
554:            poolState_.collateral -= (result_.collateralAmount + result_.compensatedCollateral); // deduct collateral taken plus collateral compensated if NFT auction settled


/// [Audit Comment] ToCollateral
556:            // adjust t0Debt2ToCollateral ratio if auction settled by take action


/// [Audit Comment] pre
559:                    0, // debt pre take (for loan in auction) not taken into account


/// [Audit Comment] pre
561:                    0, // collateral pre take (for loan in auction) not taken into account


/// [Audit Comment] param Struct param poolState Struct
577:         *  @param poolState_ Struct containing pool details.


/// [Audit Comment] interestState ToCollateral dev interestState ToCollateral dev dev interestState ToCollateral param debtPreAction param debtPostAction param colPreAction param colPostAction
596:         *  @dev    Anytime a borrower's debt or collateral changes, the `interestState.t0Debt2ToCollateral` must be updated.


/// [Audit Comment] inflator dev PoolCommons updateInterestState dev dev PoolCommons updateInterestState dev EMA dev interestRateUpdate dev inflator inflatorUpdate dev dev PoolCommons updateInterestState UpdateInterestRate param poolState Struct param lup LUP
631:         *  @dev    `PoolCommons.updateInterestState`: `UpdateInterestRate`


/// [Audit Comment] inflator
638:            // update pool inflator


/// [Audit Comment] inflator inflatorUpdate inflatorState
642:                // if the debt in the current pool state is 0, also update the inflator and inflatorUpdate fields in inflatorState


/// [Audit Comment] param param
652:         *  @param  from_    Sender address.


/// [Audit Comment] param param
663:         *  @param  to_     Receiver address.


/// [Audit Comment] normalized
671:         *  @notice Returns the pool quote token balance normalized to `WAD` to be used for calculating pool reserves.


/// [Audit Comment] inheritdoc IPoolState
687:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
720:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
726:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolDerivedState
738:        /// @inheritdoc IPoolDerivedState


/// [Audit Comment] inheritdoc IPoolState
746:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
751:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
758:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolDerivedState
771:        /// @inheritdoc IPoolDerivedState


/// [Audit Comment] inheritdoc IPoolDerivedState
776:        /// @inheritdoc IPoolDerivedState


/// [Audit Comment] inheritdoc IPoolDerivedState
781:        /// @inheritdoc IPoolDerivedState


/// [Audit Comment] inheritdoc IPoolDerivedState
786:        /// @inheritdoc IPoolDerivedState


/// [Audit Comment] inheritdoc IPoolDerivedState
791:        /// @inheritdoc IPoolDerivedState


/// [Audit Comment] inheritdoc IPoolState
796:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
801:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
806:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
811:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
816:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
827:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
837:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
842:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
851:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
856:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
866:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
871:        /// @inheritdoc IPoolState


/// [Audit Comment] inheritdoc IPoolState
876:        /// @inheritdoc IPoolState

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/Pool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/Pool.sol)


```solidity
File: ./ajna-core/src/base/PoolDeployer.sol

/// [Audit Comment] Deployer Deployer
8:     *  @title  Pool Deployer base contract


/// [Audit Comment] dev
13:        /// @dev Min interest rate value allowed for deploying the pool (1%)


/// [Audit Comment] dev
15:        /// @dev Max interest rate value allowed for deploying the pool (10%


/// [Audit Comment] dev Ajna
18:        /// @dev `Ajna` token address


/// [Audit Comment] Ajna
19:        address public ajna; // Ajna token contract address on a network.


/// [Audit Comment] dev SubsetHash CollateralAddress QuoteAddress
25:        /// @dev SubsetHash => CollateralAddress => QuoteAddress => Pool Address mapping


/// [Audit Comment] uninitialized
26:        // slither-disable-next-line uninitialized-state


/// [Audit Comment] uninitialized
30:        // slither-disable-next-line uninitialized-state


/// [Audit Comment] integrations
54:         * @dev    This function is used by integrations to access deployed pools.


/// [Audit Comment] deployedPoolsList
64:         * @return Length of `deployedPoolsList` array.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/PoolDeployer.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/PoolDeployer.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/IERC3156FlashBorrower.sol

/// [Audit Comment] keccak FlashBorrower onFlashLoan
14:         * @return The `keccak256` hash of `ERC3156FlashBorrower.onFlashLoan`

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/IERC3156FlashBorrower.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/IERC3156FlashBorrower.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/IERC3156FlashLender.sol

/// [Audit Comment] flashloan
35:         * @return `True` when successful flashloan, `false` otherwise.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/IERC3156FlashLender.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/IERC3156FlashLender.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/IPool.sol

/// [Audit Comment] dev
39:    /// @dev Pool type enum - `ERC20` and `ERC721`


/// [Audit Comment] dev
42:    /// @dev `ERC20` token interface.


/// [Audit Comment] dev
55:    /// @dev `ERC721` token interface.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/IPool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/IPool.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/IPoolFactory.sol

/// [Audit Comment] dev funigible
7:     *  @dev   Used to deploy both funigible and non fungible pools.


/// [Audit Comment] param
40:         *  @param  pool_ The address of the new pool.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/IPoolFactory.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/IPoolFactory.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolBorrowerActions.sol

/// [Audit Comment] colalteralized restamp collateralized restamp msg
11:         *  @notice Called by fully colalteralized borrowers to restamp the `Neutral Price` of the loan (only if loan is fully collateralized and not in auction).

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolBorrowerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolBorrowerActions.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolDerivedState.sol

/// [Audit Comment] param exchangeRate
12:         *  @param  index_        The bucket index.


/// [Audit Comment] param
21:         *  @param  index_   The bucket index.


/// [Audit Comment] param
30:         *  @param  debt_  The debt amount to calculate bucket index for.


/// [Audit Comment] utilization utilization
44:         *  @notice Returns the meaningful actual utilization of the pool.


/// [Audit Comment] param
51:         *  @param  index_  Deposit index.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolDerivedState.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolDerivedState.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolErrors.sol

/// [Audit Comment] arbed
34:         *  @notice The auction price is greater than the arbed bucket price.


/// [Audit Comment] initialized
39:         *  @notice Pool already initialized.


/// [Audit Comment] drawDebt
49:         *  @notice Recipient of borrowed quote tokens doesn't match the caller of the `drawDebt` function.


/// [Audit Comment] collateralized
54:         *  @notice Borrower has a healthy over-collateralized position.


/// [Audit Comment] flashLoan
79:         *  @notice Callback invoked by `flashLoan` function did not return the expected hash (see `ERC-3156` spec).


/// [Audit Comment] flashloan flashloan
84:         *  @notice Balance of pool contract before flashloan is different than the balance after flashloan.


/// [Audit Comment] flashloan
89:         *  @notice Pool cannot facilitate a flashloan for the specified token address.


/// [Audit Comment] limitIndex
126:         *  @notice Borrower is attempting to borrow more quote token than is available before the supplied `limitIndex`.


/// [Audit Comment] HTP LUP HTP LUP
131:         *  @notice When moving quote token `HTP` must stay below `LUP`.


/// [Audit Comment] LUP
137:         *  @notice Liquidation must result in `LUP` below the borrowers threshold price.


/// [Audit Comment] attemptign
167:         *  @notice Lender must have non-zero `LP` when attemptign to remove quote token from the pool.


/// [Audit Comment] collateralization
178:         *  @notice Borrower is attempting to borrow an amount of quote tokens that will push the pool into under-collateralization.


/// [Audit Comment] LUP
183:         *  @notice Actor is attempting to remove using a bucket with price below the `LUP`.


/// [Audit Comment] attemps
213:         *  @notice Owner of the `LP` attemps to transfer `LP` to same address.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolErrors.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolErrors.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolEvents.sol

/// [Audit Comment] param param param param lpAwarded param lup LUP
20:         *  @param  lup       `LUP` calculated after deposit.


/// [Audit Comment] param param param param param lpRedeemedFrom param lpAwardedTo param lup LUP
38:         *  @param  lup            `LUP` calculated after removal.


/// [Audit Comment] param param param param lpRedeemed param lup LUP
56:         *  @param  lup        `LUP` calculated after removal.


/// [Audit Comment] param param param NFT param lpRedeemed
70:         *  @param  amount     The amount of collateral (or number of `NFT` tokens) transferred to the claimer.


/// [Audit Comment] param msg param quoteRepaid param collateralPulled NFT param lup LUP
88:         *  @param  collateralPulled The amount of collateral (or number of `NFT` tokens) transferred to the claimer.


/// [Audit Comment] param param param param
104:         *  @param  borrower   Identifies the loan being liquidated.


/// [Audit Comment] param param reciever param
119:         *  @param  reciever The address receiving withdrawn bond amount.


/// [Audit Comment] arb param param param param param bondChange param isReward bondChange penalized dev
135:         *  @param  isReward    `True` if kicker was rewarded with `bondChange` amount, `false` if kicker was penalized.


/// [Audit Comment] param param param lpAwardedTaker param lpAwardedKicker
151:         *  @param  lpAwardedTaker  Amount of `LP` awarded to the taker.


/// [Audit Comment] param param param NFT param bondChange param isReward bondChange penalized dev
167:         *  @param  isReward   `True` if kicker was rewarded with `bondChange` amount, `false` if kicker was penalized.


/// [Audit Comment] param param settledDebt dev amountRemaining
181:         *  @param  settledDebt Amount of pool debt settled in this transaction.


/// [Audit Comment] param param
191:         *  @param  borrower   Address of borrower that exits auction.


/// [Audit Comment] NFT param param param lp param
203:         *  @param  lp         Amount of `LP` given to the borrower to compensate fractional collateral (if any).


/// [Audit Comment] Claimaible claimableReservesRemaining auctionPrice Ajna currentBurnEpoch
216:         *  @return auctionPrice               Current price at which `1` quote token may be purchased, denominated in `Ajna`.


/// [Audit Comment] Claimaible claimableReservesRemaining auctionPrice Ajna currentBurnEpoch
228:         *  @return auctionPrice               Current price at which `1` quote token may be purchased, denominated in `Ajna`.


/// [Audit Comment] param param param param
243:         *  @param  owner     `LP` owner.


/// [Audit Comment] param param param param
257:         *  @param  owner     `LP` owner.


/// [Audit Comment] param param param
271:         *  @param  owner   `LP` owner.


/// [Audit Comment] param param transferors
284:         *  @param  transferors List of addresses that can transfer `LP` to lender.


/// [Audit Comment] transferors param param transferors
294:         *  @param  transferors List of addresses that won't be able to transfer `LP` to lender anymore.


/// [Audit Comment] dev PositionManager memorializePositions param param newOwner param param lp
305:         *  @param  newOwner The new owner address of the position.


/// [Audit Comment] param param lpForfeited
323:         *  @param  lpForfeited Amount of `LP` forfeited by lenders.


/// [Audit Comment] flashloan param flashloan param flashloaned param flashloaned
333:         *  @param  token    The address of token flashloaned from pool.


/// [Audit Comment] restamped param
343:         *  @notice Emitted when a loan `Neutral Price` is restamped.


/// [Audit Comment] debtEma depositEma param oldRate param newRate
352:         *  @param  oldRate Old pool interest rate.


/// [Audit Comment] param oldRate param newRate
362:         *  @param  oldRate Old pool interest rate.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolEvents.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolEvents.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolImmutables.sol

/// [Audit Comment] Immutables
6:     * @title Pool Immutables


/// [Audit Comment] quoteTokenScale
26:         *  @notice Returns the `quoteTokenScale` state variable.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolImmutables.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolImmutables.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolInternals.sol

/// [Audit Comment] Param Structs
10:    /*** Auction Param Structs ***/


/// [Audit Comment] dev Struct KickerAction
13:    /// @dev Struct used to return result of `KickerAction.kick` action.


/// [Audit Comment] lup
18:        uint256 lup;                  // [WAD] current lup


/// [Audit Comment] dev Struct SettlerAction settlePoolDebt
23:    /// @dev Struct used to hold parameters for `SettlerAction.settlePoolDebt` action.


/// [Audit Comment] dev Struct SettlerAction settlePoolDebt
30:    /// @dev Struct used to return result of `SettlerAction.settlePoolDebt` action.


/// [Audit Comment] dev Struct TakerAction TakerAction bucketTake
40:    /// @dev Struct used to return result of `TakerAction.take` and `TakerAction.bucketTake` actions.


/// [Audit Comment] NFT
46:        uint256 excessQuoteToken;      // [WAD] (NFT only) amount of quote tokens to be paid by taker to borrower for fractional collateral, used in take action


/// [Audit Comment] lup
50:        uint256 newLup;                // [WAD] current lup


/// [Audit Comment] dev Struct KickerAction kickReserveAuction
59:    /// @dev Struct used to hold parameters for `KickerAction.kickReserveAuction` action.


/// [Audit Comment] inflator
64:        uint256 inflator;    // [WAD] pool current inflator


/// [Audit Comment] Param Structs
68:    /*** Liquidity Management Param Structs ***/


/// [Audit Comment] dev Struct LenderAction addQuoteToken
71:    /// @dev Struct used to hold parameters for `LenderAction.addQuoteToken` action.


/// [Audit Comment] dev Struct LenderAction moveQuoteToken
77:    /// @dev Struct used to hold parameters for `LenderAction.moveQuoteToken` action.


/// [Audit Comment] dev Struct LenderAction removeQuoteToken
85:    /// @dev Struct used to hold parameters for `LenderAction.removeQuoteToken` action.


/// [Audit Comment] Param Structs
93:    /*** Loan Management Param Structs ***/


/// [Audit Comment] dev Struct BorrowerActions drawDebt
96:    /// @dev Struct used to return result of `BorrowerActions.drawDebt` action.


/// [Audit Comment] LUP
99:        uint256 newLup;                // [WAD] new pool LUP after draw debt


/// [Audit Comment] NFT
102:        uint256 remainingCollateral;   // [WAD] amount of borrower collateral after draw debt (for NFT can be diminished if auction settled)


/// [Audit Comment] dev Struct BorrowerActions repayDebt
112:    /// @dev Struct used to return result of `BorrowerActions.repayDebt` action.


/// [Audit Comment] LUP
115:        uint256 newLup;                // [WAD] new pool LUP after draw debt

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolInternals.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolInternals.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolKickerActions.sol

/// [Audit Comment] param param npLimitIndex
17:         *  @param  npLimitIndex_ Index of the lower bound of `NP` tolerated when kicking the auction.


/// [Audit Comment] param param npLimitIndex
27:         *  @param  npLimitIndex_ Index of the lower bound of `NP` tolerated when kicking the auction.


/// [Audit Comment] param param maxAmount
37:         *  @param  maxAmount_ The max amount to withdraw from auction bonds. Constrained by claimable amounts and liquidity


/// [Audit Comment] CRA
49:         *  @notice Called by actor to start a `Claimable Reserve Auction` (`CRA`).

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolKickerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolKickerActions.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolLPActions.sol

/// [Audit Comment] dev PositionManager param param param
13:         *  @param  spender_ The new owner of the `LP`.


/// [Audit Comment] dev PositionManager param param param
26:         *  @param  spender_ The new owner of the `LP`.


/// [Audit Comment] param param
38:         *  @param  spender_ Address that is having it's allowance revoked.


/// [Audit Comment] dev PositionManager param transferors
48:         *  @dev    Intended for use by the `PositionManager` contract.


/// [Audit Comment] dev PositionManager param transferors
57:         *  @dev    Intended for use by the `PositionManager` contract.


/// [Audit Comment] approveLpOwnership dev PositionManager memorializePositions param param newOwner param
68:         *  @param  newOwner_ The new owner address of the position.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolLPActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolLPActions.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolLenderActions.sol

/// [Audit Comment] param param param unfavorable bucketLP
18:         *  @param  expiry_   Timestamp after which this transaction will revert, preventing inclusion in a block with unfavorable price.


/// [Audit Comment] param maxAmount param fromIndex param toIndex param unfavorable fromBucketLP toBucketLP movedAmount
29:         *  @param  maxAmount_    The maximum amount of quote token to be moved by a lender.


/// [Audit Comment] param maxAmount NFT param removedAmount redeemedLP
46:         *  @param  maxAmount_     The amount of collateral (or the number of `NFT` tokens) to claim.


/// [Audit Comment] param maxAmount param removedAmount redeemedLP
58:         *  @param  maxAmount_     The max amount of quote token to be removed by a lender.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolLenderActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolLenderActions.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolSettlerActions.sol

/// [Audit Comment] param borrowerAddress param maxDepth HPB dev maxDepth
13:         *  @param  maxDepth_        Measured from `HPB`, maximum number of buckets deep to settle debt.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolSettlerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolSettlerActions.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolState.sol

/// [Audit Comment] param bondFactor bondSize kickTime kickMomp utilizes neutralPrice alreadyTaken
17:         *  @return kickMomp_     Price where the average loan utilizes deposit, at the time when the loan is liquidated (kicked).


/// [Audit Comment] accruedDebt inflator debtInAuction ToCollateral accross collateralization
45:         *  @return t0Debt2ToCollateral_ t0debt accross all borrowers divided by their collateral, used in determining a collateralization weighted debt.


/// [Audit Comment] dev params param
59:         *  @dev    NOTE: Cannot use appended underscore syntax for return params since struct is used.


/// [Audit Comment] dev params param lpAccumulator availableCollateral bankruptcyTime bucketDeposit bucketScale
76:         *  @dev    NOTE: Cannot use appended underscore syntax for return params since struct is used.


/// [Audit Comment] burnEventEpoch BurnEvent dev param burnEventEpoch burnBlock totalInterest totalBurned ajna
96:         *  @notice Mapping of burnEventEpoch to `BurnEvent` structs.


/// [Audit Comment] burnEventEpoch dev burnEventEpoch
106:         *  @notice Returns the latest `burnEventEpoch` of reserve auctions.


/// [Audit Comment] EMA debtColEma lupt DebtEma LUP debtEma depositEma MAU
115:         *  @return lupt0DebtEma_ Exponential of `LUP * t0 debt`, denominator to `TU` calculation


/// [Audit Comment] inflator inflator inflator lastUpdate inflator
132:         *  @return lastUpdate_ The timestamp of the last `inflator` update.


/// [Audit Comment] interestRate interestRateUpdate
145:         *  @return interestRateUpdate_ The timestamp of the last interest rate update.


/// [Audit Comment] param
158:         *  @param  kicker_    The address of the kicker to retrieved info for.


/// [Audit Comment] param param lpBalance depositTime
172:         *  @param  index_       Bucket index.


/// [Audit Comment] param param param utilize
190:         *  @param  index_     Bucket index.


/// [Audit Comment] param loanId thresholdPrice
203:         *  @param  loanId_         Loan's id within loan heap. Max loan is position `1`.


/// [Audit Comment] maxBorrower maxThresholdPrice noOfLoans
219:         *  @return maxBorrower_       Borrower address with highest threshold price.


/// [Audit Comment] liquidationBondEscrowed reserveAuctionUnclaimed reserveAuctionKicked totalInterestEarned
234:         *  @return liquidationBondEscrowed_ Amount of liquidation bond across all liquidators.


/// [Audit Comment] pledgedCollateral
250:         *  @notice Returns the `pledgedCollateral` state variable.


/// [Audit Comment] totalAuctions
257:         *  @return totalAuctions_ Number of active auctions.


/// [Audit Comment] dev inflator
263:         *  @dev    This value should be multiplied by inflator in order to calculate current debt of the pool.


/// [Audit Comment] DebtInAuction dev inflator DebtInAuction
269:         *  @notice Returns the `t0DebtInAuction` state variable.


/// [Audit Comment] param param transferor Transferor transferor
278:         *  @param  transferor_ Transferor that transfers `LP`.


/// [Audit Comment] Structs
289:    /*** State Structs ***/


/// [Audit Comment] dev Struct inflator
296:    /// @dev Struct holding inflator state.


/// [Audit Comment] inflator
298:        uint208 inflator;       // [WAD] pool's inflator


/// [Audit Comment] inflator
299:        uint48  inflatorUpdate; // [SEC] last time pool's inflator was updated


/// [Audit Comment] dev Struct
302:    /// @dev Struct holding pool interest state.


/// [Audit Comment] meaningfulDeposit
307:        uint256 meaningfulDeposit;   // [WAD] previous update's meaningfulDeposit


/// [Audit Comment] utilization accross
308:        uint256 t0Debt2ToCollateral; // [WAD] utilization weight accumulator, tracks debt and collateral relationship accross borrowers 


/// [Audit Comment] LUP
310:        uint256 lupt0Debt;           // [WAD] previous LUP * t0 debt


/// [Audit Comment] dev Struct EMAs
313:    /// @dev Struct holding pool EMAs state.


/// [Audit Comment] EMA MAU
315:        uint256 debtEma;             // [WAD] sample of debt EMA, numerator to MAU calculation


/// [Audit Comment] EMA MAU
316:        uint256 depositEma;          // [WAD] sample of meaningful deposit EMA, denominator to MAU calculation


/// [Audit Comment] EMA
317:        uint256 debtColEma;          // [WAD] debt squared to collateral EMA, numerator to TU calculation


/// [Audit Comment] EMA LUP
318:        uint256 lupt0DebtEma;        // [WAD] EMA of LUP * t0 debt, denominator to TU calculation


/// [Audit Comment] EMAs
319:        uint256 emaUpdate;           // [SEC] last time pool's EMAs were updated


/// [Audit Comment] dev Struct
322:    /// @dev Struct holding pool balances state.


/// [Audit Comment] LPB
325:        uint256 t0DebtInAuction;   // [WAD] Total debt in auction used to restrict LPB holder from withdrawing


/// [Audit Comment] dev Struct params
329:    /// @dev Struct holding pool params (in memory only).


/// [Audit Comment] inflator
336:        uint256 inflator;             // [WAD] current pool inflator


/// [Audit Comment] dev Struct
346:    /// @dev Struct holding lender state.


/// [Audit Comment] dev Struct
352:    /// @dev Struct holding bucket state.


/// [Audit Comment] dev Struct Fenwick
364:    /// @dev Struct holding deposits (Fenwick) values and scaling.


/// [Audit Comment] FenwickTree
366:        uint256[8193] values;  // Array of values in the FenwickTree.


/// [Audit Comment] FenwickTree accross
367:        uint256[8193] scaling; // Array of values which scale (multiply) the FenwickTree accross indexes.


/// [Audit Comment] dev Struct
374:    /// @dev Struct holding loans state.


/// [Audit Comment] dev Struct
381:    /// @dev Struct holding loan state.


/// [Audit Comment] dev Struct
387:    /// @dev Struct holding borrower state.


/// [Audit Comment] dev Struct
398:    /// @dev Struct holding pool auctions state.


/// [Audit Comment] dev Struct
408:    /// @dev Struct holding liquidation state.


/// [Audit Comment] Momp
414:        uint96  kickMomp;     // [WAD] Momp when liquidation was started


/// [Audit Comment] dev Struct
421:    /// @dev Struct holding kicker state.


/// [Audit Comment] dev Struct
431:    /// @dev Struct holding reserve auction state.


/// [Audit Comment] ajna
436:        uint256 totalAjnaBurned;                   // [WAD] Total ajna burned in the pool.


/// [Audit Comment] burnEventEpoch BurnEvent
438:        mapping (uint256 => BurnEvent) burnEvents; // Mapping burnEventEpoch => BurnEvent.


/// [Audit Comment] dev Struct
441:    /// @dev Struct holding burn event state.


/// [Audit Comment] occured
443:        uint256 timestamp;     // time at which the burn event occured

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolState.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolState.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolTakerActions.sol

/// [Audit Comment] arb param borrowerAddress borower param depositTake param HPB
12:         *  @param  borrowerAddress_  Address of the borower take is being called upon.


/// [Audit Comment] param borrowerAddress borower param maxAmount NFT param callee param callee callee atomicSwapCallback
26:         *  @param  callee_           Identifies where collateral should be sent and where quote token should be obtained.


/// [Audit Comment] CRA Ajna param maxAmount
43:         *  @notice Purchases claimable reserves during a `CRA` using `Ajna` token.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolTakerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolTakerActions.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc20/IERC20Pool.sol

/// [Audit Comment] Initializes param
23:         *  @notice Initializes a new pool, setting initial state variables.


/// [Audit Comment] param bucketIndex bucketIndex
30:         *  @param  bucketIndex_ The bucket index for which the dust limit is desired, or `0` for pledged collateral.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20Pool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20Pool.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc20/IERC20PoolBorrowerActions.sol

/// [Audit Comment] dev amountToBorrow collateralToPledge param borrowerAddress param amountToBorrow param limitIndex LUP param collateralToPledge
14:         *  @param  amountToBorrow_     The amount of quote tokens to borrow.


/// [Audit Comment] dev maxQuoteTokenAmountToRepay collateralAmountToPull param borrowerAddress param maxQuoteTokenAmountToRepay param collateralAmountToPull param param limitIndex LUP
29:         *  @param  maxQuoteTokenAmountToRepay_ The max amount of quote tokens to repay.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolBorrowerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolBorrowerActions.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc20/IERC20PoolEvents.sol

/// [Audit Comment] param param param param lpAwarded
15:         *  @param  lpAwarded Amount of `LP` awarded for the deposit. 


/// [Audit Comment] param param amountBorrowed param collateralPledged param lup LUP
29:         *  @param  lup               `LUP` after borrow.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolEvents.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolEvents.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc20/IERC20PoolFactory.sol

/// [Audit Comment] dev
9:     *  @dev   Used to deploy `ERC20` pools.


/// [Audit Comment] dev WETH ETH param param param interestRate
22:         *  @param  interestRate_ Initial interest rate of the pool.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolFactory.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolFactory.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc20/IERC20PoolImmutables.sol

/// [Audit Comment] Immutables
6:     * @title ERC20 Pool Immutables


/// [Audit Comment] collateralScale
11:         *  @notice Returns the `collateralScale` immutable.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolImmutables.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolImmutables.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc20/IERC20PoolLenderActions.sol

/// [Audit Comment] param amountToAdd param param unfavorable bucketLP
12:         *  @param  amountToAdd_    Amount of collateral to deposit.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolLenderActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolLenderActions.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc20/IERC20Taker.sol

/// [Audit Comment] param collateralAmount denormalized param quoteAmountDue Denormalized collateralAmount param calldata
9:         *  @param  quoteAmountDue   Denormalized amount of quote token required to purchase `collateralAmount` at the 

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20Taker.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20Taker.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721Pool.sol

/// [Audit Comment] Initializes param tokenIds tokenIds param
29:         *  @param  tokenIds_  Enumerates `tokenIds_` to be allowed in the pool.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721Pool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721Pool.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721PoolBorrowerActions.sol

/// [Audit Comment] dev amountToBorrow collateralToPledge param drawDebt param amountToBorrow param limitIndex LUP param tokenIdsToPledge
14:         *  @param  amountToBorrow_   The amount of quote tokens to borrow.


/// [Audit Comment] dev maxQuoteTokenAmountToRepay collateralAmountToPull param borrowerAddress param maxQuoteTokenAmountToRepay param noOfNFTsToPull NFT param param limitIndex LUP
29:         *  @param  maxQuoteTokenAmountToRepay_ The max amount of quote tokens to repay.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolBorrowerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolBorrowerActions.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721PoolErrors.sol

/// [Audit Comment] NFT tokenId
11:         *  @notice User attempted to add an `NFT` to the pool with a `tokenId` outside of the allowed subset.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolErrors.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolErrors.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721PoolEvents.sol

/// [Audit Comment] param param param tokenIds tokenIds param lpAwarded
14:         *  @param  tokenIds  Array of tokenIds to be added to the pool.


/// [Audit Comment] param param collateralMerged param toIndexLps toIndex
28:         *  @param  toIndexLps              If non-zero, amount of LP in toIndex when collateral is merged into bucket. If 0, no collateral is merged.


/// [Audit Comment] param msg param amountBorrowed param tokenIdsPledged tokenIds param lup LUP
40:         *  @param  tokenIdsPledged   Array of tokenIds to be added to the pool.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolEvents.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolEvents.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721PoolFactory.sol

/// [Audit Comment] dev
9:     *  @dev   Used to deploy non fungible pools.


/// [Audit Comment] tokenIds
18:         *  @notice User tried to deploy a pool with an array of `tokenIds` that weren't sorted, or contained duplicates.


/// [Audit Comment] dev WETH ETH param NFT param NFT param tokenIds NFT param interestRate
31:         *  @param  tokenIds_     Ids of subset `NFT` tokens.


/// [Audit Comment] NFT
43:         *  @notice User attempted to make pool with non supported `NFT` contract as collateral.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolFactory.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolFactory.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721PoolImmutables.sol

/// [Audit Comment] Immutables
6:     * @title ERC721 Pool Immutables


/// [Audit Comment] NFT NTF
11:         *  @notice Returns the type of `NFT` pool.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolImmutables.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolImmutables.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721PoolLenderActions.sol

/// [Audit Comment] param tokenIds param param unfavorable bucketLP
12:         *  @param  tokenIds_ Array of token ids to deposit.


/// [Audit Comment] accross removalIndexes NFT param removalIndexes param noOfNFTsToRemove Intergral NFT noOfNFTsToRemove toIndex param toIndex collateralMerged toIndex bucketLP toIndex
26:         *  @param  noOfNFTsToRemove_ Intergral number of `NFT`s to remove if collateral amount is met `noOfNFTsToRemove_`, else merge at bucket index, `toIndex_`.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolLenderActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolLenderActions.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721PoolState.sol

/// [Audit Comment] param tokenId
12:         *  @param  tokenId The token id to check.


/// [Audit Comment] NFT param NFT param nftIndex NFT tokenId NFT
22:         *  @param  nftIndex `NFT` index in borrower's pledged token ids array.


/// [Audit Comment] NFT param nftIndex NFT tokenId NFT
32:         *  @param  nftIndex `NFT` index in bucket's token ids array.


/// [Audit Comment] NFT param NFT NFT
41:         *  @param  borrower_ The address of borrower that pledged the `NFT`.


/// [Audit Comment] NFT NFT
49:         *  @notice Returns the total `NFT` added in pool bucket.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolState.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolState.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721Taker.sol

/// [Audit Comment] param tokenIds NFT param quoteAmountDue Denormalized collateralAmount param calldata
9:         *  @param  quoteAmountDue Denormalized amount of quote token required to purchase `collateralAmount` at the 

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721Taker.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721Taker.sol)


```solidity
File: ./ajna-core/src/interfaces/position/IPositionManagerDerivedState.sol

/// [Audit Comment] tokenId dev getter param tokenId param lp
13:         *  @param  tokenId_ Unique `ID` of token.


/// [Audit Comment] NFT dev param tokenId
25:         *  @param  tokenId_  Unique `ID` of token.


/// [Audit Comment] NFT param tokenId
34:         *  @param  tokenId_ Unique `ID` of token.


/// [Audit Comment] NFT param tokenId param
43:         *  @param  tokenId_ Unique `ID` of token.


/// [Audit Comment] tokenId param tokenId param bucketInPosition tokenId
56:         *  @param  tokenId_           Unique `ID` of token.


/// [Audit Comment] tokenId param tokenId param isBankrupt
67:         *  @param  tokenId_           Unique ID of token.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerDerivedState.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerDerivedState.sol)


```solidity
File: ./ajna-core/src/interfaces/position/IPositionManagerErrors.sol

/// [Audit Comment] utilize
11:         * @notice User attempting to utilize `LP` from a bankrupt bucket.


/// [Audit Comment] NFT
16:         * @notice User attempting to burn a `LP` `NFT` before removing liquidity.


/// [Audit Comment] authorized NFT
21:         * @notice User not authorized to interact with the specified `NFT`.


/// [Audit Comment] NFT Ajna
26:         * @notice User attempted to mint an `NFT` pointing to a pool that wasn't deployed by an `Ajna` factory.


/// [Audit Comment] NFT
31:         * @notice User failed to remove position from their `NFT`.


/// [Audit Comment] tokenId
36:         * @notice User attempting to interact with a pool that doesn't match the pool associated with the `tokenId`.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerErrors.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerErrors.sol)


```solidity
File: ./ajna-core/src/interfaces/position/IPositionManagerEvents.sol

/// [Audit Comment] NFT param param tokenId NFT
13:         *  @param  tokenId The token id of the `NFT` that was burned.


/// [Audit Comment] memorialized NFT param tokenId tokenId NFT param memorialized
22:         *  @param  tokenId The `tokenId` of the `NFT`.


/// [Audit Comment] NFT param param param tokenId tokenId NFT
35:         *  @param  tokenId The `tokenId` of the newly minted `NFT`.


/// [Audit Comment] param param tokenId tokenId NFT param fromIndex param toIndex param lpRedeemedFrom param lpAwardedTo
46:         *  @param  tokenId        The `tokenId` of the newly minted `NFT`.


/// [Audit Comment] NFT param tokenId tokenId NFT param
63:         *  @param  tokenId The `tokenId` of the `NFT`.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerEvents.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerEvents.sol)


```solidity
File: ./ajna-core/src/interfaces/position/IPositionManagerOwnerActions.sol

/// [Audit Comment] NFT dev NFT param params Calldata NFT
13:         *  @param  params_ Calldata struct supplying inputs required to update the underlying assets owed to an `NFT`.


/// [Audit Comment] memorialize NFT dev dev NFT memorialized dev dev increaseLPAllowance memorialized param params Calldata memorialization
22:         *  @dev    The NFT must have already been created, and the number of buckets to be memorialized at a time determined by function caller.


/// [Audit Comment] Ajna NFT dev NFT minited Ajna PoolFactory PoolFactory param params Calldata NFT tokenId tokenId NFT
33:         *  @dev    Position `NFT`s can only be minited with an association to pools that have been deployed by the `Ajna` `ERC20PoolFactory` or `ERC721PoolFactory`.


/// [Audit Comment] param params Calldata
43:         *  @param  params_  Calldata struct supplying inputs required to move liquidity tokens.


/// [Audit Comment] reedem NFT dev dev NFT memorialized dev dev approveLPTransferors param params Calldata
52:         *  @dev    The `NFT` must have already been created, and the number of buckets to be memorialized at a time determined by function caller.


/// [Audit Comment] Struct params
62:        /*** Struct params ***/


/// [Audit Comment] Struct NFT
66:         *  @notice Struct holding parameters for burning an `NFT`.


/// [Audit Comment] tokenId NFT
69:            uint256 tokenId; // The tokenId of the positions NFT to burn


/// [Audit Comment] NFT
70:            address pool;    // The pool address associated with burned positions NFT


/// [Audit Comment] Struct
74:         *  @notice Struct holding parameters for tracking positions.


/// [Audit Comment] tokenId NFT
77:            uint256   tokenId; // The tokenId of the positions NFT


/// [Audit Comment] memorialize
78:            uint256[] indexes; // The array of bucket indexes to memorialize positions


/// [Audit Comment] Struct
82:         *  @notice Struct holding mint parameters.


/// [Audit Comment] NFT
86:            address pool;           // The pool address associated with minted positions NFT


/// [Audit Comment] Struct
91:         *  @notice Struct holding parameters for moving the liquidity of a position.


/// [Audit Comment] tokenId NFT
94:            uint256 tokenId;   // The tokenId of the positions NFT


/// [Audit Comment] NFT
95:            address pool;      // The pool address associated with positions NFT


/// [Audit Comment] unfavorable
98:            uint256 expiry;    // Timestamp after which this TX will revert, preventing inclusion in a block with unfavorable price


/// [Audit Comment] Struct
102:         *  @notice Struct holding parameters for tracking positions.


/// [Audit Comment] tokenId NFT
105:            uint256   tokenId; // The tokenId of the positions NFT


/// [Audit Comment] NFT
106:            address   pool;    // The pool address associated with positions NFT


/// [Audit Comment] reedem
107:            uint256[] indexes; // The array of bucket indexes to reedem positions for

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerOwnerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerOwnerActions.sol)


```solidity
File: ./ajna-core/src/interfaces/position/IPositionManagerState.sol

/// [Audit Comment] NFT param tokenId NFT NFT
12:         *  @param  tokenId_ The token id of the positions `NFT`.


/// [Audit Comment] dev Struct
20:    /// @dev Struct holding Position `LP` state.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerState.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerState.sol)


```solidity
File: ./ajna-core/src/interfaces/rewards/IRewardsManagerDerivedState.sol

/// [Audit Comment] NFT param tokenId NFT param epochToClaim NFT
12:         *  @param  tokenId_      `ID` of the staked `LP` `NFT`.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerDerivedState.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerDerivedState.sol)


```solidity
File: ./ajna-core/src/interfaces/rewards/IRewardsManagerErrors.sol

/// [Audit Comment] params
20:         *  @notice User provided move index params that didn't match in size.


/// [Audit Comment] NFT
25:         *  @notice User attempted to interact with an `NFT` they aren't the owner of.


/// [Audit Comment] Ajna
30:         *  @notice Can't deploy with `Ajna` token address `0x` address.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerErrors.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerErrors.sol)


```solidity
File: ./ajna-core/src/interfaces/rewards/IRewardsManagerEvents.sol

/// [Audit Comment] NFT param NFT param ajnaPool Ajna NFT param tokenId NFT param epochsClaimed param Ajna staker
13:         *  @param  ajnaPool      Address of the `Ajna` pool the `NFT` corresponds to.


/// [Audit Comment] NFT param tokenId NFT param fromIndexes param toIndexes
28:         *  @param  tokenId     `ID` of the staked `NFT`.


/// [Audit Comment] NFT param NFT param ajnaPool Ajna NFT param tokenId NFT
41:         *  @param  ajnaPool Address of the `Ajna` pool the `NFT` corresponds to.


/// [Audit Comment] param param ajnaPool Ajna param indexesUpdated param rewardsClaimed Ajna
53:         *  @param  ajnaPool        Address of the `Ajna` pool whose exchange rates are being updated.


/// [Audit Comment] NFT param NFT param ajnaPool Ajna NFT param tokenId NFT
67:         *  @param  ajnaPool Address of the `Ajna` pool the `NFT` corresponds to.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerEvents.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerEvents.sol)


```solidity
File: ./ajna-core/src/interfaces/rewards/IRewardsManagerOwnerActions.sol

/// [Audit Comment] Ajna NFT dev NFT param tokenId NFT param epochToClaim
13:         *  @param  tokenId_      `ID` of the staked `LP` `NFT`.


/// [Audit Comment] NFT dev PositionManager moveLiquidity dev NFT dev fromBuckets toBuckets fromBuckets toBuckets param tokenId NFT param fromBuckets param toBuckets param unfavorable
25:         *  @dev    `fromBuckets_` and `toBuckets_` must be the same array length. Liquidity is moved from the `fromBuckets_` to the `toBuckets_` in the same index.


/// [Audit Comment] NFT dev NFT param tokenId NFT
41:         *  @param  tokenId_ `ID` of the `LP` `NFT` to stake in the `Rewards contract.


/// [Audit Comment] NFT param tokenId NFT
50:         *  @param  tokenId_ `ID` of the staked `LP` `NFT`.


/// [Audit Comment] dev param param
59:         *  @param  pool_    Address of the pool whose exchange rates are being updated.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerOwnerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerOwnerActions.sol)


```solidity
File: ./ajna-core/src/interfaces/rewards/IRewardsManagerState.sol

/// [Audit Comment] param tokenId NFT param
12:         *  @param  tokenId_ ID of the staked `LP` `NFT`.


/// [Audit Comment] param
23:         *  @param  epoch_   The burn epoch to track if rewards were claimed.


/// [Audit Comment] param
32:         *  @param  epoch_   The burn epoch to track if rewards were claimed.


/// [Audit Comment] param tokenId NFT NFT NFT lastClaimedEpoch NFT
41:         *  @param  tokenId_          `ID` of the `NFT` staked in the rewards contract to retrieve information about.


/// [Audit Comment] param tokenId NFT param bucketId NFT
52:         *  @param  tokenId_  `ID` of the `NFT` staked in the rewards contract to retrieve information about.


/// [Audit Comment] Structs
65:    /*** State Structs ***/


/// [Audit Comment] dev Struct
68:    /// @dev Struct holding stake info state.


/// [Audit Comment] Ajna NFT
70:        address ajnaPool;                         // address of the Ajna pool the NFT corresponds to


/// [Audit Comment] NFT
72:        address owner;                            // owner of the LP NFT


/// [Audit Comment] NFT's
74:        mapping(uint256 => BucketState) snapshot; // the LP NFT's balances and exchange rates in each bucket at the time of staking


/// [Audit Comment] dev Struct
77:    /// @dev Struct holding bucket state at stake time.


/// [Audit Comment] NFT
79:        uint128 lpsAtStakeTime;  // [WAD] LP amount the NFT owner is entitled in current bucket at the time of staking

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerState.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerState.sol)


```solidity
File: ./ajna-core/src/libraries/external/BorrowerActions.sol

/// [Audit Comment] BorrowerActions
36:        @title  BorrowerActions library


/// [Audit Comment] Structs
43:        /*** Local Var Structs ***/


/// [Audit Comment] dev Struct drawDebt
46:        /// @dev Struct used for `drawDebt` function local vars.


/// [Audit Comment] NFTs
50:            uint256 compensatedCollateral; // [WAD] amount of borrower collateral that is compensated with LP (NFTs only)


/// [Audit Comment] restamped
54:            bool    stampT0Np;             // true if loan's t0 neutral price should be restamped (when drawing debt or pledge settles auction)


/// [Audit Comment] dev Struct repayDebt
57:        /// @dev Struct used for `repayDebt` function local vars.


/// [Audit Comment] NFTs
60:            uint256 compensatedCollateral; // [WAD] amount of borrower collateral that is compensated with LP (NFTs only)


/// [Audit Comment] restamped
63:            bool    stampT0Np;             // true if loan's t0 neutral price should be restamped (when repay settles auction or pull collateral)


/// [Audit Comment] IPoolEvents
72:        // See `IPoolEvents` for descriptions


/// [Audit Comment] IPoolErrors
79:        // See `IPoolErrors` for descriptions


/// [Audit Comment] PoolBorrowerActions PoolBorrowerActions dev dev SettlerActions settleAuction removeAuction dev accumumlator dev dev totalBondEscrowed dev dev upsert dev dev dev dev dev BorrowerNotSender dev AmountLTMinDebt dev LimitIndexExceeded dev BorrowerUnderCollateralized dev dev SettlerActions settleAuction AuctionNFTSettle AuctionSettle
110:         *  @dev    - `SettlerActions._settleAuction`: `AuctionNFTSettle` or `AuctionSettle`


/// [Audit Comment] collateralized
151:                // if loan is auctioned and becomes collateralized by newly pledged collateral then settle auction


/// [Audit Comment] collateralized
159:                    // borrower becomes re-collateralized, entire borrower debt is removed from pool auctions debt accumulator


/// [Audit Comment] collateralized LUP
189:                // an auctioned borrower in not allowed to draw more debt (even if collateralized at the new LUP) if auction is not settled


/// [Audit Comment] LUP
214:                // revert if borrow drives LUP price under the specified price limit


/// [Audit Comment] lup collateralization
217:                // calculate new lup and check borrow action won't push borrower into a state of under-collateralization


/// [Audit Comment] PoolBorrowerActions PoolBorrowerActions dev dev SettlerActions settleAuction removeAuction dev accumumlator dev dev totalBondEscrowed dev dev upsert dev dev dev dev dev NoDebt dev AmountLTMinDebt dev BorrowerNotSender dev InsufficientCollateral dev LimitIndexExceeded dev dev SettlerActions settleAuction AuctionNFTSettle AuctionSettle
265:         *  @dev    - `SettlerActions._settleAuction`: `AuctionNFTSettle` or `AuctionSettle`


/// [Audit Comment] collateralized
327:                // if loan is auctioned and becomes collateralized by repaying debt then settle auction


/// [Audit Comment] collateralized
333:                        // borrower becomes re-collateralized, entire borrower debt is removed from pool auctions debt accumulator


/// [Audit Comment] collateralized LUP
366:                // an auctioned borrower in not allowed to pull collateral (even if collateralized at the new LUP) if auction is not settled


/// [Audit Comment] LUP
369:                // calculate LUP only if it wasn't calculated in repay action


/// [Audit Comment] LUP
376:                    borrower.t0Debt != 0 && encumberedCollateral == 0 || // case when small amount of debt at a high LUP results in encumbered collateral calculated as 0


/// [Audit Comment] IPoolBorrowerActions dev dev upsert dev dev dev dev dev AuctionActive dev collateralized BorrowerUnderCollateralized dev dev LoanStamped
416:         *  @dev    loan not fully collateralized `BorrowerUnderCollateralized()`


/// [Audit Comment] collateralized LUP
435:            // revert if loan is not fully collateralized at current LUP


/// [Audit Comment] dev accuratley DebtInAuction param Struct param
468:         *  @dev    Used to accuratley increment and decrement `t0DebtInAuction` accumulator.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/BorrowerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/BorrowerActions.sol)


```solidity
File: ./ajna-core/src/libraries/external/KickerActions.sol

/// [Audit Comment] undercollateralized
45:                - kick undercollateralized loans; start reserve auctions


/// [Audit Comment] Structs
50:        /*** Local Var Structs ***/


/// [Audit Comment] dev Struct
53:        /// @dev Struct used for `kick` function local vars.


/// [Audit Comment] MOMP
58:            uint256 noOfLoans;          // number of loans and auctions in pool (used to calculate MOMP)


/// [Audit Comment] MOMP
59:            uint256 momp;               // [WAD] MOMP of kicked auction


/// [Audit Comment] dev Struct kickWithDeposit
66:        /// @dev Struct used for `kickWithDeposit` function local vars.


/// [Audit Comment] unscaled
75:            uint256 bucketUnscaledDeposit;    // [WAD] unscaled amount of quote tokens in bucket


/// [Audit Comment] IPoolEvents
84:        // See `IPoolEvents` for descriptions


/// [Audit Comment] IPoolErrors
94:        // See `IPoolErrors` for descriptions


/// [Audit Comment] IPoolKickerActions KickResult
107:         *  @notice See `IPoolKickerActions` for descriptions.


/// [Audit Comment] IPoolKickerActions dev dev unscaledRemove Fenwick dev lps dev lps dev dev RemoveQuoteToken kickResult KickResult
134:         *  @dev   - `Deposits.unscaledRemove` (remove amount in `Fenwick` tree, from index): update `values` array state


/// [Audit Comment] LUP
196:                kickResult_.lup = Deposits.getLup(deposits_, poolState_.debt + vars.amountToDebitFromDeposit); // recalculate the LUP with the amount to cover bond


/// [Audit Comment] LUP
202:            // revert if the bucket price used to kick and remove is below new LUP


/// [Audit Comment] IPoolKickerActions dev dev reserveAuction dev reserveAuction dev dev NoReserves dev dev KickReserveAuction kickerAward
259:         *  @dev    update `reserveAuction.unclaimed` accumulator


/// [Audit Comment] dev dev recordAuction dev dev dev totalBondEscrowed dev dev updateKicker dev dev dev dev dev dev param Struct param Struct param Struct param poolState param borrowerAddress param limitIndex param additionalDebt LUP kickResult KickResult
320:         *  @dev    - `_recordAuction`:


/// [Audit Comment] LUP
357:            // add amount to remove to pool debt in order to calculate proposed LUP


/// [Audit Comment] collateralized
364:            // revert if kick on a collateralized borrower


/// [Audit Comment] params
369:            // calculate auction params


/// [Audit Comment] frontrunning
371:            // check if NP is not less than price at the limit index provided by the kicker - done to prevent frontrunning kick auction call with a large amount of loan


/// [Audit Comment] penalized
372:            // which will make it harder for kicker to earn a reward and more likely that the kicker is penalized


/// [Audit Comment] dev dev param Struct param bondSize bondDifference
428:         *  @param  auctions_       Struct for pool auctions state.


/// [Audit Comment] escrowed
445:                // decrement total bond escrowed by bond size 


/// [Audit Comment] escrowed
451:                // decrement total bond escrowed by kicker claimable


/// [Audit Comment] dev dev dev dev param Struct param borrowerAddress param bondSize param bondFactor param momp MOMP param neutralPrice
466:         *  @param  momp_            Current pool `MOMP`.


/// [Audit Comment] totalBondEscrowed
491:            // update totalBondEscrowed accumulator

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol)


```solidity
File: ./ajna-core/src/libraries/external/LPActions.sol

/// [Audit Comment] LPActions transferors
12:        @title  LPActions library


/// [Audit Comment] IPoolEvents
22:        // See `IPoolEvents` for descriptions


/// [Audit Comment] IPoolErrors
34:        // See `IPoolErrors` for descriptions


/// [Audit Comment] IPoolLenderActions dev dev dev dev InvalidAllowancesInput dev dev IncreaseLPAllowance
51:         *  @dev    invalid indexes and amounts input `InvalidAllowancesInput()`


/// [Audit Comment] IPoolLenderActions dev dev dev dev InvalidAllowancesInput dev dev DecreaseLPAllowance
87:         *  @dev    invalid indexes and amounts input `InvalidAllowancesInput()`


/// [Audit Comment] IPoolLenderActions dev dev dev dev RevokeLPAllowance
124:         *  @dev    - `RevokeLPAllowance`


/// [Audit Comment] IPoolLenderActions dev dev approvedTransferors dev dev ApproveLPTransferors
152:         *  @dev    `approvedTransferors` mapping


/// [Audit Comment] IPoolLenderActions dev dev approvedTransferors dev dev RevokeLPTransferors
176:         *  @dev    `approvedTransferors` mapping


/// [Audit Comment] IPoolLenderActions dev dev dev lps depositTime dev dev dev InvalidIndex dev NoAllowance dev dev TransferLP
201:         *  @dev    increment new `lender.lps` accumulator and `lender.depositTime` state


/// [Audit Comment] msg transferor
217:            // revert if msg.sender is not the new owner and is not approved as a transferor by the new owner

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LPActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LPActions.sol)


```solidity
File: ./ajna-core/src/libraries/external/LenderActions.sol

/// [Audit Comment] LenderActions
24:        @title  LenderActions library


/// [Audit Comment] Structs
32:        /*** Local Var Structs ***/


/// [Audit Comment] dev Struct moveQuoteToken
35:        /// @dev Struct used for `moveQuoteToken` function local vars.


/// [Audit Comment] unscaled
47:            uint256 toBucketUnscaledDeposit;    // Amount of unscaled deposit in to bucket.


/// [Audit Comment] dev Struct removeQuoteToken
54:        /// @dev Struct used for `removeQuoteToken` function local vars.


/// [Audit Comment] LPB
57:            uint256 lpConstraint;      // [WAD] Constraint in LPB terms.


/// [Audit Comment] LPB
58:            uint256 bucketLP;          // [WAD] Total LPB in the bucket.


/// [Audit Comment] IPoolEvents
69:        // See `IPoolEvents` for descriptions


/// [Audit Comment] IPoolErrors
79:        // See `IPoolErrors` for descriptions


/// [Audit Comment] PoolLenderActions PoolLenderActions dev dev addCollateral dev lps dev addLenderLP lps depositTime dev dev InvalidIndex
101:         *  @dev      `addLenderLP`: increment `lender.lps` accumulator and `lender.depositTime `state


/// [Audit Comment] IPoolLenderActions dev dev unscaledAdd Fenwick dev lps dev lps depositTime dev dev InvalidIndex dev BucketBankruptcyBlock dev dev AddQuoteToken
133:         *  @dev    - increment `lender.lps` accumulator and `lender.depositTime` state


/// [Audit Comment] unutilized
164:            // charge unutilized deposit fee where appropriate


/// [Audit Comment] LUP
187:            // only need to recalculate LUP if the deposit was above it


/// [Audit Comment] IPoolLenderActions dev dev removeMaxDeposit dev unscaledRemove Fenwick dev unscaledAdd Fenwick dev lps dev lps depositTime dev lps dev lps dev dev MoveToSameIndex dev DustAmountNotExceeded dev InvalidIndex dev dev BucketBankruptcy dev MoveQuoteToken
209:         *  @dev    - increment `lender.lps` accumulator and `lender.depositTime` state for to bucket


/// [Audit Comment] unutilized LUP LUP
269:            // apply unutilized deposit fee if quote token is moved from above the LUP to below the LUP


/// [Audit Comment] htp lup LUP HTP
290:            // check loan book's htp against new lup, revert if move drives LUP below HTP


/// [Audit Comment] lp
317:                // bucket is bankrupt and deposit was done before bankruptcy time, reset lender lp amount


/// [Audit Comment] IPoolLenderActions dev dev removeMaxDeposit dev unscaledRemove Fenwick dev lps dev lps dev dev NoClaim dev LUP HTP LUPBelowHTP dev dev RemoveQuoteToken dev BucketBankruptcy
352:         *  @dev    `LUP` lower than `HTP` `LUPBelowHTP()`


/// [Audit Comment] htp lup
395:                // check loan book's htp doesn't exceed new lup


/// [Audit Comment] lup htp htp lup LUP
399:                // this can happen if lup and htp are less than min bucket price and htp > lup (since LUP is capped at min bucket price)


/// [Audit Comment] IPoolLenderActions dev dev lps dev lps dev dev InsufficientCollateral dev InsufficientLP dev dev BucketBankruptcy
433:         *  @dev    decrement `lender.lps` accumulator


/// [Audit Comment] dev dev removeMaxCollateral dev lps dev lps dev dev InsufficientCollateral dev NoClaim
506:         *  @dev      decrement `lender.lps` accumulator


/// [Audit Comment] PoolLenderActions dev dev addCollateral dev lps dev lps depositTime dev dev CannotMergeToHigherPrice
536:         *  @dev      increment `lender.lps` accumulator and `lender.depositTime` state


/// [Audit Comment] collateralAmount noOfBuckets
553:            // Loop over buckets, exit if collateralAmount is reached or max noOfBuckets is reached


/// [Audit Comment] toIndex
574:                // Merge totalled collateral to specified bucket, toIndex_


/// [Audit Comment] dev dev lps dev lps dev dev InsufficientCollateral dev NoClaim dev dev BucketBankruptcy collateralAmount lpAmount
595:         *  @dev    decrement `lender.lps` accumulator


/// [Audit Comment] LPB
639:            // limit withdrawal by the lender's LPB


/// [Audit Comment] collateralAmount
641:                // withdraw collateralAmount_ as is


/// [Audit Comment] dev dev unscaledRemove Fenwick dev removedAmount redeemedLP unscaledRemaining unscaled
680:         *  @dev    - `Deposits.unscaledRemove` (remove amount in `Fenwick` tree, from index):


/// [Audit Comment] pseudocode LPB
705:            // Below is pseudocode explaining the logic behind finding the constrained amount of deposit and LPB


/// [Audit Comment] scaledRemovedAmount maxAmount scaledDeposit
706:            // scaledRemovedAmount is constrained by the scaled maxAmount(in QT), the scaledDeposit constraint, and


/// [Audit Comment] LPB LPB
707:            // the lender LPB exchange rate in scaled deposit-to-LPB for the bucket:


/// [Audit Comment] scaledRemovedAmount maxAmount scaledDeposit lenderLPBalance exchangeRate
708:            // scaledRemovedAmount = min ( maxAmount_, scaledDeposit, lenderLPBalance*exchangeRate)


/// [Audit Comment] redeemedLP maxAmount scaledExchangeRate scaledDeposit exchangeRate lenderLPBalance
709:            // redeemedLP_ = min ( maxAmount_/scaledExchangeRate, scaledDeposit/exchangeRate, lenderLPBalance)


/// [Audit Comment] depositConstraint
716:                // depositConstraint is binding constraint


/// [Audit Comment] scaledDeposit
720:                // scaledDeposit is binding constraint


/// [Audit Comment] FenwickTree
737:            Deposits.unscaledRemove(deposits_, params_.index, unscaledRemovedAmount); // update FenwickTree

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LenderActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LenderActions.sol)


```solidity
File: ./ajna-core/src/libraries/external/PoolCommons.sol

/// [Audit Comment] PoolCommons params utilization
18:        @title  PoolCommons library


/// [Audit Comment] ln
35:        int256  internal constant NEG_H_MAU_HOURS      = -0.057762265046662105 * 1e18; // -ln(2)/12


/// [Audit Comment] ln
36:        int256  internal constant NEG_H_TU_HOURS       = -0.008251752149523158 * 1e18; // -ln(2)/84


/// [Audit Comment] IPoolEvents
42:        // See `IPoolEvents` for descriptions


/// [Audit Comment] Structs
47:        /*** Local Var Structs ***/


/// [Audit Comment] dev Struct updateInterestState
50:        /// @dev Struct used for `updateInterestState` function local vars.


/// [Audit Comment] EMAs dev dev EMA dev interestRateUpdate dev dev UpdateInterestRate ResetInterestRate
80:         *  @dev    - `UpdateInterestRate` / `ResetInterestRate`


/// [Audit Comment] EMA
90:            // load existing EMA values


/// [Audit Comment] params
99:            // calculate new interest params


/// [Audit Comment] EMAs
116:            // update EMAs only once per block


/// [Audit Comment] EMAs initialize EMAs
119:                // first time EMAs are updated, initialize EMAs


/// [Audit Comment] EMA MAU
130:                    // calculate the t0 debt EMA, used for MAU


/// [Audit Comment] EMA MAU
136:                    // update the meaningful deposit EMA, used for MAU


/// [Audit Comment] EMA
142:                    // calculate the debt squared to collateral EMA, used for TU


/// [Audit Comment] EMA LUP
148:                    // calculate the EMA of LUP * t0 debt


/// [Audit Comment] EMAs
155:                // save EMAs in storage


/// [Audit Comment] EMA
161:                // save last EMA update time


/// [Audit Comment] debtEma depositEma
165:            // reset interest rate if pool rate > 10% and debtEma < 5% of depositEma


/// [Audit Comment] params
200:            // save new interest rate params to storage


/// [Audit Comment] fenwick dev dev mult Fenwick dev param emaParams Struct EMA param Struct param poolState param thresholdPrice param inflator newInflator inflator
212:         *  @param  emaParams_      Struct for pool `EMA`s state.


/// [Audit Comment] inflator
226:            // Scale the borrower inflator to update amount of interest owed by borrowers


/// [Audit Comment] HTP
234:            if (htp > MAX_PRICE)      accrualIndex = 1;                 // if HTP is over the highest price bucket then no buckets earn interest


/// [Audit Comment] HTP
235:            else if (htp < MIN_PRICE) accrualIndex = MAX_FENWICK_INDEX; // if HTP is under the lowest price bucket then all buckets earn interest


/// [Audit Comment] HPT
236:            else                      accrualIndex = _indexOf(htp);     // else HPT bucket earn interest


/// [Audit Comment] lup htp
239:            // accrual price is less of lup and htp, and prices decrease as index increases


/// [Audit Comment] fenwick
250:                // Scale the fenwick tree to update amount of debt owed to lenders


/// [Audit Comment] utilization
273:            // meaningful actual utilization


/// [Audit Comment] utilization
275:            // meaningful actual utilization * 1.02


/// [Audit Comment] utilization
279:                // calculate meaningful actual utilization for interest rate update


/// [Audit Comment] utilization
284:            // calculate target utilization


/// [Audit Comment] tu mau tu mau
290:            // raise rates if 4*(tu-1.02*mau) < (tu+1.02*mau-1)^2-1


/// [Audit Comment] tu mau tu mau
293:            // decrease rates if 4*(tu-mau) > 1-(tu+mau-1)^2


/// [Audit Comment] utilization param debtEma EMA param depositEma EMA utilization utilization
304:         *  @param  debtEma_     `EMA` of pool debt.


/// [Audit Comment] param mau utilization
317:         *  @param  mau_ Meaningful actual utilization.


/// [Audit Comment] MAU
323:            // Net Interest Margin = ((1 - MAU1)^(1/3) * 0.15)


/// [Audit Comment] MAU MAU MAU
324:            // Where MAU1 is MAU capped at 100% (min(MAU,1))


/// [Audit Comment] PRBMath
327:            // PRBMath library forbids raising a number < 1e18 to a power.  Using the product and quotient rules of 


/// [Audit Comment] MAU
329:            // Net Interest Margin = ((s - MAU1) * s)^(1/3) / s^(1/3) * 0.15


/// [Audit Comment] unutilized infinitessimal
332:            // If unutilized deposit is infinitessimal, lenders get 100% of interest.


/// [Audit Comment] unutilized
336:                // cubic root of the percentage of meaningful unutilized deposit


/// [Audit Comment] param Struct param DebtInAuction param nonAuctionedT param inflator inflator param ToCollateral ToCollateral meaningfulDeposit
348:         *  @param  inflator_            Pool's current inflator.


/// [Audit Comment] inflator param interestRate param inflator
377:         *  @param  elapsed_        Time elapsed since last inflator update.


/// [Audit Comment] inflator inflator param inflator inflator param inflatorUpdate inflator param interestRate inflator
390:         *  @param  inflatorUpdate Timestamp when inflator was updated.


/// [Audit Comment] utilization dev
406:         *  @notice Calculates lender interest margin for a given meaningful actual utilization.


/// [Audit Comment] utilization dev
416:         *  @notice Calculates pool meaningful actual utilization.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol)


```solidity
File: ./ajna-core/src/libraries/external/PositionNFTSVG.sol

/// [Audit Comment] NFT SVG SVG NFT
9:        @title  Position NFT SVG library


/// [Audit Comment] Params Structs
17:        /*** Params Structs ***/


/// [Audit Comment] NFT
23:            uint256 tokenId;              // the ID of positions NFT token


/// [Audit Comment] NFT
24:            address pool;                 // the address of pool tracked in positions NFT token


/// [Audit Comment] NFT
25:            address owner;                // the owner of positions NFT token


/// [Audit Comment] NFT
26:            uint256[] indexes;            // the array of price buckets index with LP to be tracked by the NFT


/// [Audit Comment] JSON
42:            // encode metadata as JSON object in base64

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PositionNFTSVG.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PositionNFTSVG.sol)


```solidity
File: ./ajna-core/src/libraries/external/SettlerActions.sol

/// [Audit Comment] Structs
44:        /*** Local Var Structs ***/


/// [Audit Comment] dev Struct settlePoolDebtWithDeposit
47:        /// @dev Struct used for `_settlePoolDebtWithDeposit` function local vars.


/// [Audit Comment] HPB
51:            uint256 hpbCollateral;      // [WAD] amount of collateral in HPB bucket


/// [Audit Comment] unscaled HPB
52:            uint256 hpbUnscaledDeposit; // [WAD] unscaled amount of of quote tokens in HPB bucket before settle


/// [Audit Comment] HPB
53:            uint256 hpbLP;              // [WAD] amount of LP in HPB bucket


/// [Audit Comment] unscaled
59:            uint256 unscaledDeposit;    // [WAD] unscaled amount of quote tokens in bucket


/// [Audit Comment] IPoolEvents
66:        // See `IPoolEvents` for descriptions


/// [Audit Comment] IPoolErrors
76:        // See `IPoolErrors` for descriptions


/// [Audit Comment] IPoolSettlerActions HPB HPB dev dev dev dev NoAuction dev AuctionNotClearable dev dev SettleResult
93:         *  @dev    loan is not in auction `NoAuction()`


/// [Audit Comment] HPB
119:            // 1. settle debt with HPB deposit


/// [Audit Comment] HPB
141:                // 3. forgive bad debt from next HPB


/// [Audit Comment] dev dev AuctionNFTSettle AuctionSettle param Struct param Struct param Struct param borrowerAddress param borrowerCollateral NFT param poolType remainingCollateral compensatedCollateral
190:         *  @param  auctions_              Struct for pool auctions state.


/// [Audit Comment] NFTs
214:                // if there's fraction of NFTs remaining then reward difference to borrower as LP in auction price bucket


/// [Audit Comment] liquidationBondEscrowed dev dev accumumlator dev dev param Struct param
262:         *  @dev    decrement kicker locked accumulator, increment kicker claimable accumumlator


/// [Audit Comment] HPB dev dev unscaledRemove Fenwick dev dev addCollateral dev lps dev lps depositTime dev dev BucketBankruptcy param Struct param Struct param params Struct params param Struct param inflator inflator remainingt HPB remainingCollateral HPB bucketDepth
314:         *  @dev      increment `lender.lps` accumulator and `lender.depositTime` state


/// [Audit Comment] HPB
377:                    // use HPB bucket to swap loan collateral for loan debt


/// [Audit Comment] HPB dev dev unscaledRemove Fenwick dev dev lps bankruptcyTime dev dev BucketBankruptcy param Struct param Struct param params Struct params param Struct param inflator inflator remainingt
430:         *  @param  params_          Struct containing params for settle action.


/// [Audit Comment] unclaimable
468:                        // existing LP for the bucket shall become unclaimable

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/SettlerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/SettlerActions.sol)


```solidity
File: ./ajna-core/src/libraries/external/TakerActions.sol

/// [Audit Comment] bucketTake
44:                - `take` and `bucketTake` auctioned collateral; take reserves


/// [Audit Comment] Params Structs
49:        /*** Function Params Structs ***/


/// [Audit Comment] dev Struct bucketTake params
52:        /// @dev Struct used to hold `bucketTake` function params.


/// [Audit Comment] arb
55:            bool    depositTake;     // deposit or arb take, used by bucket take


/// [Audit Comment] inflator
57:            uint256 inflator;        // [WAD] current pool inflator


/// [Audit Comment] dev Struct params
61:        /// @dev Struct used to hold `take` function params.


/// [Audit Comment] inflator
65:            uint256 inflator;        // [WAD] current pool inflator


/// [Audit Comment] NFT
66:            uint256 poolType;        // pool type (ERC20 or NFT)


/// [Audit Comment] Structs
71:        /*** Local Var Structs ***/


/// [Audit Comment] dev Struct
74:        /// @dev Struct used for `take` function local vars.


/// [Audit Comment] beeing
77:            uint256 bondChange;               // [WAD] The change made on the bond size (beeing reward or penalty).


/// [Audit Comment] NFT
83:            uint256 excessQuoteToken;         // [WAD] Difference of quote token that borrower receives after take (for fractional NFT only)


/// [Audit Comment] penalized
85:            bool    isRewarded;               // True if kicker is rewarded (auction price lower than neutral price), false if penalized (auction price greater than neutral price).


/// [Audit Comment] Fenwick bpf
87:            uint256 quoteTokenAmount;         // [WAD] Scaled quantity in Fenwick tree and before 1-bpf factor, paid for collateral


/// [Audit Comment] repayed
88:            uint256 t0RepayAmount;            // [WAD] The amount of debt (quote tokens) that is recovered / repayed by take t0 terms.


/// [Audit Comment] intial
90:            uint256 t0DebtPenalty;            // [WAD] Borrower's t0 penalty - 7% from current debt if intial take, 0 otherwise.


/// [Audit Comment] Unscaled
91:            uint256 unscaledDeposit;          // [WAD] Unscaled bucket quantity


/// [Audit Comment] unscaled
92:            uint256 unscaledQuoteTokenAmount; // [WAD] The unscaled token amount that taker should pay for collateral taken.


/// [Audit Comment] IPoolEvents
99:        // See `IPoolEvents` for descriptions


/// [Audit Comment] IPoolErrors
109:        // See `IPoolErrors` for descriptions


/// [Audit Comment] IPoolTakerActions dev dev InsufficientCollateral TakeResult
129:         *  @dev    not enough collateral to take `InsufficientCollateral()`


/// [Audit Comment] params
168:            // update pool params after take


/// [Audit Comment] IPoolTakerActions dev dev InsufficientCollateral TakeResult
196:         *  @dev    insufficient collateral to take `InsufficientCollateral()`


/// [Audit Comment] NFT
215:                (poolState_.poolType == uint8(PoolType.ERC721) && borrower.collateral < 1e18) || // revert in case of NFT take when there isn't a full token to be taken


/// [Audit Comment] params
240:            // update pool params after take


/// [Audit Comment] IPoolTakerActions dev dev reserveAuction dev dev NoReservesAuction dev dev ReserveAuction
273:         *  @dev    decrement `reserveAuction.unclaimed` accumulator


/// [Audit Comment] ajna
301:                // accumulate additional ajna burned


/// [Audit Comment] dev dev param Struct param Struct param params Struct params Struct
331:         *  @param  params_   Struct containing take action params details.


/// [Audit Comment] calculateTakeFlows
348:            // These are placeholder max values passed to calculateTakeFlows because there is no explicit bound on the


/// [Audit Comment] bucketTake
349:            // quote token amount in take calls (as opposed to bucketTake)


/// [Audit Comment] NFT
354:            // for NFT take make sure the take flow and bond change calculation happens for the rounded collateral that can be taken


/// [Audit Comment] analagous
361:            // reduce the debt of the borrower -- analagous to the amount of deposit in the bucket for a bucket take


/// [Audit Comment] dev dev BucketTake param Struct param Struct param Struct param Struct param params Struct Struct
406:         *  @param  params_   Struct containing take action details.


/// [Audit Comment] arbed
427:            // revert if no quote tokens in arbed bucket


/// [Audit Comment] arb
432:            // cannot arb with a price lower than the auction price


/// [Audit Comment] recollateralized dev dev AmountLTMinDebt param Struct param Struct param Struct param Struct param poolState Struct param param borrowerAddress newLup LUP settledAuction NFT rebalance remainingCollateral NFT rebalanced NFT compensatedCollateral
479:         *  @param  poolState_             Struct containing pool details.


/// [Audit Comment] lup
512:            // calculate new lup with repaid debt from take


/// [Audit Comment] dev dev dev dev totalBondEscrowed param Struct param Struct param Struct
554:         *  @param  auctions_     Struct for pool auctions state.


/// [Audit Comment] neutralPrice
564:                // take is below neutralPrice, Kicker is rewarded


/// [Audit Comment] neutralPrice penalized
569:                // take is above neutralPrice, Kicker is penalized


/// [Audit Comment] dev dev addLenderLP dev lps depositTime dev lps ender depositTime dev dev dev totalBondEscrowed dev unscaledRemove Fenwick dev dev lps dev dev BucketTakeLPAwarded param Struct param Struct param Struct param Struct param bucketIndex HPB param depositTake param Struct
582:         *  @dev      increment taker `lender.lps` accumulator and `lender.depositTime` state


/// [Audit Comment] arb LPB
623:            // if arb take - taker is awarded collateral * (bucket price - auction price) worth (in quote token terms) units of LPB in the bucket


/// [Audit Comment] LPB
634:            // the bondholder/kicker is awarded bond change worth of LPB in the bucket


/// [Audit Comment] neutralPrice penalized
641:                // take is above neutralPrice, Kicker is penalized


/// [Audit Comment] dev alreadyTaken dev NoAuction TakeNotPastCooldown param param param param inflator inflator
676:         *  @param  inflator_    The pool's inflator, used to calculate borrower debt.


/// [Audit Comment] param totalCollateral param inflator inflator param TakeParams buckettake
720:         *  @param  inflator_               Current pool inflator.


/// [Audit Comment] bpf
732:            // from the borrower point of view, the price is actually (1-bpf) * price, as the rewards to the


/// [Audit Comment] unscaled
737:            // If there is no unscaled quote token bound, then we pass in max, but that cannot be scaled without an overflow.  So we check in the line below.


/// [Audit Comment] neutralPrice
768:                // take is below neutralPrice, Kicker is rewarded


/// [Audit Comment] neutralPrice penalized
771:                // take is above neutralPrice, Kicker is penalized

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol)


```solidity
File: ./ajna-core/src/libraries/helpers/PoolHelper.sol

/// [Audit Comment] dev
19:        /// @dev constant price indices defining the min and max of the potential price range


/// [Audit Comment] dev
27:        /// @dev step amounts in basis points. This is a constant across pools at `0.005`, achieved by dividing `WAD` by `10,000`


/// [Audit Comment] Fenwick dev BucketIndexOutOfBounds dev math EVM dev dev Fenwick dev Fenwick dev fenwick dev fenwick dev fenwick dev dev dev
35:         *  @dev    Fenwick index is converted to bucket index.


/// [Audit Comment] Fenwick
47:            // Lowest Fenwick index is highest price, so invert the index and offset by highest bucket index.


/// [Audit Comment] Fenwick dev BucketPriceOutOfBounds dev dev dev dev dev Fenwick
68:         *  @dev    `Fenwick index = 7388 - bucket index + 3232`


/// [Audit Comment] param param loansCount minDebtAmount
94:         *  @param  loansCount_    The number of loans in pool.


/// [Audit Comment] annualized param interestRate
109:         *  @param  interestRate_ The current interest rate.


/// [Audit Comment] annualized
115:            // greater of the current annualized interest rate divided by 52 (one week of interest) or 5 bps


/// [Audit Comment] unutilized LUP interestRate
120:         * @notice Calculates the unutilized deposit fee, charged to lenders who deposit below the `LUP`.


/// [Audit Comment] annualized
127:            // current annualized rate divided by 365 (24 hours of interest)


/// [Audit Comment] param param inflator inflator param ToCollateral
134:         *  @param  inflator_            Pool's borrower inflator.


/// [Audit Comment] Collateralization param collateralization param collateralization param collateralization param collateralization
147:         *  @param debt_       Debt to calculate collateralization for.


/// [Audit Comment] param bucketIndex pricePrecisionAdjustment Unscaled
171:         *  @param  bucketIndex_              Index of the bucket, or `0` for encumbered collateral with no bucket affinity.


/// [Audit Comment] optimization
177:            // conditional is a gas optimization


/// [Audit Comment] param bucketCollateral param bucketLP param param lenderLPBalance param bucketPrice collateralAmount
187:         *  @param  bucketCollateral_ Amount of collateral in bucket.


/// [Audit Comment] lp
201:            // max collateral to lp


/// [Audit Comment] param bucketLP param bucketCollateral param param lenderLPBalance param maxQuoteToken param bucketPrice quoteTokenAmount
214:         *  @param  bucketLP_         Amount of `LP` in bucket.


/// [Audit Comment] param param tokenScale scaledAmount
241:         *  @param  tokenScale_   Scale of the token, presented as a power of `10`.


/// [Audit Comment] param param tokenScale scaledAmount
254:         *  @param  tokenScale_   Scale of the token, presented as a power of `10`.


/// [Audit Comment] param param poolSize param totalBondEscrowed escrowed param reserveAuctionUnclaimed param quoteTokenBalance
277:         *  @param  totalBondEscrowed_       Total bond escrowed.


/// [Audit Comment] param reserveAuctionKicked
296:         *  @param  reserveAuctionKicked_ Time when reserve auction was started (kicked).


/// [Audit Comment] param kickMomp MOMP param neutralPrice param kickTime
317:         *  @param  kickMomp_     `MOMP` recorded at the time of kick.


/// [Audit Comment] dev param param param neutralPrice param bondFactor bondSize param auctionPrice bpf
343:         *  @param bondFactor_   Factor used to determine bondSize.


/// [Audit Comment] BPF BondFactor neutralPrice neutralPrice thresholdPrice
358:                // BPF = BondFactor * min(1, max(-1, (neutralPrice - price) / (neutralPrice - thresholdPrice)))


/// [Audit Comment] param borrowerDebt param param momp momp
382:         *  @param  momp_         Current pool `momp`.


/// [Audit Comment] bondFactor MOMP thresholdPrice MOMP
391:            // bondFactor = min(30%, max(1%, (MOMP - thresholdPrice) / MOMP))

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/PoolHelper.sol)


```solidity
File: ./ajna-core/src/libraries/helpers/RevertsHelper.sol

/// [Audit Comment] IPoolErrors
19:        // See `IPoolErrors` for descriptions


/// [Audit Comment] dev RemoveDepositLockedByAuctionDebt param DebtInAuction param param inflator inflator
32:         *  @param  inflator_        The pool inflator used to properly assess t0 debt in auctions.


/// [Audit Comment] clearable dev AuctionNotCleared clearable
48:         *  @dev    Reverts with `AuctionNotCleared` if auction is clearable.


/// [Audit Comment] MEV dev LimitIndexExceeded param newPrice LUP param limitIndex
68:         *  @param newPrice_   New price to be compared with given limit price (can be `LUP`, `NP`).


/// [Audit Comment] unfavorable dev TransactionExpired param
81:         *  @dev    Reverts with `TransactionExpired` if expired.


/// [Audit Comment] honored dev DustAmountNotExceeded AmountLTMinDebt param param poolDebt param borrowerDebt param quoteDust
94:         *  @param  poolDebt_     Total pool debt, used to calculate average debt.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/RevertsHelper.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/RevertsHelper.sol)


```solidity
File: ./ajna-core/src/libraries/helpers/SafeTokenNamer.sol

/// [Audit Comment] keccak
14:            // 0x95d89b41 = bytes4(keccak256("symbol()"))


/// [Audit Comment] fdde keccak
24:            // 0x06fdde03 = bytes4(keccak256("name()"))


/// [Audit Comment] len
74:        // converts an address to the uppercase hex string, extracting only len bytes (up to 20, multiple of 2)


/// [Audit Comment] unicode ascii
94:        // this method converts those values to the unicode/ascii code point for the hex representation

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/SafeTokenNamer.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/helpers/SafeTokenNamer.sol)


```solidity
File: ./ajna-core/src/libraries/internal/Buckets.sol

/// [Audit Comment] IPoolError
19:        // See `IPoolError` for descriptions


/// [Audit Comment] coresponding dev lps dev addLenderLP dev lps depositTime param param param collateralAmountToAdd param bucketPrice addedLP
30:         *  @dev    increment `lender.lps` accumulator and `lender.depositTime` state


/// [Audit Comment] dev lps param param bankruptcyTime param param lpAmount
70:         *  @param  bankruptcyTime_ Time when bucket become insolvent.


/// [Audit Comment] param bucketCollateral param bucketLP param param param bucketPrice lp
96:         *  @param  bucketCollateral_ Amount of collateral in bucket.


/// [Audit Comment] param bucketCollateral param bucketLP param param quoteTokens param bucketPrice coresponding
117:         *  @param  bucketCollateral_ Amount of collateral in bucket.


/// [Audit Comment] param bucketCollateral param bucketLP param bucketDeposit param bucketPrice
139:         *  @param  bucketCollateral_ Amount of collateral in bucket.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Buckets.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Buckets.sol)


```solidity
File: ./ajna-core/src/libraries/internal/Deposits.sol

/// [Audit Comment] dev Fenwick
14:        @dev    Implemented as `Fenwick Tree` data structure.


/// [Audit Comment] dev Fenwick
18:        /// @dev Max index supported in the `Fenwick` tree


/// [Audit Comment] FenwickTree dev param param unscaledAddAmount unscaled
25:         *  @param  unscaledAddAmount_ The unscaled amount to increase deposit by.


/// [Audit Comment] Fenwick
32:            // price buckets are indexed starting at 0, Fenwick bit logic is more elegant starting at 1


/// [Audit Comment] unscaledAddAmount
35:            // unscaledAddAmount_ is the raw amount to add directly to the value at index_, unaffected by the scale array


/// [Audit Comment] unscaledAdd
36:            // For example, to denote an amount of deposit added to the array, we would need to call unscaledAdd with


/// [Audit Comment] unscaledAdd
38:            // 1- scale(index) is often already known in the context of where unscaledAdd(..) is called, and we want to avoid


/// [Audit Comment] Fenwick
39:            //    redundant iterations through the Fenwick tree.


/// [Audit Comment] unscaledRemove
41:            //    This is more relevant to unscaledRemove(...), where we need to ensure the value is precisely set to 0, but we


/// [Audit Comment] unscaledAddAmount propogate Fenwick
55:                // Update unscaledAddAmount to propogate up the Fenwick tree


/// [Audit Comment] addAmount
56:                // Note: we can't just multiply addAmount_ by scaling[i_] due to rounding


/// [Audit Comment] precice
57:                // We need to track the precice change in values[i_] in order to ensure


/// [Audit Comment] dev LUP param sumIndex prefixsum sumIndexSum sumIndex sumIndexScale sumIndex
73:         *  @return sumIndex_      Smallest index where prefixsum greater than the sum.


/// [Audit Comment] MSB LSB sumIndex
81:            // i iterates over bits from MSB to LSB.  We check at each stage if the target sum is to the left or right of sumIndex_+i


/// [Audit Comment] numBits
82:            uint256 i  = 4096; // 1 << (_numBits - 1) = 1 << (13 - 1) = 4096


/// [Audit Comment] sumIndex MSB LSB lowerIndexSum
85:            // We construct the target sumIndex_ bit by bit, from MSB to LSB.  lowerIndexSum_ always maintains the sum


/// [Audit Comment] sumIndex
86:            // up to the current value of sumIndex_


/// [Audit Comment] sumIndex
94:                // Consider if the target index is less than or greater than sumIndex_ + i


/// [Audit Comment] sumIndex
99:                // Compute sum up to sumIndex_ + i


/// [Audit Comment] sumIndex
105:                    // Target value is too small, need to consider increasing sumIndex_ still


/// [Audit Comment] sumIndex Fenwick
107:                        // sumIndex_+i is in range of Fenwick prices.  Target index has this bit set to 1.  


/// [Audit Comment] scaledValue sumIndexSum
116:                    // Current scaledValue is <= targetSum_, it's a candidate value for sumIndexSum_


/// [Audit Comment] findIndexAndSumOfSum dev LUP param sumIndex prefixsum
127:         *  @dev    Used in `LUP` calculation


/// [Audit Comment] LSB dev param LSB
141:         *  @param  i_  The integer with which to return the `LSB`.


/// [Audit Comment] dev param param
155:         *  @param  index_   The index to start scaling from.


/// [Audit Comment] Fenwick
163:            // price buckets are indexed starting at 0, Fenwick bit logic is more elegant starting at 1


/// [Audit Comment] LSB iteratively MSB
171:            // Starting with the LSB of index, we iteratively move up towards the MSB of SIZE


/// [Audit Comment] subtree
172:            // Case 1:     the bit of index_ is set to 1.  In this case, the entire subtree below index_


/// [Audit Comment] scaleing
173:            //             is scaled.  So, we include factor_ into scaleing[index_], and remember in sum how much


/// [Audit Comment] subtree
174:            //             we increased the subtree by, so that we can use it in case we encounter 0 bits (below).


/// [Audit Comment] subtree
175:            // Case 2:     The bit of index_ is set to 0.  In this case, consider the subtree below the node


/// [Audit Comment] subtree
176:            //             index_+bit. The subtree below that is not entirely scaled, but it does contain the


/// [Audit Comment] subtree
177:            //             subtree what was scaled earlier.  Therefore: we need to increment it's stored value


/// [Audit Comment] interation
178:            //             (in sum) which was set in a prior interation in case 1.


/// [Audit Comment] Calc
185:                    // Calc sum, will only be stored in range parents of starting node, index_


/// [Audit Comment] Fenwick
201:                    // Unset the bit in index to continue traversing up the Fenwick tree


/// [Audit Comment] superRangeIndex
204:                    // Case 2 above.  superRangeIndex is the index of the node to consider that


/// [Audit Comment] dev sumIndex param sumIndex param
223:         *  @param  sumIndex_  The index to receive the prefix sum.


/// [Audit Comment] Fenwick
230:            // price buckets are indexed starting at 0, Fenwick bit logic is more elegant starting at 1


/// [Audit Comment] Fenwick
233:            uint256 runningScale = Maths.WAD; // Tracks scale(index_) as we move down Fenwick tree


/// [Audit Comment] MSB LSB
234:            uint256 j            = SIZE;      // bit that iterates from MSB to LSB


/// [Audit Comment] sumIndex
235:            uint256 index        = 0;         // build up sumIndex bit by bit


/// [Audit Comment] sumIndex
237:            // Used to terminate loop.  We don't need to consider final 0 bits of sumIndex_


/// [Audit Comment] Fenwick
244:                // Skip considering indices outside bounds of Fenwick tree


/// [Audit Comment] runningScale
248:                // either to increment sum_ or to accumulate in runningScale


/// [Audit Comment] sumIndex
260:                    // terminate if we've already matched sumIndex_


/// [Audit Comment] signficant
266:                // shift j to consider next less signficant bit


/// [Audit Comment] FenwickTree dev param param unscaledRemoveAmount Unscaled
275:         *  @param  unscaledRemoveAmount_ Unscaled amount to decrease deposit by.


/// [Audit Comment] Fenwick
282:            // price buckets are indexed starting at 0, Fenwick bit logic is more elegant starting at 1


/// [Audit Comment] unscaledRemoveAmount
285:            // We operate with unscaledRemoveAmount_ here instead of a scaled quantity to avoid duplicate computation of scale factor


/// [Audit Comment] Fenwick
286:            // (thus redundant iterations through the Fenwick tree), and ALSO so that we can set the value of a given deposit exactly


/// [Audit Comment] removeAmount
290:                // Decrement deposits_ at index_ for removeAmount, storing new value in value


/// [Audit Comment] unscaledRemoveAmount
294:                // If scale factor != 1, we need to adjust unscaledRemoveAmount by scale factor to adjust values further up in tree


/// [Audit Comment] unscaledRemoveAmount wmul unscaledRemoveAmount
296:                // unscaledRemoveAmount_ = Maths.wmul(unscaledRemoveAmount, scaling).  This will introduce nonzero values up


/// [Audit Comment] propogate
298:                // and propogate that upwards.


/// [Audit Comment] Fenwick
301:                // Traverse upward through the "update" path of the Fenwick tree


/// [Audit Comment] dev param
308:         *  @dev    Starts at leaf/target and moved up towards root.


/// [Audit Comment] Fenwick
316:            // price buckets are indexed starting at 0, Fenwick bit logic is more elegant starting at 1


/// [Audit Comment] Fenwick
322:                // Traverse up through Fenwick tree via "update" path, accumulating scale factors as we go


/// [Audit Comment] Fenwick
336:            // In a scaled Fenwick tree, sum is at the root node and never scaled


/// [Audit Comment] param depositValue
342:         *  @param  index_        The deposit index.


/// [Audit Comment] unscaled
349:            // Get unscaled value at index and multiply by scale


/// [Audit Comment] Fenwick
357:            // In a scaled Fenwick tree, sum is at the root node, but needs to be scaled


/// [Audit Comment] unscaled unscaled
362:            // Returns the unscaled value at the node.  We consider the unscaled value for two reasons:


/// [Audit Comment] unscaled
363:            // 1- If we want to zero out deposit in bucket, we need to subtract the exact unscaled value


/// [Audit Comment] LUP param LUP LUP
384:         *  @param  debt_ The debt amount to calculate `LUP` for.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Deposits.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Deposits.sol)


```solidity
File: ./ajna-core/src/libraries/internal/Loans.sol

/// [Audit Comment] dev TP dev
21:        @dev    The `Loans` heap is a `Max Heap` data structure (complete binary tree), the root node is the loan with the highest threshold price (`TP`)


/// [Audit Comment] IPoolErrors
35:        // See `IPoolErrors` for descriptions


/// [Audit Comment] Initialization
39:        /***  Initialization ***/


/// [Audit Comment] Initializes dev Organizes param
44:         *  @dev    Organizes loans so `Highest Threshold Price` can be retrieved easily.


/// [Audit Comment] upsert TP dev dev upsert dev dev dev dev param param Struct param Struct param param borrowerAddress param poolDebt param poolRate param lup LUP param inAuction param NpUpdate
64:         *  @param auctions_        Struct for pool auctions state.


/// [Audit Comment] borrwer
102:                // if loan is in heap and borrwer is no longer active (no debt, no collateral) then remove loan from heap


/// [Audit Comment] param param param
130:         *  @param loans_ Holds loans heap data.


/// [Audit Comment] param param param
146:         *  @param loans_ Holds loans heap data.


/// [Audit Comment] param param param
175:         *  @param loans_ Holds loans heap data.


/// [Audit Comment] param param param
188:         *  @param loans_    Holds loans heap data.


/// [Audit Comment] existance param param param upserted param thresholdPrice
208:         *  @param index_          Index of `Loan` to be upserted.


/// [Audit Comment] Retreives param param
241:         *  @param loans_ Holds loans heap data.


/// [Audit Comment] Retreives param
250:         *  @notice Retreives `Loan` with the highest threshold price value.


/// [Audit Comment] param
260:         *  @param loans_ Holds loans heap data.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Loans.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/internal/Loans.sol)


```solidity
File: ./ajna-grants/src/grants/GrantFund.sol

/// [Audit Comment] inheritdoc IGrantFund
21:        /// @inheritdoc IGrantFund


/// [Audit Comment] proposalId proposalId FundingMechanism
32:         * @notice Given a proposalId, find if it is a standard or extraordinary proposal.


/// [Audit Comment] inheritdoc IGrantFund
44:        /// @inheritdoc IGrantFund


/// [Audit Comment] inheritdoc IGrantFund
57:        /// @inheritdoc IGrantFund


/// [Audit Comment] ajna
66:            // transfer ajna tokens to the treasury

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/GrantFund.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/GrantFund.sol)


```solidity
File: ./ajna-grants/src/grants/base/ExtraordinaryFunding.sol

/// [Audit Comment] Keccak
26:         * @notice Keccak hash of a prefix string for extraordinary funding mechanism


/// [Audit Comment] proposalId ExtraordinaryFundingProposal
36:         * @dev proposalId => ExtraordinaryFundingProposal.


/// [Audit Comment] proposalIds
41:         * @notice The list of extraordinary funding proposalIds that have been executed.


/// [Audit Comment] proposalId bool
47:         * @dev proposalId => address => bool.


/// [Audit Comment] inheritdoc IExtraordinaryFunding
55:        /// @inheritdoc IExtraordinaryFunding


/// [Audit Comment] uint uint
66:            // since we are casting from uint128 to uint256, we can safely assume that the value will not overflow


/// [Audit Comment] succesful
69:            // check proposal is succesful and hasn't already been executed


/// [Audit Comment] calldata
80:            // execute proposal's calldata


/// [Audit Comment] inheritdoc IExtraordinaryFunding
84:        /// @inheritdoc IExtraordinaryFunding


/// [Audit Comment] inheritdoc IExtraordinaryFunding
130:        /// @inheritdoc IExtraordinaryFunding


/// [Audit Comment] msg
134:            // revert if msg.sender already voted on proposal


/// [Audit Comment] proposalId
161:         * @param  proposalId_ The ID of the proposal being checked.


/// [Audit Comment] ProposalState GrantFund analytics compatability proposalId ProposalState
186:         * @dev    Used by GrantFund.state() for analytics compatability purposes.


/// [Audit Comment] ajna EFM
202:         * @notice Get the minimum percentage of ajna tokens required for a proposal to pass.


/// [Audit Comment] EFM
211:            // minimum threshold increases according to the number of funded EFM proposals


/// [Audit Comment] ajna
218:         * @notice Get the number of ajna tokens equivalent to a given percentage.


/// [Audit Comment] ajna
230:         * @notice Get the number of ajna tokens equivalent to a given percentage.


/// [Audit Comment] proposalId voteExtraordinary
243:         * @param  proposalId_     The ID of the proposal being voted on.


/// [Audit Comment] inheritdoc IExtraordinaryFunding
262:        /// @inheritdoc IExtraordinaryFunding


/// [Audit Comment] inheritdoc IExtraordinaryFunding
267:        /// @inheritdoc IExtraordinaryFunding


/// [Audit Comment] inheritdoc IExtraordinaryFunding
274:        /// @inheritdoc IExtraordinaryFunding


/// [Audit Comment] inheritdoc IExtraordinaryFunding
281:        /// @inheritdoc IExtraordinaryFunding


/// [Audit Comment] inheritdoc IExtraordinaryFunding
295:        /// @inheritdoc IExtraordinaryFunding


/// [Audit Comment] uint uint
297:            // since we are casting from uint128 to uint256, we can safely assume that the value will not overflow


/// [Audit Comment] inheritdoc IExtraordinaryFunding
303:        /// @inheritdoc IExtraordinaryFunding

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/ExtraordinaryFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/ExtraordinaryFunding.sol)


```solidity
File: ./ajna-grants/src/grants/base/Funding.sol

/// [Audit Comment] Immutables
17:        /*** Immutables ***/


/// [Audit Comment] ajna
20:        // address of the ajna token used in grant coordination


/// [Audit Comment] flashloan
29:         * @dev    Prevents flashloan attacks or duplicate voting with multiple accounts.


/// [Audit Comment] calldata calldata Ajna calldata Ajna calldatas calldatas
48:         * @param targets_   The list of smart contract targets for the calldata execution. Should be the Ajna token address.


/// [Audit Comment] voteStartBlock
73:         * @param voteStartBlock_ The block number the proposal became available for voting.


/// [Audit Comment] calldatas ETH calldatas calldata tokensRequested calldata
96:         * @notice Verifies proposal's targets, values, and calldatas match specifications.


/// [Audit Comment] params
109:            // check params have matching lengths


/// [Audit Comment] params
114:                // check targets and values params are valid


/// [Audit Comment] calldata
117:                // check calldata function selector is transfer()


/// [Audit Comment] tokensRequested calldata
128:                // retrieve tokensRequested from incoming calldata, accounting for selector and recipient address


/// [Audit Comment] calldata
136:                // update tokens requested for additional calldata


/// [Audit Comment] proposalId calldatas proposalId OpenZeppelin ETH calldatas calldata descriptionHash keccak proposalId proposalId params
144:         * @notice Create a proposalId from a hash of proposal's targets, values, and calldatas arrays, and a description hash.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/Funding.sol)


```solidity
File: ./ajna-grants/src/grants/base/StandardFunding.sol

/// [Audit Comment] challengephase updateSlate
30:         * @notice Length of the challengephase of the distribution period in blocks.


/// [Audit Comment] Keccak
49:         * @notice Keccak hash of a prefix string for standard funding mechanism


/// [Audit Comment] distributionId QuarterlyDistribution
67:         * @dev distributionId => QuarterlyDistribution


/// [Audit Comment] proposalId
73:         * @dev proposalId => Proposal


/// [Audit Comment] distributionId proposalIds proposalId
78:         * @dev Mapping of distributionId to a sorted array of 10 proposalIds with the most votes in the screening period.


/// [Audit Comment] proposalId
86:         * @dev slate hash => proposalId[]


/// [Audit Comment] distributionId QuadraticVoter
92:         * @dev distributionId => voter address => QuadraticVoter 


/// [Audit Comment] distributionId distributionId bool
98:         * @dev distributionId => bool


/// [Audit Comment] distributionId distributionId bool
104:         * @dev distributionId => address => bool


/// [Audit Comment] distributionId distributionId uint
110:         * @dev distributionId => address => uint256


/// [Audit Comment] inheritdoc IStandardFunding
118:        /// @inheritdoc IStandardFunding


/// [Audit Comment] currentDistributionId
145:            // set new value for currentDistributionId


/// [Audit Comment] QuarterlyDistribution
148:            // create QuarterlyDistribution struct


/// [Audit Comment] endBlock
172:         * @param  endBlock_ The end block of quarterly distribution to get the challenge stage end block for.


/// [Audit Comment] endBlock
183:         * @param  endBlock_ The end block of quarterly distribution to get the screening stage end block for.


/// [Audit Comment] distributionId
195:         * @param distributionId_ distribution Id of updating distribution 


/// [Audit Comment] readd
216:            // readd non distributed tokens to the treasury


/// [Audit Comment] DistributionPeriod newId
223:         * @notice Set a new DistributionPeriod Id.


/// [Audit Comment] inheritdoc IStandardFunding
235:        /// @inheritdoc IStandardFunding


/// [Audit Comment] delegatee
239:            // Revert if delegatee didn't vote in screening stage


/// [Audit Comment] delegatee
263:            // transfer rewards to delegatee


/// [Audit Comment] currentDistribution Struct calculat Struct
270:         * @param  currentDistribution_ Struct of the distribution period to calculat rewards for.


/// [Audit Comment] delegateeReward GBC delegatee
285:            // delegateeReward = 10 % of GBC distributed as per delegatee Voting power allocated


/// [Audit Comment] inheritdoc IStandardFunding
299:        /// @inheritdoc IStandardFunding


/// [Audit Comment] fundingVotesReceived
309:            // check the each proposal in the slate is valid, and get the sum of the proposals fundingVotesReceived


/// [Audit Comment] inheritdoc IStandardFunding
342:        /// @inheritdoc IStandardFunding


/// [Audit Comment] succesful
357:            // check proposal is succesful and hasn't already been executed


/// [Audit Comment] inheritdoc IStandardFunding
365:        /// @inheritdoc IStandardFunding


/// [Audit Comment] tokensRequested
388:            newProposal.tokensRequested = _validateCallDatas(targets_, values_, calldatas_); // check proposal parameters are valid and update tokensRequested


/// [Audit Comment] fundingVotesReceived distributionId endBlock distributionPeriodFundsAvailable proposalIds numProposalsInSlate
411:         * @notice Check the validity of a potential slate of proposals to execute, and sum the slate's fundingVotesReceived.


/// [Audit Comment] topTenProposals
437:                // check if Proposal is in the topTenProposals list


/// [Audit Comment] fundingVotesReceived
440:                // account for fundingVotesReceived possibly being negative


/// [Audit Comment] uint
444:                sum_ += uint128(proposal.fundingVotesReceived); // since we are converting from int128 to uint128, we can safely assume that the value will not overflow


/// [Audit Comment] GBC
447:                // check if slate of proposals exceeded budget constraint ( 90% of GBC )


/// [Audit Comment] proposalIds proposalIds
457:         * @notice Check an array of proposalIds for duplicate IDs.


/// [Audit Comment] proposalIdSubset
485:         * @param  proposalIdSubset_ Array of proposal Ids to sum.


/// [Audit Comment] uint
492:                // since we are converting from int128 to uint128, we can safely assume that the value will not overflow


/// [Audit Comment] ProposalState GrantFund analytics compatability proposalId ProposalState
501:         * @dev    Used by GrantFund.state() for analytics compatability purposes.


/// [Audit Comment] inheritdoc IStandardFunding
518:        /// @inheritdoc IStandardFunding


/// [Audit Comment] inheritdoc IStandardFunding
571:        /// @inheritdoc IStandardFunding


/// [Audit Comment] quadratically currentDistribution voteParams incrementalVotesUsed
604:         * @dev    Votes can be allocated to multiple proposals, quadratically, for or against.


/// [Audit Comment] votesCast
632:            // check that the voter hasn't already voted on a proposal by seeing if it's already in the votesCast array 


/// [Audit Comment] uint
637:                // since we are converting from int256 to uint256, we can safely assume that the value will not overflow


/// [Audit Comment] votesCast
643:                    // and the proposal is already in the votesCast array, revert can't change direction


/// [Audit Comment] votesCast
651:            // first time voting on this proposal, add the newly cast vote to the voter's votesCast array


/// [Audit Comment] uint
657:            // and check that attempted votes cast doesn't overflow uint128


/// [Audit Comment] uint uint
669:            // since we are moving from uint128 to uint256, we can safely assume that the value will not overflow


/// [Audit Comment] VoteCast VoteCastWithParams
681:            // emit VoteCast instead of VoteCastWithParams to maintain compatibility with Tally


/// [Audit Comment] VoteCast VoteCastWithParams
745:            // emit VoteCast instead of VoteCastWithParams to maintain compatibility with Tally


/// [Audit Comment] proposalIds proposalId proposalId proposalIds proposalId
756:         * @notice Identify where in an array of proposalIds the proposal exists.


/// [Audit Comment] proposalId
767:            index_ = -1; // default value indicating proposalId not in the array


/// [Audit Comment] FundingVoteParams proposalId proposalId voteParams FundingVoteParams proposalId
785:         * @param proposalId_ The proposalId to search for.


/// [Audit Comment] proposalId
793:            index_ = -1; // default value indicating proposalId not in the array


/// [Audit Comment] uint
795:            // since we are converting from uint256 to int256, we can safely assume that the value will not overflow


/// [Audit Comment] uint
812:         * @dev    Since we are converting from int256 to uint256, we can safely assume that the values will not overflow.


/// [Audit Comment] votesCast votesCastSumSquared
841:         * @return votesCastSumSquared_ The sum of the square of each vote cast.


/// [Audit Comment] proposalId proposalId
857:         * @param  proposalId_ The proposalId to check.


/// [Audit Comment] votingPower remainingVotingPower screeningStageEndBlock
886:         * @param  votingPower_            The voter's voting power in the funding round. Equal to the square of their tokens in the voting snapshot.


/// [Audit Comment] castVote
901:            // voter hasn't yet called _castVote in this period


/// [Audit Comment] inheritdoc IStandardFunding
916:        /// @inheritdoc IStandardFunding


/// [Audit Comment] inheritdoc IStandardFunding
927:        /// @inheritdoc IStandardFunding


/// [Audit Comment] inheritdoc IStandardFunding
932:        /// @inheritdoc IStandardFunding


/// [Audit Comment] inheritdoc IStandardFunding
946:        /// @inheritdoc IStandardFunding


/// [Audit Comment] inheritdoc IStandardFunding
953:        /// @inheritdoc IStandardFunding


/// [Audit Comment] inheritdoc IStandardFunding
960:        /// @inheritdoc IStandardFunding


/// [Audit Comment] inheritdoc IStandardFunding
965:        /// @inheritdoc IStandardFunding


/// [Audit Comment] inheritdoc IStandardFunding
979:        /// @inheritdoc IStandardFunding


/// [Audit Comment] inheritdoc IStandardFunding
986:        /// @inheritdoc IStandardFunding


/// [Audit Comment] inheritdoc IStandardFunding
993:        /// @inheritdoc IStandardFunding


/// [Audit Comment] inheritdoc IStandardFunding
1005:        /// @inheritdoc IStandardFunding


/// [Audit Comment] inheritdoc IStandardFunding
1018:        /// @inheritdoc IStandardFunding

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol)


```solidity
File: ./ajna-grants/src/grants/interfaces/IExtraordinaryFunding.sol

/// [Audit Comment] solc
3:    //slither-disable-next-line solc-version


/// [Audit Comment] Ajna
7:     * @title Ajna Grant Coordination Fund Extraordinary Proposal flow.


/// [Audit Comment] Structs
26:        /*** Structs ***/


/// [Audit Comment] ExtraordinaryFunding
30:         * @notice Contains information about proposals made to the ExtraordinaryFunding mechanism.


/// [Audit Comment] proposeExtraordinary
33:            uint256  proposalId;      // Unique ID of the proposal. Hashed from proposeExtraordinary inputs.


/// [Audit Comment] AJNA
36:            uint128  tokensRequested; // Number of AJNA tokens requested.


/// [Audit Comment] ETH calldatas calldata descriptionHash proposalId
49:         * @param calldatas_       The calldata to send to each target.


/// [Audit Comment] endBlock calldatas calldata proposalId
65:         * @param calldatas_           Array of calldata to execute in transactions.


/// [Audit Comment] proposalId votesCast
85:         * @param  proposalId_ The ID of the proposal being voted upon.


/// [Audit Comment] ajna
97:         * @notice Get the number of ajna tokens equivalent to a given percentage.


/// [Audit Comment] ajna
106:         * @notice Get the number of ajna tokens equivalent to a given percentage.


/// [Audit Comment] proposalIds ExtraordinaryFundingProposal param proposalId proposalId proposalId proposalId startBlock endBlock tokensRequested Ajna votesReceived Ajna succesful
115:         *  @notice Mapping of proposalIds to {ExtraordinaryFundingProposal} structs.


/// [Audit Comment] proposalId
130:         * @param  proposalId_ The ID of the proposal to check the status of.


/// [Audit Comment] Ajna
136:         * @notice Get the current minimum threshold percentage of Ajna tokens required for a proposal to exceed.


/// [Audit Comment] proposalId
145:         * @param  proposalId_ The ID of the proposal being voted on.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IExtraordinaryFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IExtraordinaryFunding.sol)


```solidity
File: ./ajna-grants/src/grants/interfaces/IFunding.sol

/// [Audit Comment] solc
3:    //slither-disable-next-line solc-version


/// [Audit Comment] Ajna
7:     * @title Ajna Grant Coordination Fund Extraordinary Proposal flow.


/// [Audit Comment] paramteres calldatas calldata
22:         * @dev    A proposal is invalid if it has a mismatch in the number of targets, values, or calldatas.


/// [Audit Comment] proposalId
33:         * @notice Provided proposalId isn't present in either funding mechanisms storage mappings.


/// [Audit Comment] execcution
38:         * @notice Proposal didn't meet requirements for execcution.


/// [Audit Comment] Structs
72:        /*** Structs ***/


/// [Audit Comment] Enum
76:         * @notice Enum listing available proposal types.


/// [Audit Comment] Enum lifecycle
84:         * @dev Enum listing a proposal's lifecycle.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IFunding.sol)


```solidity
File: ./ajna-grants/src/grants/interfaces/IGrantFund.sol

/// [Audit Comment] Ajna GrantFund param Ajna param treasuryBalance GrantFund's
22:         *  @param  treasuryBalance GrantFund's total treasury balance after the transfer.


/// [Audit Comment] proposalId calldatas proposalId OpenZeppelin ETH calldatas calldata descriptionHash keccak proposalId proposalId params
31:         * @notice Create a proposalId from a hash of proposal's targets, values, and calldatas arrays, and a description hash.


/// [Audit Comment] proposalId ProposalState
49:         * @param proposalId_ The id of the proposal to query the status of.


/// [Audit Comment] Ajna GrantFund fundingAmount Ajna
61:         * @notice Transfers Ajna tokens to the GrantFund contract.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IGrantFund.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IGrantFund.sol)


```solidity
File: ./ajna-grants/src/grants/interfaces/IStandardFunding.sol

/// [Audit Comment] solc
3:    //slither-disable-next-line solc-version


/// [Audit Comment] Ajna
7:     * @title Ajna Grant Coordination Fund Standard Proposal flow.


/// [Audit Comment] Delegatee
36:         * @notice Delegatee attempted to claim delegate reward before the challenge period ended.


/// [Audit Comment] proposalIds
41:         * @notice User provided a slate of proposalIds that is invalid.


/// [Audit Comment] Delegatee
46:         * @notice Delegatee attempted to claim delegate reward when not voted in screening.


/// [Audit Comment] optimized param distributionId param fundedSlateHash
71:         *  @param  distributionId  Id of the distribution period.


/// [Audit Comment] param distributionId param startBlock distrubtions param endBlock distrubtions
82:         *  @param  startBlock     Block number of the quarterly distrubtions start.


/// [Audit Comment] delegatee param delegateeAddress delegatee param distributionId param rewardClaimed
93:         *  @param  delegateeAddress Address of delegatee.


/// [Audit Comment] Structs
104:        /*** Structs ***/


/// [Audit Comment] proposalId proposeStandard
123:            uint256 proposalId;           // OZ.Governor compliant proposalId. Hash of proposeStandard inputs


/// [Audit Comment] Ajna
127:            uint128 tokensRequested;      // number of Ajna tokens requested in the proposal


/// [Audit Comment] QuadraticVoter
132:         * @notice Contains information about voters during a vote made by a QuadraticVoter in the Funding stage of a distribution period.


/// [Audit Comment] screeningVoteMulti
141:         * @dev    Used in screeningVoteMulti().


/// [Audit Comment] newDistributionId
164:         * @return newDistributionId_ The new distribution period Id.


/// [Audit Comment] delegatee distributionId whinch delegatee rewardClaimed delegatee
175:         * @param  distributionId_ Id of distribution from whinch delegatee wants to claim his reward.


/// [Audit Comment] succesfully calldata Ajna calldata calldatas calldata descriptionHash proposalId
190:         * @param  targets_         List of contracts the proposal calldata will interact with. Should be the Ajna token contract for all proposals.


/// [Audit Comment] calldata Ajna calldata calldatas calldata proposalId
206:         * @param  targets_     List of contracts the proposal calldata will interact with. Should be the Ajna token contract for all proposals.


/// [Audit Comment] maximizes QuarterlyDistribution proposalIds distributionId newTopSlate
220:         * @notice Check if a slate of proposals meets requirements, and maximizes votes. If so, update QuarterlyDistribution.


/// [Audit Comment] StandardFunding fundingVote voteParams votesCast
236:         * @dev    Calls out to StandardFunding._fundingVote().


/// [Audit Comment] StandardFunding screeningVote voteParams votesCast
248:         * @dev    Calls out to StandardFunding._screeningVote().


/// [Audit Comment] distributionId
263:         * @param  distributionId_ The  to calculate rewards for.


/// [Audit Comment] QuarterlyDistribution distributionId distributionId
273:         * @notice Retrieve the current QuarterlyDistribution distributionId.


/// [Audit Comment] distributionId QuarterlyDistribution distributionId distributionId QuarterlyDistribution distributionId distributionId startBlock endBlock fundsAvailable fundingVotePowerCast fundedSlateHash
279:         * @notice Mapping of distributionId to {QuarterlyDistribution} struct.


/// [Audit Comment] distributionId slateHash slateHash proposalIds
294:         * @param  slateHash_      The slateHash to retrieve the funded proposals from.


/// [Audit Comment] votingPower votingPower
305:         * @param  votingPower_ The provided voting power to calculate discrete votes for.


/// [Audit Comment] distributionId distributionId FundingVoteParams FundingVoteParams succesfully
316:         * @return FundingVoteParams The list of FundingVoteParams structs that have been succesfully cast the voter.


/// [Audit Comment] proposalIds proposalId proposalId proposalId proposalId distributionId distributionId votesReceived tokensRequested qvBudgetAllocated
321:         * @notice Mapping of proposalIds to {Proposal} structs.


/// [Audit Comment] proposalIds
336:         * @param  proposalIds_ Array of proposal Ids to hash.


/// [Audit Comment] distributionId distributionId topTenProposals proposalIds
347:         * @param  distributionId_ The distributionId of the distribution period to query.


/// [Audit Comment] distributionId distributionId votingPower remainingVotingPower votesCast
356:         * @param  distributionId_ The distributionId of the distribution period to check.


/// [Audit Comment] distributionId distributionId
370:         * @param  distributionId_ The distributionId of the distribution period to check.


/// [Audit Comment] distributionId distributionId
378:         * @param  distributionId_ The distributionId of the distribution period to check.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IStandardFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IStandardFunding.sol)


```solidity
File: ./ajna-grants/src/grants/libraries/Maths.sol

/// [Audit Comment] Utilizes babylonian
14:         * @dev Utilizes the babylonian method: https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/libraries/Maths.sol)


```solidity
File: ./ajna-grants/src/token/AjnaToken.sol

/// [Audit Comment] solc
3:    //slither-disable-next-line solc-version


/// [Audit Comment] AJNA seperate param param param param param unix param secp param secp param secp
51:         *  @param  v_        Component of secp256k1 signature

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/AjnaToken.sol)


```solidity
File: ./ajna-grants/src/token/BurnWrapper.sol

/// [Audit Comment] solc
3:    //slither-disable-next-line solc-version


/// [Audit Comment] Ethereum mainnet Ajna
17:         * @notice Ethereum mainnet address of the Ajna Token.


/// [Audit Comment] mainnet Ajna
27:         * @notice Only mainnet Ajna token can be wrapped.


/// [Audit Comment] Ajna
49:            // since the Ajna Token has 18 decimals, we can just return 18 here.

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/BurnWrapper.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/token/BurnWrapper.sol)



## [NC-9] Inadequate indexing of event fields
Indexed event fields enhance accessibility for off-chain tools parsing events, 
proving particularly beneficial for address-based filtering. However, gas costs increase with each 
indexed field during emission, posing a challenge in maximizing the use of the allowable three fields per event. 
Events with three or more fields should ideally utilize all three indexed fields, provided that gas usage is not a 
significant concern. In events with fewer than three fields, it's advisable to index all applicable fields, balancing 
quick accessibility and efficient gas consumption

*This issue was found 15 times:*

```solidity
File: ./ajna-core/src/interfaces/pool/IPoolFactory.sol

42:        event PoolCreated(address pool_);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/IPoolFactory.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/IPoolFactory.sol)


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

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolEvents.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/commons/IPoolEvents.sol)


```solidity
File: ./ajna-core/src/interfaces/rewards/IRewardsManagerEvents.sol

32:        event MoveStakedLiquidity(
33:            uint256 tokenId,
34:            uint256[] fromIndexes,
35:            uint256[] toIndexes
36:        );

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerEvents.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerEvents.sol)


```solidity
File: ./ajna-core/src/libraries/external/KickerActions.sol

87:        event KickReserveAuction(uint256 claimableReservesRemaining, uint256 auctionPrice, uint256 currentBurnEpoch);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol)


```solidity
File: ./ajna-core/src/libraries/external/LPActions.sol

28:        event TransferLP(address owner, address newOwner, uint256[] indexes, uint256 lp);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LPActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LPActions.sol)


```solidity
File: ./ajna-core/src/libraries/external/PoolCommons.sol

43:        event ResetInterestRate(uint256 oldRate, uint256 newRate);


44:        event UpdateInterestRate(uint256 oldRate, uint256 newRate);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol)


```solidity
File: ./ajna-core/src/libraries/external/TakerActions.sol

103:        event ReserveAuction(uint256 claimableReservesRemaining, uint256 auctionPrice, uint256 currentBurnEpoch);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/TakerActions.sol)


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

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IFunding.sol)


```solidity
File: ./ajna-grants/src/grants/interfaces/IGrantFund.sol

24:        event FundTreasury(uint256 amount, uint256 treasuryBalance);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IGrantFund.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IGrantFund.sol)



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



