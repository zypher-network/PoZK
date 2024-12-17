// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

enum ProverStatus {
    Reviewing,
    Working,
    Upgrading,
    Stopped
}

enum ProverType {
    ZK,
    ZK_VM,
    Z4,
    AI_MODEL,
    AI_AGENT
}

interface IProver {
    function isProver(address prover) external view returns (bool);

    function totalWork() external view returns (uint256);

    function work(address prover) external view returns (uint256);

    function version(address prover) external view returns (uint256);

    function overtime(address prover) external view returns (uint256);

    function verifier(address prover) external view returns (address);

    function owner(address prover) external view returns (address);

    function checkUrl(address prover, string memory url) external view returns (bool);
}
