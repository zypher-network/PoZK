// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

/// Network mode for new miner and prover
enum NetworkMode {
    /// only allowlist accounts
    Allowlist,
    /// register and certification
    Permissioned,
    /// permissionless for all
    Permissionless
}

interface IEpoch {
    function getAndUpdate() external returns (uint256);

    function get() external view returns (uint256);

    function networkMode() external view returns (NetworkMode);

    function isDao(address account) external view returns (bool);
}
