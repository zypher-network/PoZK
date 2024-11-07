// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

enum ProverStatus {
    Reviewing,
    Working,
    Upgrading,
    Stopped
}

interface IProver {
    function isProver(address prover) external view returns (bool);

    function totalWork() external view returns (uint256);

    function work(address prover) external view returns (uint256);

    function version(address prover) external view returns (uint256);

    function overtime(address prover) external view returns (uint256);

    function verifier(address prover) external view returns (address);

    function owner(address prover) external view returns (address);
}
