// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title Stake Contract
 */
contract Stake is Ownable {
    using SafeERC20 for IERC20;

    struct Staking {
        uint256 value;
        uint256 newValue;
        uint256 newEpoch;
    }

    mapping(address => mapping(address =>uint256)) private gameStakers;
    mapping(address => Staking) private gamesStaking;
    mapping(address => Staking) private minersStaking;
    mapping(address => Staking) private playersStaking;

    function gameStaking(address game) external view returns (uint256) {
        uint256 currentEpoch = 0; // TODO
        Staking storage st = gamesStaking[game];

        if (currentEpoch >= st.newEpoch) {
            return st.newValue;
        } else {
            return st.value;
        }
    }

    function stakeGame(address game, uint256 amount) external {
        gameStakers[msg.sender][game] += amount;
        Staking storage st = gamesStaking[game];

        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue += amount;
    }

    function unstakeGame(address game, uint256 amount) external {
        gameStakers[msg.sender][game] -= amount;
        Staking storage st = gamesStaking[game];

        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue -= amount;
    }
}
