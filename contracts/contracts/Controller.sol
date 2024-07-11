// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IController.sol";

contract Controller is Initializable, OwnableUpgradeable, IController {
    address addresses;

    /// account => controllers
    mapping(address => mapping(address => bool)) controllers;

    event ChangeController(address account, address controller, bool isAdd);

    function initialize(address _addresses) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;
    }

    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    function check(address account, address controller) external view returns(bool) {
        return account == controller || controllers[account][controller];
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
