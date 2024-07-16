// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

interface IEpoch {
    function getAndUpdate() external returns (uint256);

    function get() external view returns (uint256);
}
