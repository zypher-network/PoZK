// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";

contract Vesting is Ownable {

    uint256 private _minersTotal;
    mapping(address => uint256) miners;

    constructor() Ownable(msg.sender) {}

    function mine(uint256 epoch) external view returns(uint256) {
        // TODO
        return 1000;
    }

    function setMinerAmount(address[] calldata _miners, uint256[] calldata amounts) external onlyOwner {
        for (uint i = 0; i < _miners.length; i++) {
            miners[_miners[i]] += amounts[i];
        }
    }

    function minersTotal() external view returns(uint256) {
        return _minersTotal;
    }

    function miner(address account) external view returns(uint256) {
        return miners[account];
    }
}
