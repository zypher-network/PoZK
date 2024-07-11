// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

enum GameStatus {
    Reviewing,
    Working,
    Upgrading,
    Stopped
}

interface IGameMarket {
    function totalWork() external view returns (uint256);

    function work(address game) external view returns (uint256);

    function version(address game) external view returns (uint256);

    function verifier(address game) external view returns (address);
}
