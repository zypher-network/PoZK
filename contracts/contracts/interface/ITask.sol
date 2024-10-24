// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

enum TaskStatus {
    Over,
    Waiting,
    Proving,
    Disputing
}

interface ITask {
    function create(address prover, address player, uint256 fee, bytes calldata data, bytes calldata publics) external returns(uint256);
}
