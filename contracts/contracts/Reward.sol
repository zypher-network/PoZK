// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import '@openzeppelin/contracts/token/ERC20/IERC20.sol';
import '@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol';

/**
 * @title Reward Contract
 */
contract Reward {
    using SafeERC20 for IERC20;

    /// @notice Game pool
    struct GamePool {
        uint256 totalWork;

        uint256 totalReward;
        uint256 unclaimReward;
        uint256 unclaimTotalLabor;

        mapping(address => uint256) minerLabor;
        mapping(address => uint256) gamerLabor;
    }

    struct RunningGame {
        uint unclaimGames;
        address[] games;
        mapping(address => uint) gameIndex;
    }

    struct EpochPool {
        uint256 totalUnclaimedGame;
        mapping(address => RunningGame) minerUnclaimedGames;
        mapping(address => RunningGame) gamerUnclaimedGames;
        mapping(address => GamePool) gamePools;
    }

    mapping(uint256 => EpochPool) private pools;

    /// @notice the numerator of Percentage of the game stake and labor (1-alpha) in the total
    int32 public alphaNumerator;

    /// @notice the denominator of the alpha
    int32 public alphaDenominator;

    /// @notice the numerator of Percentage of the miner stake and labor (1-beta) in the total
    int32 public betaNumerator;

    /// @notice the denominator of the beta
    int32 public betaDenominator;

    /// @notice the numerator of Percentage of the player stake and labor (1-beta) in the total
    int32 public gammaNumerator;

    /// @notice the denominator of the gamma
    int32 public gammaDenominator;

    /// @dev ### EVENTS
    /// @notice Emitted when update the alpha for cobb-douglas function
    event Alpha(int32 alphaNumerator, int32 alphaDenominator);

    /// @notice Emitted when update the alpha for cobb-douglas function
    event Beta(int32 betaNumerator, int32 betaDenominator);

    /// @notice Emitted when update the alpha for cobb-douglas function
    event Gamma(int32 betaNumerator, int32 betaDenominator);

    /// @notice Emitted when add Labor(reward) for current pool
    event MinerLabor(bytes32 game, address miner, uint256 work, uint256 total);

    /// @notice Emitted when add Labor(reward) for current pool
    event PlayerLabor(bytes32 game, address player, uint256 play, uint256 total);

    /// @notice Emitted when collect reward (stake) from pool
    event MinerCollect(uint256 epoch, bytes32 game, address miner, uint256 amount);

    /// @notice Emitted when collect reward (stake) from pool
    event PlayerCollect(uint256 epoch, bytes32 game, address player, uint256 amount);

    /**
     * @notice Update the alpha for cobb-douglas function
     * @param _alphaNumerator the numerator of the alpha
     * @param _alphaDenominator the denominator of the alpha
     */
    function setAlpha(int32 _alphaNumerator, int32 _alphaDenominator) public onlyOwner {
        require(_alphaNumerator > 0 && _alphaDenominator > 0, 'R01');
        alphaNumerator = _alphaNumerator;
        alphaDenominator = _alphaDenominator;

        emit Alpha(alphaNumerator, alphaDenominator);
    }
}
