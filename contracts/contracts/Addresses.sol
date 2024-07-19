// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IAddresses.sol";

/// @notice Store all contracts address and can update them
contract Addresses is Initializable, OwnableUpgradeable, IAddresses {
    /// @notice All contracts address key by Contracts enum
    mapping(Contracts => address) private addresses;

    /// @notice Initialize
    function initialize() public initializer {
        __Ownable_init(msg.sender);
    }

    /// @notice Owner can update some contract address
    /// @param c the Contract enum
    /// @param _address the contract new address
    function set(Contracts c, address _address) external onlyOwner {
        addresses[c] = _address;
    }

    /// @notice Owner can batch update contracts' address
    /// @param _cs the Contracts list
    /// @param _addresses the contracts new addresses
    function batchSet(Contracts[] calldata _cs, address[] calldata _addresses) external onlyOwner {
        require(_cs.length == _addresses.length, "A01");
        for (uint i = 0; i < _cs.length; i++) {
            addresses[_cs[i]] = _addresses[i];
        }
    }

    /// @notice Get contract address
    /// @param c the Contract enum
    /// @return the contract address
    function get(Contracts c) external view returns (address) {
        return addresses[c];
    }
}
