// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

import "./Epoch.sol";
import "./Stake.sol";
import "./GameMarket.sol";
import "./Vesting.sol";
import "./utils/FixedMath.sol";

/**
 * @title Reward Contract
 */
contract Reward is Ownable {
    using SafeERC20 for IERC20;

    struct StakerLabor {
        uint256 staking;
        uint256 working;
    }

    /// @notice Game pool
    struct GamePool {
        /// miner + player work (double)
        uint256 totalWorking;

        /// game total staking
        uint256 totalStaking;

        uint256 totalMinerStaking;
        uint256 totalPlayerStaking;

        uint256 totalMinerReward;
        uint256 totalPlayerReward;

        uint256 unclaimReward;
        uint256 unclaimLabor;

        mapping(address => StakerLabor) minerLabor;
        mapping(address => StakerLabor) playerLabor;
    }

    struct RunningGame {
        uint unclaim;
        // first is 0 for better use
        address[] games;
        mapping(address => uint) index;
    }

    struct EpochPool {
        uint256 unclaim;
        uint256 totalGameStaking;
        mapping(address => GamePool) gamePools;
        mapping(address => RunningGame) minerUnclaimedGames;
        mapping(address => RunningGame) playerUnclaimedGames;
    }

    // TODO
    address stake;
    address _epoch;
    address vesting;
    address gameMarket;
    address taskMarket;

    mapping(uint256 => EpochPool) private pools;

    /// @notice the numerator of Percentage of the game stake and labor (1-alpha) in the total
    int256 public alphaNumerator;

    /// @notice the denominator of the alpha
    int256 public alphaDenominator;

    /// @notice the numerator of Percentage of the miner stake and labor (1-beta) in the total
    int256 public betaNumerator;

    /// @notice the denominator of the beta
    int256 public betaDenominator;

    /// @notice the numerator of Percentage of the player stake and labor (1-beta) in the total
    int256 public gammaNumerator;

    /// @notice the denominator of the gamma
    int256 public gammaDenominator;

    /// @dev ### EVENTS
    /// @notice Emitted when update the alpha for cobb-douglas function
    event Alpha(int256 alphaNumerator, int256 alphaDenominator);

    /// @notice Emitted when update the alpha for cobb-douglas function
    event Beta(int256 betaNumerator, int256 betaDenominator);

    /// @notice Emitted when update the alpha for cobb-douglas function
    event Gamma(int256 betaNumerator, int256 betaDenominator);

    /// @notice Emitted when add Labor(reward) for current pool
    event MinerLabor(uint256 epoch, address game, address miner, uint256 work);

    /// @notice Emitted when add Labor(reward) for current pool
    event PlayerLabor(uint256 epoch, address game, address player, uint256 play);

    /// @notice Emitted when collect reward (stake) from pool
    event MinerCollect(uint256 epoch, address game, address miner, uint256 amount);

    /// @notice Emitted when collect reward (stake) from pool
    event PlayerCollect(uint256 epoch, address game, address player, uint256 amount);

    constructor() Ownable(msg.sender) {}

    /**
     * @notice Update the alpha for cobb-douglas function
     * @param _alphaNumerator the numerator of the alpha
     * @param _alphaDenominator the denominator of the alpha
     */
    function setAlpha(int256 _alphaNumerator, int256 _alphaDenominator) public onlyOwner {
        require(_alphaNumerator > 0 && _alphaDenominator > 0, 'R01');
        alphaNumerator = _alphaNumerator;
        alphaDenominator = _alphaDenominator;

        emit Alpha(alphaNumerator, alphaDenominator);
    }

    /**
     * @notice Update the beta for cobb-douglas function
     * @param _betaNumerator the numerator of the beta
     * @param _betaDenominator the denominator of the beta
     */
    function setBeta(int256 _betaNumerator, int256 _betaDenominator) public onlyOwner {
        require(_betaNumerator > 0 && _betaDenominator > 0, 'R01');
        betaNumerator = _betaNumerator;
        betaDenominator = _betaDenominator;

        emit Beta(betaNumerator, betaDenominator);
    }

    /**
     * @notice Update the gamma for cobb-douglas function
     * @param _gammaNumerator the numerator of the gamma
     * @param _gammaDenominator the denominator of the gamma
     */
    function setGamma(int256 _gammaNumerator, int256 _gammaDenominator) public onlyOwner {
        require(_gammaNumerator > 0 && _gammaDenominator > 0, 'R01');
        gammaNumerator = _gammaNumerator;
        gammaDenominator = _gammaDenominator;

        emit Gamma(gammaNumerator, gammaDenominator);
    }

    function work(address game, address player, address miner) external {
        require(msg.sender == taskMarket, "R01"); // only task contract

        Stake s = Stake(stake);

        uint256 currentEpoch = Epoch(_epoch).getAndUpdate();
        EpochPool storage ep = pools[currentEpoch];

        GamePool storage gp = ep.gamePools[game];
        RunningGame storage rgm = ep.minerUnclaimedGames[miner];
        RunningGame storage rgp = ep.playerUnclaimedGames[player];

        // game first has reward in this epoch
        if (gp.totalWorking == 0) {
            uint256 gameStaking = s.gameTotalStaking(game);
            ep.unclaim += 1;
            ep.totalGameStaking += gameStaking;
            gp.totalStaking = gameStaking;
        }

        gp.totalWorking += 2;
        gp.unclaimLabor += 2;

        if (gp.minerLabor[miner].working == 0) {
            uint256 minerStaking = s.minerStaking(game, miner);
            gp.minerLabor[miner].staking = minerStaking;
            gp.totalMinerStaking += minerStaking;
        }
        gp.minerLabor[miner].working += 1;

        if (gp.playerLabor[player].working == 0) {
            uint256 playerStaking = s.playerStaking(player);
            gp.playerLabor[player].staking = playerStaking;
            gp.totalPlayerStaking += playerStaking;
        }
        gp.playerLabor[player].working += 1;

        if (rgm.index[game] == 0) {
            if (rgm.unclaim == 0) {
                rgm.games.push(address(0));
            }
            rgm.games.push(game);
            rgm.unclaim += 1;
            rgm.index[game] = rgm.unclaim;
        }

        if (rgp.index[game] == 0) {
            if (rgp.unclaim == 0) {
                rgp.games.push(address(0));
            }
            rgp.games.push(game);
            rgp.unclaim += 1;
            rgp.index[game] = rgp.unclaim;
        }

        emit MinerLabor(currentEpoch, game, miner, gp.minerLabor[miner].working);
        emit PlayerLabor(currentEpoch, game, player, gp.playerLabor[player].working);
    }

    /// miner collect reward in epoch and game
    function minerCollect(uint256 epoch, address game, address miner) public {
        uint256 currentEpoch = Epoch(_epoch).getAndUpdate();
        require(epoch < currentEpoch, "R02");

        // prevent duplicated collect
        require(pools[epoch].minerUnclaimedGames[miner].index[game] > 0, "R03");

        _claimGameRewards(epoch, game);

        EpochPool storage ep = pools[epoch];
        GamePool storage gp = ep.gamePools[game];
        RunningGame storage rgm = ep.minerUnclaimedGames[miner];

        uint256 labor = gp.minerLabor[miner].working;
        uint256 amount = _cobbDouglas(
            gp.totalMinerReward,
            labor,
            gp.totalWorking / 2,
            gp.minerLabor[miner].staking,
            gp.totalMinerStaking,
            betaNumerator,
            betaDenominator
        );

        // Add amount to unstaking list
        if (amount > 0) {
            // TODO
        }

        // clear unclaim game
        uint index = rgm.index[game];
        address lastGame = rgm.games[rgm.unclaim];
        rgm.games[index] = lastGame;
        rgm.games.pop();
        rgm.index[lastGame] = index;
        delete rgm.index[game];

        // clear miner
        rgm.unclaim -= 1;
        if (rgm.unclaim == 0) {
            delete ep.minerUnclaimedGames[miner];
            delete gp.minerLabor[miner];
        }

        gp.unclaimLabor -= labor;
        gp.unclaimReward -= amount;

        _clearPool(epoch, game);

        emit MinerCollect(epoch, game, miner, amount);
    }

    /// player collect reward in epoch and game
    function playerCollect(uint256 epoch, address game, address player) public {
        uint256 currentEpoch = Epoch(_epoch).getAndUpdate();
        require(epoch < currentEpoch, "R02");

        // prevent duplicated collect
        require(pools[epoch].playerUnclaimedGames[player].index[game] > 0, "R03");

        _claimGameRewards(epoch, game);

        EpochPool storage ep = pools[epoch];
        GamePool storage gp = ep.gamePools[game];
        RunningGame storage rgp = ep.playerUnclaimedGames[player];

        uint256 labor = gp.playerLabor[player].working;
        uint256 amount = _cobbDouglas(
            gp.totalPlayerReward,
            labor,
            gp.totalWorking / 2,
            gp.playerLabor[player].staking,
            gp.totalPlayerStaking,
            gammaNumerator,
            gammaDenominator
        );

        // Add amount to unstaking list
        if (amount > 0) {
            // TODO
        }

        // clear unclaim game
        uint index = rgp.index[game];
        address lastGame = rgp.games[rgp.unclaim];
        rgp.games[index] = lastGame;
        rgp.games.pop();
        rgp.index[lastGame] = index;
        delete rgp.index[game];

        // clear player
        rgp.unclaim -= 1;
        if (rgp.unclaim == 0) {
            delete ep.playerUnclaimedGames[player];
            delete gp.playerLabor[player];
        }

        gp.unclaimLabor -= labor;
        gp.unclaimReward -= amount;

        _clearPool(epoch, game);

        emit PlayerCollect(epoch, game, player, amount);
    }

    /// miner batch collect all games in a epoch
    function minerBatchCollect(uint256 epoch, address miner) external {
        uint256 currentEpoch = Epoch(_epoch).getAndUpdate();
        require(epoch < currentEpoch, "R02");

        EpochPool storage ep = pools[epoch];
        RunningGame storage rgm = ep.minerUnclaimedGames[miner];

        uint lastIndex = rgm.unclaim;
        for (uint i = lastIndex; i > 0; i--) {
            address game = rgm.games[i];
            minerCollect(epoch, game, miner);
        }
    }

    /// player batch collect all games in a epoch
    function playerBatchCollect(uint256 epoch, address player) external {
        uint256 currentEpoch = Epoch(_epoch).getAndUpdate();
        require(epoch < currentEpoch, "R02");

        EpochPool storage ep = pools[epoch];
        RunningGame storage rgm = ep.playerUnclaimedGames[player];

        uint lastIndex = rgm.unclaim;
        for (uint i = lastIndex; i > 0; i--) {
            address game = rgm.games[i];
            playerCollect(epoch, game, player);
        }
    }

    function _claimGameRewards(uint256 epoch, address game) private {
        EpochPool storage ep = pools[epoch];
        GamePool storage gp = ep.gamePools[game];

        // check or collect game total reward,
        // and release epoch token to reward
        if (gp.totalMinerReward == 0 && gp.totalPlayerReward == 0) {
            GameMarket gm = GameMarket(gameMarket);
            Vesting vesting = Vesting(vesting);

            uint256 amount = _cobbDouglas(
                vesting.mine(epoch),
                gm.work(game),
                gm.totalWork(),
                gp.totalStaking,
                ep.totalGameStaking,
                alphaNumerator,
                alphaDenominator
            );

            // TODO release epoch amount token to contract
            gp.unclaimReward = amount;

            // TODO check or collect miner/player total reward,
            gp.totalMinerReward = 1000;
            gp.totalPlayerReward = 100;
        }
    }

    function _clearPool(uint256 epoch, address game) private {
        EpochPool storage ep = pools[epoch];
        GamePool storage gp = ep.gamePools[game];

        // clear game pool
        if (gp.unclaimLabor == 0) {
            // TODO return the remained
            if (gp.unclaimReward > 0) {
                //
            }

            delete ep.gamePools[game];
            ep.unclaim -= 1;

            // clear epoch pool
            if (ep.unclaim == 0) {
                delete pools[epoch];
            }
        }
    }

    /// @notice The cobb-doublas function has the form:
    /// @notice `reward * feeRatio ^ alpha * stakeRatio ^ (1-alpha)`
    /// @notice This is equivalent to:
    /// @notice `reward * stakeRatio * e^(alpha * (ln(feeRatio / stakeRatio)))`
    /// @notice However, because `ln(x)` has the domain of `0 < x < 1`
    /// @notice and `exp(x)` has the domain of `x < 0`,
    /// @notice and fixed-point math easily overflows with multiplication,
    /// @notice we will choose the following if `stakeRatio > feeRatio`:
    /// @notice `reward * stakeRatio / e^(alpha * (ln(stakeRatio / feeRatio)))`
    function _cobbDouglas(
        uint256 reward,
        uint256 myLabor,
        uint256 totalLabor,
        uint256 myStake,
        uint256 totalStake,
        int256 numerator,
        int256 denominator
    ) private pure returns (uint256) {
        if (myStake == totalStake || myLabor == totalLabor) {
            return reward;
        }

        int256 feeRatio = FixedMath.toFixed(myLabor, totalLabor);
        int256 stakeRatio = FixedMath.toFixed(myStake, totalStake);
        if (feeRatio == 0 || stakeRatio == 0) {
            return 0;
        }

        // Compute
        // `e^(alpha * ln(feeRatio/stakeRatio))` if feeRatio <= stakeRatio
        // or
        // `e^(alpa * ln(stakeRatio/feeRatio))` if feeRatio > stakeRatio
        int256 n = feeRatio <= stakeRatio
            ? FixedMath.div(feeRatio, stakeRatio)
            : FixedMath.div(stakeRatio, feeRatio);
        n = FixedMath.exp(
            FixedMath.mulDiv(FixedMath.ln(n), numerator, denominator)
        );
        // Compute
        // `reward * n` if feeRatio <= stakeRatio
        // or
        // `reward / n` if stakeRatio > feeRatio
        // depending on the choice we made earlier.
        n = feeRatio <= stakeRatio ? FixedMath.mul(stakeRatio, n) : FixedMath.div(stakeRatio, n);
        // Multiply the above with reward.
        return FixedMath.uintMul(n, reward);
    }
}
