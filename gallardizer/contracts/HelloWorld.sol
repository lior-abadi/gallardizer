// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

contract HelloWorld {
    function test3(uint256 f, uint256 g) external returns (uint256) {
        uint256 x = f / 2;
        uint256 y = g / 2;

        return f / g;
    }
}
