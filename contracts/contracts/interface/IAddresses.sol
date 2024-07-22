// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

enum Contracts {
    Token,
    Vesting,
    Epoch,
    Stake,
    Reward,
    ProverMarket,
    TaskMarket,
    Controller
}

interface IAddresses {
    function get(Contracts c) external view returns(address);
}
