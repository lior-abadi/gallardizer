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

    function SetGreet2(string memory _greet) public nonReentrant someModifier {
        greet = _greet;
    }
}
