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
| [NC-6] | Inadequate indexing of event fields | 65 |


Total: 140 instances over 6 issues

## Gas Optimizations
| |Issue|Instances|Total Gas Saved|
|-|:-|:-:|:-:|
| [G-1] | Adopt custom errors over `revert()/require()` strings | 7 | 350 |


Total: 7 instances over 1 issue, saving over 350 gas units

## Overall Results
**Total: 174 instances over 14 issues, potentially saving over 350 gas units**

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
utilize predefined units like seconds, minutes, hours, days, or weeks..

*This issue was found 3 times:*

```solidity
File: ./ajna-grants/src/grants/base/StandardFunding.sol

34:        uint256 internal constant CHALLENGE_PERIOD_LENGTH = 50400;


40:        uint48 internal constant DISTRIBUTION_PERIOD_LENGTH = 648000;


46:        uint256 internal constant FUNDING_PERIOD_LENGTH = 72000;

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/base/StandardFunding.sol)



## [NC-6] Inadequate indexing of event fields
Indexed event fields enhance accessibility for off-chain tools parsing events, 
proving particularly beneficial for address-based filtering. However, gas costs increase with each 
indexed field during emission, posing a challenge in maximizing the use of the allowable three fields per event. 
Events with three or more fields should ideally utilize all three indexed fields, provided that gas usage is not a 
significant concern. In events with fewer than three fields, it's advisable to index all applicable fields, balancing 
quick accessibility and efficient gas consumption

*This issue was found 65 times:*

```solidity
File: ./ajna-core/src/interfaces/pool/IPoolFactory.sol

42:        event PoolCreated(address pool_);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/IPoolFactory.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/IPoolFactory.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/commons/IPoolEvents.sol

22:        event AddQuoteToken(
23:            address indexed lender,
24:            uint256 indexed index,
25:            uint256 amount,
26:            uint256 lpAwarded,
27:            uint256 lup
28:        );


58:        event RemoveQuoteToken(
59:            address indexed lender,
60:            uint256 indexed index,
61:            uint256 amount,
62:            uint256 lpRedeemed,
63:            uint256 lup
64:        );


73:        event RemoveCollateral(
74:            address indexed claimer,
75:            uint256 indexed index,
76:            uint256 amount,
77:            uint256 lpRedeemed
78:        );


91:        event RepayDebt(
92:            address indexed borrower,
93:            uint256 quoteRepaid,
94:            uint256 collateralPulled,
95:            uint256 lup
96:        );


109:        event Kick(
110:            address indexed borrower,
111:            uint256 debt,
112:            uint256 collateral,
113:            uint256 bond
114:        );


122:        event BondWithdrawn(
123:            address indexed kicker,
124:            address indexed reciever,
125:            uint256 amount
126:        );


138:        event BucketTake(
139:            address indexed borrower,
140:            uint256 index,
141:            uint256 amount,
142:            uint256 collateral,
143:            uint256 bondChange,
144:            bool    isReward
145:        );


154:        event BucketTakeLPAwarded(
155:            address indexed taker,
156:            address indexed kicker,
157:            uint256 lpAwardedTaker,
158:            uint256 lpAwardedKicker
159:        );


170:        event Take(
171:            address indexed borrower,
172:            uint256 amount,
173:            uint256 collateral,
174:            uint256 bondChange,
175:            bool    isReward
176:        );


184:        event Settle(
185:            address indexed borrower,
186:            uint256 settledDebt
187:        );


194:        event AuctionSettle(
195:            address indexed borrower,
196:            uint256 collateral
197:        );


206:        event AuctionNFTSettle(
207:            address indexed borrower,
208:            uint256 collateral,
209:            uint256 lp,
210:            uint256 index
211:        );


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


248:        event IncreaseLPAllowance(
249:            address indexed owner,
250:            address indexed spender,
251:            uint256[] indexes,
252:            uint256[] amounts
253:        );


262:        event DecreaseLPAllowance(
263:            address indexed owner,
264:            address indexed spender,
265:            uint256[] indexes,
266:            uint256[] amounts
267:        );


275:        event RevokeLPAllowance(
276:            address indexed owner,
277:            address indexed spender,
278:            uint256[] indexes
279:        );


286:        event ApproveLPTransferors(
287:            address indexed lender,
288:            address[] transferors
289:        );


296:        event RevokeLPTransferors(
297:            address indexed lender,
298:            address[] transferors
299:        );


309:        event TransferLP(
310:            address owner,
311:            address newOwner,
312:            uint256[] indexes,
313:            uint256 lp
314:        );


325:        event BucketBankruptcy(
326:            uint256 indexed index,
327:            uint256 lpForfeited
328:        );


336:        event Flashloan(
337:            address indexed receiver,
338:            address indexed token,
339:            uint256 amount
340:        );


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
File: ./ajna-core/src/interfaces/pool/erc20/IERC20PoolEvents.sol

17:        event AddCollateral(
18:            address indexed actor,
19:            uint256 indexed index,
20:            uint256 amount,
21:            uint256 lpAwarded
22:        );


31:        event DrawDebt(
32:            address indexed borrower,
33:            uint256 amountBorrowed,
34:            uint256 collateralPledged,
35:            uint256 lup
36:        );

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolEvents.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc20/IERC20PoolEvents.sol)


