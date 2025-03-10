// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

interface IController {
    function check(address account, address controller) external view returns(bool);
}
