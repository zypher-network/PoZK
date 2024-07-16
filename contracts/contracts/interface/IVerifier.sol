// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

interface IVerifier {
    function verify(bytes calldata publics, bytes calldata proof) external view returns (bool);
}
