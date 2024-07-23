// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

interface IProver {
    /// prover name
    function name() external view returns (string memory);
}
