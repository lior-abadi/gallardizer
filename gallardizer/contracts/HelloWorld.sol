// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

contract HelloWorld {
    event Testing(uint256 rojo, address azul, bytes32 amarillo, uint8 indexed verde);

    function _execute(
        uint256 proposalId_,
        address[] memory targets_,
        uint256[] memory values_,
        bytes[] memory calldatas_
    ) internal {
        // require(targets_.length == values_.length);
        // require(targets_.length == calldatas_.length);

        // assert(targets_.length == calldatas_.length);
        // assert(targets_.length == values_.length);

        if (targets_.length != values_.length) {
            revert ErrorArrayMismatch();
        }

        // if (targets_.length != calldatas_.length) {
        //     revert ErrorArrayMismatch();
        // }

        _checkArrayLengths(targets_.length, calldatas_.length);

        string memory errorMessage = "Governor: call reverted without message";
        for (uint256 i = 0; i < targets_.length; ++i) {
            (bool success, bytes memory returndata) = targets_[i].call{value: values_[i]}(calldatas_[i]);
            Address.verifyCallResult(success, returndata, errorMessage);
        }
    }

    // modifier nonReentrant() {
    //     _;
    // }

    // modifier someModifier() {
    //     _;
    // }

    // modifier onlyOwner() {
    //     _;
    // }

    // function test(uint256 a, uint256 b) external authorized returns (uint256) {
    //     revert();
    //     revert("Someting");
    //     require(a != b, "perro");
    //     require(a != b);

    //     manuelito(a != 0);
    //     return a / b;
    // }

    // function test2(uint256 c, uint256 d) external returns (uint256) {
    //     _check(d);
    //     return c / d;
    // }

    // function test3(uint256 f, uint256 g) external returns (uint256) {
    //     return f / g;
    // }

    // function _check(uint256 denomintator) internal {
    //     require(denomintator != 0);
    // }

    // function SetGreet(string memory _greet) public someModifier nonReentrant {
    //     greet = _greet;
    // }

    // function calculate(uint256 number) public returns (uint256) {
    //     uint256 num2 = number / 354;
    //     uint256 num3 = number / 2e18;
    //     _mint(a, b);

    //     require(something, "error");
    //     revert("error2");

    //     for (uint256 i = 0; i < something.length; i++) {
    //         somefunc.calculate();
    //         IERC20(contractAddr).tranf();
    //         _callIt();
    //     }
    //     for (uint256 i = 0; i < 5000; i++) {
    //         somefunc.calculate();
    //         IERC20(contractAddr).tranf();
    //         _callIt();
    //     }

    //     for (uint256 i = 0; i < memLength; i++) {
    //         somefunc.calculate();
    //         IERC20(contractAddr).tranf();
    //         _callIt();
    //     }

    //     return num2 / 1e18;
    // }

    // some comment **
    /**
     *
     *
     *
     */
    // function SetGreet2(string memory _greet, uint256 _greet1, uint256 _greet2, uint256 _greet3, uint256 _greet4)
    //     public
    //     nonReentrant
    //     someModifier
    // {
    //     require(_greet2 == _greet1);
    //     require(_greet3 == _greet1, "greet3 is not greet1");
    //     uint256 newNumb = _greet1 / _greet2;
    //     uint256 newNumb2 = _greet1 ** _greet2;

    //     greet = _greet;
    // }
}
