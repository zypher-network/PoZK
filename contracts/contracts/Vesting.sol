// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";

contract Vesting is Ownable {

    uint256 minersTotal;
    mapping(address => uint256) miners;


    function mine(uint256 epoch) external view returns(uint256) {
        // TODO
        return 1000;
    }

    function setMinerAmount(address[] miners, uint256[] amounts) external onlyOwner {
        for (int i = 0; i < miners.length; i++) {
            miners[miners[i]] += amounts[i];
        }
    }

    function minersTotal() external view returns(uint256) {
        return minersTotal;
    }

    function miner(address account) external view returns(uint256) {
        return miners[account];
    }
}
