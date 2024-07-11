// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

interface IController {
    function check(address account, address controller) external view returns(bool);
}
