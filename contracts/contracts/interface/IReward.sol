// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

interface IReward {
    function work(address prover, address player, address miner) external;
}
