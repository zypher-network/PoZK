// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

interface IStake {
    /// get total game staking
    function gameTotalStaking(address game) external view returns (uint256);

    /// get game staking by account
    function gameStaking(address game, address account) external view returns (uint256);

    /// get total miner staking
    function minerTotalStaking(address game) external view returns (uint256);

    /// get miner staking
    function minerStaking(address game, address account) external view returns (uint256);

    // check account is miner or not
    function isMiner(address game, address account) external view returns (bool);

    /// get total player staking
    function playerTotalStaking() external view returns (uint256);

    /// get player staking
    function playerStaking(address account) external view returns (uint256);

    /// get claimable unstaking amount
    function claimable(address account) external view returns (uint256);
}
