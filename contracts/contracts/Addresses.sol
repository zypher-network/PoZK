// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IAddresses.sol";

contract Addresses is Initializable, OwnableUpgradeable, IAddresses {
    mapping(Contracts => address) private addresses;

    function initialize() public initializer {
        __Ownable_init(msg.sender);
    }

    function set(Contracts c, address _address) external onlyOwner {
        addresses[c] = _address;
    }

    function batchSet(Contracts[] calldata _cs, address[] calldata _addresses) external onlyOwner {
        require(_cs.length == _addresses.length, "A01");
        for (uint i = 0; i < _cs.length; i++) {
            addresses[_cs[i]] = _addresses[i];
        }
    }

    function get(Contracts c) external view returns (address) {
        return addresses[c];
    }
}