```solidity
File: ./ajna-core/src/interfaces/pool/erc721/IERC721PoolEvents.sol

17:        event AddCollateralNFT(
18:            address indexed actor,
19:            uint256 indexed index,
20:            uint256[] tokenIds,
21:            uint256   lpAwarded
22:        );


30:        event MergeOrRemoveCollateralNFT(
31:            address indexed actor,
32:            uint256 collateralMerged,
33:            uint256 toIndexLps
34:        );


43:        event DrawDebtNFT(
44:            address indexed borrower,
45:            uint256   amountBorrowed,
46:            uint256[] tokenIdsPledged,
47:            uint256   lup
48:        );

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolEvents.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/pool/erc721/IERC721PoolEvents.sol)


```solidity
File: ./ajna-core/src/interfaces/position/IPositionManagerEvents.sol

25:        event MemorializePosition(
26:            address indexed lender,
27:            uint256 tokenId,
28:            uint256[] indexes
29:        );


37:        event Mint(
38:            address indexed lender,
39:            address indexed pool,
40:            uint256 tokenId
41:        );


52:        event MoveLiquidity(
53:            address indexed lender,
54:            uint256 tokenId,
55:            uint256 fromIndex,
56:            uint256 toIndex,
57:            uint256 lpRedeemedFrom,
58:            uint256 lpAwardedTo
59:        );


66:        event RedeemPosition(
67:            address indexed lender,
68:            uint256 tokenId,
69:            uint256[] indexes
70:        );

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerEvents.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/position/IPositionManagerEvents.sol)


```solidity
File: ./ajna-core/src/interfaces/rewards/IRewardsManagerEvents.sol

32:        event MoveStakedLiquidity(
33:            uint256 tokenId,
34:            uint256[] fromIndexes,
35:            uint256[] toIndexes
36:        );


57:        event UpdateExchangeRates(
58:            address indexed caller,
59:            address indexed ajnaPool,
60:            uint256[] indexesUpdated,
61:            uint256 rewardsClaimed
62:        );

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerEvents.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/interfaces/rewards/IRewardsManagerEvents.sol)


```solidity
File: ./ajna-core/src/libraries/external/KickerActions.sol

85:        event Kick(address indexed borrower, uint256 debt, uint256 collateral, uint256 bond);


86:        event RemoveQuoteToken(address indexed lender, uint256 indexed price, uint256 amount, uint256 lpRedeemed, uint256 lup);


87:        event KickReserveAuction(uint256 claimableReservesRemaining, uint256 auctionPrice, uint256 currentBurnEpoch);


88:        event BucketBankruptcy(uint256 indexed index, uint256 lpForfeited);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/KickerActions.sol)


```solidity
File: ./ajna-core/src/libraries/external/LPActions.sol

23:        event ApproveLPTransferors(address indexed lender, address[] transferors);


24:        event RevokeLPTransferors(address indexed lender, address[] transferors);


25:        event IncreaseLPAllowance(address indexed owner, address indexed spender, uint256[] indexes, uint256[] amounts);


26:        event DecreaseLPAllowance(address indexed owner, address indexed spender, uint256[] indexes, uint256[] amounts);


27:        event RevokeLPAllowance(address indexed owner, address indexed spender, uint256[] indexes);


28:        event TransferLP(address owner, address newOwner, uint256[] indexes, uint256 lp);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LPActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LPActions.sol)


```solidity
File: ./ajna-core/src/libraries/external/LenderActions.sol

70:        event AddQuoteToken(address indexed lender, uint256 indexed index, uint256 amount, uint256 lpAwarded, uint256 lup);


71:        event BucketBankruptcy(uint256 indexed index, uint256 lpForfeited);


73:        event RemoveQuoteToken(address indexed lender, uint256 indexed index, uint256 amount, uint256 lpRedeemed, uint256 lup);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LenderActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/LenderActions.sol)


```solidity
File: ./ajna-core/src/libraries/external/PoolCommons.sol

43:        event ResetInterestRate(uint256 oldRate, uint256 newRate);


44:        event UpdateInterestRate(uint256 oldRate, uint256 newRate);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/PoolCommons.sol)


```solidity
File: ./ajna-core/src/libraries/external/SettlerActions.sol

67:        event AuctionSettle(address indexed borrower, uint256 collateral);


68:        event AuctionNFTSettle(address indexed borrower, uint256 collateral, uint256 lp, uint256 index);


69:        event BucketBankruptcy(uint256 indexed index, uint256 lpForfeited);


70:        event Settle(address indexed borrower, uint256 settledDebt);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/SettlerActions.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/libraries/external/SettlerActions.sol)


```solidity
File: ./ajna-core/src/libraries/external/TakerActions.sol

100:        event BucketTake(address indexed borrower, uint256 index, uint256 amount, uint256 collateral, uint256 bondChange, bool isReward);


101:        event BucketTakeLPAwarded(address indexed taker, address indexed kicker, uint256 lpAwardedTaker, uint256 lpAwardedKicker);


102:        event Take(address indexed borrower, uint256 amount, uint256 collateral, uint256 bondChange, bool isReward);


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


69:        event VoteCast(address indexed voter, uint256 proposalId, uint8 support, uint256 weight, string reason);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IFunding.sol)


```solidity
File: ./ajna-grants/src/grants/interfaces/IGrantFund.sol

24:        event FundTreasury(uint256 amount, uint256 treasuryBalance);

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IGrantFund.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IGrantFund.sol)


```solidity
File: ./ajna-grants/src/grants/interfaces/IStandardFunding.sol

85:        event QuarterlyDistributionStarted(
86:            uint256 indexed distributionId,
87:            uint256 startBlock,
88:            uint256 endBlock
89:        );


97:        event DelegateRewardClaimed(
98:            address indexed delegateeAddress,
99:            uint256 indexed distributionId,
100:            uint256 rewardClaimed
101:        );

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IStandardFunding.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-grants/src/grants/interfaces/IStandardFunding.sol)



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



