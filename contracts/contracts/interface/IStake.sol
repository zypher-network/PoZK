// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

interface IStake {
    /// get total prover staking
    function proverTotalStaking(address prover) external view returns (uint256);

    /// get prover staking by account
    function proverStaking(address prover, address account) external view returns (uint256);

    /// get total miner staking
    function minerTotalStaking(address prover) external view returns (uint256);

    /// get miner staking
    function minerStaking(address prover, address account) external view returns (uint256);

    // check account is miner or not
    function isMiner(address prover, address account) external view returns (bool);

    /// get total player staking
    function playerTotalStaking() external view returns (uint256);

    /// get player staking
    function playerStaking(address account) external view returns (uint256);

    /// get claimable unstaking amount
    function claimable(address account) external view returns (uint256);
}
