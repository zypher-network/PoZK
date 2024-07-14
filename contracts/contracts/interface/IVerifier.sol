// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

interface IVerifier {
    function verify(bytes calldata publics, bytes calldata proof) external view returns (bool);
}
