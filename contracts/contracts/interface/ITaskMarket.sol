// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

enum TaskStatus {
    Waiting,
    Proving
}

interface ITaskMarket {
    function create(address game, address player, uint256 fee, bytes calldata data) external returns(uint256);
}
