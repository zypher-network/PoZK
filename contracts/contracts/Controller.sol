// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IController.sol";

/// @notice User can set multiple controllers to help them with some functions,
contract Controller is Initializable, OwnableUpgradeable, IController {
    /// @notice Common Addresses contract
    address addresses;

    /// @notice Store all controllers by account
    mapping(address => mapping(address => bool)) controllers;

    /// @notice Emit when controller changed, isAdd if true is add, if false is remove
    event ChangeController(address account, address controller, bool isAdd);

    /// @notice Initialize
    /// @param _addresses the common Addresses contract
    function initialize(address _addresses) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;
    }

    /// @notice Set common Addresses contract
    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    /// @notice Check if controller belongs to account or account self
    /// @param account the account address
    /// @param controller the controller address
    /// @return success or failure
    function check(address account, address controller) external view returns(bool) {
        return account == controller || controllers[account][controller];
    }

    /// @notice Add new controller to account
    /// @param controller the controller address
    function add(address controller) external {
        controllers[msg.sender][controller] = true;

        emit ChangeController(msg.sender, controller, true);
    }

    /// @notice Remove a controller from account
    /// @param controller the controller address
    function remove(address controller) external {
        delete controllers[msg.sender][controller];

        emit ChangeController(msg.sender, controller, false);
    }
}
