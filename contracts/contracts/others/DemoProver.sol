// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/introspection/ERC165.sol";

import "../interface/IVerifier.sol";

contract DemoProver is ERC165, IVerifier {
    function supportsInterface(bytes4 interfaceId) public view virtual override(ERC165) returns (bool) {
        return interfaceId == type(IVerifier).interfaceId || super.supportsInterface(interfaceId);
    }

    function name() external view returns (string memory) {
        return "Demo-Prover";
    }

    function permission() external view returns (bool) {
        return false;
    }

    function verify(bytes calldata _publics, bytes calldata _proof) external view returns (bool) {
        return true;
    }

    function inputs() external pure returns (string memory) {
        return "uint256";
    }

    function publics() external pure returns (string memory) {
        return "uint256";
    }
}
