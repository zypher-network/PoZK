// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

interface IVerifier {
    function verify(bytes calldata publics, bytes calldata proof) external view returns (bool);

    /// show how to serialize/deseriaze the inputs params
    /// e.g. "uint256,bytes32,string,bytes32[],address[],ipfs"
    function inputs() external pure returns (string memory);

    /// show how to serialize/deserialize the publics params
    /// e.g. "uint256,bytes32,string,bytes32[],address[],ipfs"
    function publics() external pure returns (string memory);
}
