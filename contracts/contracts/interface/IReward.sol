// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

interface IReward {
    function work(address game, address player, address miner) external;
}
