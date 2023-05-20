// SPDX-License-Identifier: MIT
// compiler version must be greater than or equal to 0.8.17 and less than 0.9.0
pragma solidity 0.8.17;

contract HelloWorld {
    string public greet = "Hello World!";

    modifier nonReentrant() {
        _;
    }

    modifier someModifier() {
        _;
    }

    function SetGreet(string memory _greet) public someModifier nonReentrant {
        greet = _greet;
    }

    // some comment **
    /**
     *
     *
     *
     */
    function SetGreet2(string memory _greet, uint256 _greet1, uint256 _greet2, uint256 _greet3, uint256 _greet4)
        public
        nonReentrant
        someModifier
    {
        require(_greet2 == _greet1);
        require(_greet3 == _greet1, "greet3 is not greet1");
        uint256 newNumb = _greet1 / _greet2;
        uint256 newNumb2 = _greet1 ** _greet2;

        greet = _greet;
    }
}
