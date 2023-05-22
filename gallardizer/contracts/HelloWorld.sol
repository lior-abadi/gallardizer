// SPDX-License-Identifier: MIT
// compiler version must be greater than or equal to 0.8.17 and less than 0.9.0
pragma solidity 0.8.17;

contract HelloWorld {
    function test(uint256 a, uint256 b) external returns (uint256) {
        require(b != 0);
        return a / b;
    }

    function test2(uint256 c, uint256 d) external returns (uint256) {
        _check(d);
        return c / d;
    }

    function test3(uint256 f, uint256 g) external returns (uint256) {
        return f / g;
    }

    function _check(uint256 denomintator) internal {
        require(denomintator != 0);
    }

    // modifier nonReentrant() {
    //     _;
    // }

    // modifier someModifier() {
    //     _;
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
