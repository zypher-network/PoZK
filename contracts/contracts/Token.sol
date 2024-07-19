// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/// @notice the main Token(ERC20) for zypher network
contract Token is ERC20 {
    constructor(uint256 initialSupply) ERC20("Zypher Network Coin", "ZNC") {
        _mint(msg.sender, initialSupply);
    }
}
