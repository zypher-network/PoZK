// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

enum Contracts {
    Token,
    Vesting,
    Epoch,
    Stake,
    Reward,
    GameMarket,
    TaskMarket,
    Controller
}

interface IAddresses {
    function get(Contracts c) external view returns(address);
}
