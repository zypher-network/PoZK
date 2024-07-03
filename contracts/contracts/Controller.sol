// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";

contract Controller is Ownable {
    /// account => controllers
    mapping(address => mapping(address => bool)) controllers;

    event ChangeController(address account, address controller, bool isAdd);

    function check(address account, address controller) external view returns(bool) {
        require(account == controller || controllers[account][controller], "C01");
    }

    function add(address controller) external {
        controllers[msg.sender][controller] = true;

        emit ChangeController(msg.sender, controller, true);
    }

    function remove(address controller) external {
        delete controllers[msg.sender][controller];

        emit ChangeController(msg.sender, controller, false);
    }
}
