# Issues Summary
## Low Risk Issues
| |Issue|Instances|
|-|:-|:-:|
| [L-1] | Insecure declaration of pragma version | 2 |


Total: 2 instances over 1 issue

## Non-Critical Issues
| |Issue|Instances|
|-|:-|:-:|
| [NC-1] | The <code>nonReentrant</code> modifier should precede all other modifiers | 31 |


Total: 31 instances over 1 issue

# Low Risk Issues
## [L-1] Insecure declaration of pragma version
The specified <code>pragma</code> version allows for the utilization of different compiler versions to compile the source code.
It's important to consider the potential risks associated with using a floating or flexible pragma version. 
For instance, employing versions <code>0.8.7</code> or earlier may result in compilation errors, as they lack support for 
functions overriding interface functions without using the <code>override</code> modifier, 
which is exclusively available in Solidity <code>0.8.8</code> and newer versions.<br> 

Similarly, the usage of <code>abi.encodeCall</code>, which was introduced in Solidity <code>0.8.11</code>, 
may cause issues if the codebase relies on it. Although it is uncertain whether these specific bugs related to <code>override</code> 
or <code>encode</code> will manifest in the code, exercising caution is advised to avoid potential unexpected scenarios or compatibility
issues that may arise with the inclusion of new features or implementations.
Considering the uncertainty of potential bugs related to <code>override</code>, <code>encode</code>, or others, using a floating (flexible)
<code>pragma</code> version might lead to the project compiling with uncertain versions within that range.<br>

Consider upgrading the pragma version to a newer release, preferably the most recent version available, 
in order to mitigate potential risks stemming from bug fixes introduced in previous releases. 
Additionally, it is recommended to make the pragma version fixed to ensure consistency and stability in the project.

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

*This issue was found 31 times:*

```solidity
File: ./ajna-core/src/ERC20Pool.sol

131:        function drawDebt(


205:        function repayDebt(


281:        function addCollateral(


314:        function removeCollateral(


354:        function settle(


385:        function take(


434:        function bucketTake(

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC20Pool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC20Pool.sol)


```solidity
File: ./ajna-core/src/ERC721Pool.sol

139:        function drawDebt(


214:        function repayDebt(


288:        function addCollateral(


319:        function mergeOrRemoveCollateral(


360:        function removeCollateral(


432:        function take(


487:        function bucketTake(

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/ERC721Pool.sol)


```solidity
File: ./ajna-core/src/PositionManager.sol

227:        function mint(


262:        function moveLiquidity(

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/PositionManager.sol)


```solidity
File: ./ajna-core/src/base/FlashloanablePool.sol

28:        function flashLoan(

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/FlashloanablePool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/FlashloanablePool.sol)


```solidity
File: ./ajna-core/src/base/Pool.sol

170:        function addQuoteToken(uint256 amount_, uint256 index_, uint256 expiry_)


194:        function moveQuoteToken(uint256 maxAmount_, uint256 fromIndex_, uint256 toIndex_, uint256 expiry_)


223:        function removeQuoteToken(uint256 maxAmount_, uint256 index_)


251:        function updateInterest() external override nonReentrant {


267:        function stampLoan() external override nonReentrant {


291:        function kick(address borrower_, uint256 npLimitIndex_) external override nonReentrant {


325:        function kickWithDeposit(uint256 index_, uint256 npLimitIndex_) external override nonReentrant {


362:        function withdrawBonds(address recipient_, uint256 maxAmount_) external override nonReentrant {


401:        function kickReserveAuction() external override nonReentrant {


424:        function takeReserves(uint256 maxAmount_) external override nonReentrant returns (uint256 amount_) {


448:        function increaseLPAllowance(address spender_, uint256[] calldata indexes_, uint256[] calldata amounts_)


457:        function decreaseLPAllowance(address spender_, uint256[] calldata indexes_, uint256[] calldata amounts_)


466:        function revokeLPAllowance(address spender_, uint256[] calldata indexes_) external override nonReentrant {


481:        function transferLP(address owner_, address newOwner_, uint256[] calldata indexes_)

```

**Location link:** [https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/Pool.sol](https://github.com/code-423n4/2023-05-ajna/blob/main/ajna-core/src/base/Pool.sol)



