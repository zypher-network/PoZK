// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IAddresses.sol";
import "./interface/IEpoch.sol";
import "./interface/IStake.sol";
import "./interface/IReward.sol";
import "./interface/IProver.sol";
import "./interface/IVesting.sol";
import "./utils/FixedMath.sol";

/// @notice Reward Contract for reward distribute and claim, miner & player can get reward,
/// when they play game and prove zkp, all of them can get reward,
/// use cobb-douglas function for work and labor
contract Reward is Initializable, OwnableUpgradeable, IReward {
    using SafeERC20 for IERC20;

    /// @notice The unit struct of stake and labor
    struct StakerLabor {
        uint256 staking;
        uint256 working;
    }

    /// @notice The struct of prover pool
    struct ProverPool {
        /// the prover PoZK
        uint256 pozk;

        /// miner + player work (double)
        uint256 totalWorking;

        /// prover total staking
        uint256 totalStaking;

        uint256 totalMinerStaking;
        uint256 totalPlayerStaking;

        uint256 totalMinerReward;
        uint256 totalPlayerReward;
        uint256 totalMinerExtraReward;
        uint256 totalPlayerExtraReward;

        uint256 unclaimReward;
        uint256 unclaimMinerLabor;
        uint256 unclaimPlayerLabor;
        uint256 unclaimExtra;
        address extraRewardToken;

        mapping(address => StakerLabor) minerLabor;
        mapping(address => StakerLabor) playerLabor;
    }

    /// @notice The struct of a prover status
    struct RunningProver {
        uint unclaim;
        // first is 0 for better use
        address[] provers;
        mapping(address => uint) index;
    }

    /// @notice The struct of the epoch status
    struct EpochPool {
        uint256 unclaim;
        uint256 totalPozk;
        uint256 totalProverStaking;
        mapping(address => ProverPool) proverPools;
        mapping(address => RunningProver) minerUnclaimedProvers;
        mapping(address => RunningProver) playerUnclaimedProvers;
    }

    /// @notice The extra prover reward struct
    struct ExtraProverReward {
        /// the token address of this reward
        address token;
        /// total reward, if not used, prover owner can claim
        uint256 remain;
    }

    /// @notice Common Addresses contract
    address addresses;

    /// @notice Store all epoch provers
    mapping(uint256 => EpochPool) private pools;

    /// @notice The numerator of Percentage of the prover stake and labor (1-alpha) in the total
    int256 public alphaNumerator;

    /// @notice The denominator of the alpha
    int256 public alphaDenominator;

    /// @notice The numerator of Percentage of the miner stake and labor (1-beta) in the total
    int256 public betaNumerator;

    /// @notice The denominator of the beta
    int256 public betaDenominator;

    /// @notice The numerator of Percentage of the player stake and labor (1-beta) in the total
    int256 public gammaNumerator;

    /// @notice The denominator of the gamma
    int256 public gammaDenominator;

    /// @notice The miner max percent of reward
    uint256 public minerMaxPer;

    /// @notice The miner min percent of reward
    uint256 public minerMinPer;

    /// @notice The player max games number when reach minerMaxPer
    uint256 public playerMaxNum;

    /// @notice The extra prover rewards by game, it will distribute with main token
    mapping(address => mapping(uint256 => ExtraProverReward)) public extraProverRewards;

    /// @notice Emitted when update the alpha for cobb-douglas function
    event Alpha(int256 alphaNumerator, int256 alphaDenominator);

    /// @notice Emitted when update the alpha for cobb-douglas function
    event Beta(int256 betaNumerator, int256 betaDenominator);

    /// @notice Emitted when update the alpha for cobb-douglas function
    event Gamma(int256 gammaNumerator, int256 gammaDenominator);

    /// @notice Emitted when update the percent of miner and player
    event MinerPlayerPer(uint256 minerMaxPer, uint256 minerMinPer, uint256 playerMaxNum);

    /// @notice Emitted when add Labor(reward) for current pool
    event MinerLabor(uint256 epoch, address prover, address miner, uint256 work);

    /// @notice Emitted when add Labor(reward) for current pool
    event PlayerLabor(uint256 epoch, address prover, address player, uint256 play);

    /// @notice Emitted when collect reward (stake) from pool
    event MinerCollect(uint256 epoch, address prover, address miner, uint256 amount);

    /// @notice Emitted when collect reward (stake) from pool
    event PlayerCollect(uint256 epoch, address prover, address player, uint256 amount);

    /// @notice Emitted when deposit extra reward token for miner
    event DepositExtraProverRewards(address prover, uint256 epoch, address token, uint256 amount);

    /// @notice Emitted when claimed unused extra reward token
    event ClaimExtraProverRewards(address prover, uint256 epoch, uint256 remain);

    /// @notice Initialize
    /// @param _addresses the Addresses contract
    /// @param _alphaNumerator the numerator of the alpha
    /// @param _alphaDenominator the denominator of the alpha
    /// @param _betaNumerator the numerator of the beta
    /// @param _betaDenominator the denominator of the beta
    /// @param _gammaNumerator the numerator of the gamma
    /// @param _gammaDenominator the denominator of the gamma
    /// @param _minerMaxPer The miner max percent of reward
    /// @param _minerMinPer The miner min percent of reward
    /// @param _playerMaxNum The player max games number when reach minerMaxPer
    function initialize(
        address _addresses,
        int256 _alphaNumerator,
        int256 _alphaDenominator,
        int256 _betaNumerator,
        int256 _betaDenominator,
        int256 _gammaNumerator,
        int256 _gammaDenominator,
        uint256 _minerMaxPer,
        uint256 _minerMinPer,
        uint256 _playerMaxNum
    ) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;

        alphaNumerator = _alphaNumerator;
        alphaDenominator = _alphaDenominator;
        emit Alpha(alphaNumerator, alphaDenominator);

        betaNumerator = _betaNumerator;
        betaDenominator = _betaDenominator;
        emit Beta(betaNumerator, betaDenominator);

        gammaNumerator = _gammaNumerator;
        gammaDenominator = _gammaDenominator;
        emit Gamma(gammaNumerator, gammaDenominator);

        minerMaxPer = _minerMaxPer;
        minerMinPer = _minerMinPer;
        playerMaxNum = _playerMaxNum;
        emit MinerPlayerPer(minerMaxPer, minerMinPer, playerMaxNum);
    }

    /// @notice Set the Addresses contract
    /// @param _addresses the Addresses contract
    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    /// @notice Update the alpha for cobb-douglas function
    /// @param _alphaNumerator the numerator of the alpha
    /// @param _alphaDenominator the denominator of the alpha
    function setAlpha(int256 _alphaNumerator, int256 _alphaDenominator) public onlyOwner {
        require(_alphaNumerator > 0 && _alphaDenominator > 0, 'R01');
        alphaNumerator = _alphaNumerator;
        alphaDenominator = _alphaDenominator;

        emit Alpha(alphaNumerator, alphaDenominator);
    }

    /// @notice Update the beta for cobb-douglas function
    /// @param _betaNumerator the numerator of the beta
    /// @param _betaDenominator the denominator of the beta
    function setBeta(int256 _betaNumerator, int256 _betaDenominator) public onlyOwner {
        require(_betaNumerator > 0 && _betaDenominator > 0, 'R01');
        betaNumerator = _betaNumerator;
        betaDenominator = _betaDenominator;

        emit Beta(betaNumerator, betaDenominator);
    }

    /// @notice Update the gamma for cobb-douglas function
    /// @param _gammaNumerator the numerator of the gamma
    /// @param _gammaDenominator the denominator of the gamma
    function setGamma(int256 _gammaNumerator, int256 _gammaDenominator) public onlyOwner {
        require(_gammaNumerator > 0 && _gammaDenominator > 0, 'R01');
        gammaNumerator = _gammaNumerator;
        gammaDenominator = _gammaDenominator;

        emit Gamma(gammaNumerator, gammaDenominator);
    }

    /// @notice Update the miner and player reward percent
    /// @param _minerMaxPer The miner max percent of reward
    /// @param _minerMinPer The miner min percent of reward
    /// @param _playerMaxNum The player max games number when reach minerMaxPer
    function setMinerPlayerPer(uint256 _minerMaxPer, uint256 _minerMinPer, uint256 _playerMaxNum) public onlyOwner {
        minerMaxPer = _minerMaxPer;
        minerMinPer = _minerMinPer;
        playerMaxNum = _playerMaxNum;

        emit MinerPlayerPer(minerMaxPer, minerMinPer, playerMaxNum);
    }

    /// @notice Add work(labor) to current epoch & prover, only call from Task
    /// @param prover the prover address
    /// @param player player account
    /// @param miner miner account
    function work(address prover, address player, address miner) external {
        require(msg.sender == IAddresses(addresses).get(Contracts.Task), "R01"); // only task contract

        IStake s = IStake(IAddresses(addresses).get(Contracts.Stake));

        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();
        EpochPool storage ep = pools[currentEpoch];

        ProverPool storage gp = ep.proverPools[prover];
        RunningProver storage rgm = ep.minerUnclaimedProvers[miner];
        RunningProver storage rgp = ep.playerUnclaimedProvers[player];

        // prover first has reward in this epoch
        if (gp.totalWorking == 0) {
            uint256 proverStaking = s.proverTotalStaking(prover);
            if (proverStaking == 0) {
                return;
            }

            ep.unclaim += 1;
            ep.totalProverStaking += proverStaking;
            gp.totalStaking = proverStaking;

            IProver p = IProver(IAddresses(addresses).get(Contracts.Prover));
            gp.pozk = p.work(prover);
            ep.totalPozk += gp.pozk;
        }
        gp.totalWorking += 2;

        // add labor to miner
        if (gp.minerLabor[miner].working == 0) {
            uint256 minerStaking = s.minerStaking(prover, miner);
            gp.minerLabor[miner].staking = minerStaking;
            gp.totalMinerStaking += minerStaking;
        }
        if (gp.minerLabor[miner].staking > 0) {
            gp.unclaimMinerLabor += 1;
        }
        gp.minerLabor[miner].working += 1;

        // add labor to player
        if (gp.playerLabor[player].working == 0) {
            uint256 playerStaking = s.playerStaking(player);
            gp.playerLabor[player].staking = playerStaking;
            gp.totalPlayerStaking += playerStaking;
        }
        if (gp.playerLabor[player].staking > 0) {
            gp.unclaimPlayerLabor += 1;
        }
        gp.playerLabor[player].working += 1;

        // add unclaim list
        if (rgm.index[prover] == 0) {
            if (rgm.unclaim == 0) {
                rgm.provers.push(address(0));
            }
            rgm.provers.push(prover);
            rgm.unclaim += 1;
            rgm.index[prover] = rgm.unclaim;
        }

        if (rgp.index[prover] == 0) {
            if (rgp.unclaim == 0) {
                rgp.provers.push(address(0));
            }
            rgp.provers.push(prover);
            rgp.unclaim += 1;
            rgp.index[prover] = rgp.unclaim;
        }

        emit MinerLabor(currentEpoch, prover, miner, 1);
        emit PlayerLabor(currentEpoch, prover, player, 1);
    }

    /// @notice Miner collect reward in epoch and prover, collect reward to unstaking list
    /// @param epoch the epoch height
    /// @param prover the prover address
    /// @param miner the miner account
    function minerCollect(uint256 epoch, address prover, address miner) public {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();
        require(epoch < currentEpoch, "R02");

        // prevent duplicated collect
        require(pools[epoch].minerUnclaimedProvers[miner].index[prover] > 0, "R03");

        _claimProverRewards(epoch, prover);

        EpochPool storage ep = pools[epoch];
        ProverPool storage gp = ep.proverPools[prover];
        RunningProver storage rgm = ep.minerUnclaimedProvers[miner];

        uint256 labor = gp.minerLabor[miner].working;
        (uint256 amount, uint256 extraAmount) = _doubleCobbDouglas(
            gp.totalMinerReward,
            gp.totalMinerExtraReward,
            labor,
            gp.totalWorking / 2,
            gp.minerLabor[miner].staking,
            gp.totalMinerStaking,
            betaNumerator,
            betaDenominator
        );

        // Add amount to unstaking list
        if (amount > 0) {
            address stake = IAddresses(addresses).get(Contracts.Stake);
            IERC20(IAddresses(addresses).get(Contracts.Token)).transfer(stake, amount);
            IStake(stake).addUnstaking(miner, amount);
        }

        if (extraAmount > 0) {
            IERC20(gp.extraRewardToken).transfer(miner, extraAmount);
        }

        if (gp.minerLabor[miner].staking > 0) {
            gp.unclaimMinerLabor -= labor;
        }

        // clear unclaim prover
        uint index = rgm.index[prover];
        address lastProver = rgm.provers[rgm.unclaim];
        rgm.provers[index] = lastProver;
        rgm.provers.pop();
        rgm.index[lastProver] = index;
        delete rgm.index[prover];

        // clear miner
        rgm.unclaim -= 1;
        if (rgm.unclaim == 0) {
            delete ep.minerUnclaimedProvers[miner];
            delete gp.minerLabor[miner];
        }

        gp.unclaimReward -= amount;
        gp.unclaimExtra -= extraAmount;

        _clearPool(epoch, prover);

        emit MinerCollect(epoch, prover, miner, amount);
    }

    /// @notice Player collect reward in epoch and prover, collect to player wallet
    /// @param epoch the epoch height
    /// @param prover the prover address
    /// @param player the player account
    function playerCollect(uint256 epoch, address prover, address player) public {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();
        require(epoch < currentEpoch, "R02");

        // prevent duplicated collect
        require(pools[epoch].playerUnclaimedProvers[player].index[prover] > 0, "R03");

        _claimProverRewards(epoch, prover);

        EpochPool storage ep = pools[epoch];
        ProverPool storage gp = ep.proverPools[prover];
        RunningProver storage rgp = ep.playerUnclaimedProvers[player];

        uint256 labor = gp.playerLabor[player].working;
        (uint256 amount, uint256 extraAmount) = _doubleCobbDouglas(
            gp.totalPlayerReward,
            gp.totalPlayerExtraReward,
            labor,
            gp.totalWorking / 2,
            gp.playerLabor[player].staking,
            gp.totalPlayerStaking,
            gammaNumerator,
            gammaDenominator
        );

        // Add amount to unstaking list
        if (amount > 0) {
            address stake = IAddresses(addresses).get(Contracts.Stake);
            IERC20(IAddresses(addresses).get(Contracts.Token)).transfer(stake, amount);
            IStake(stake).addUnstaking(player, amount);
        }

        if (extraAmount > 0) {
            IERC20(gp.extraRewardToken).transfer(player, extraAmount);
        }

        if (gp.playerLabor[player].staking > 0) {
            gp.unclaimPlayerLabor -= labor;
        }

        // clear unclaim prover
        uint index = rgp.index[prover];
        address lastProver = rgp.provers[rgp.unclaim];
        rgp.provers[index] = lastProver;
        rgp.provers.pop();
        rgp.index[lastProver] = index;
        delete rgp.index[prover];

        // clear player
        rgp.unclaim -= 1;
        if (rgp.unclaim == 0) {
            delete ep.playerUnclaimedProvers[player];
            delete gp.playerLabor[player];
        }

        gp.unclaimReward -= amount;
        gp.unclaimExtra -= extraAmount;

        _clearPool(epoch, prover);

        emit PlayerCollect(epoch, prover, player, amount);
    }

    /// @notice Miner batch collect all provers in a epoch
    /// @param epoch the epoch height
    /// @param miner the miner account
    function minerBatchCollect(uint256 epoch, address miner) external {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();
        require(epoch < currentEpoch, "R02");

        EpochPool storage ep = pools[epoch];
        RunningProver storage rgm = ep.minerUnclaimedProvers[miner];

        uint lastIndex = rgm.unclaim;
        for (uint i = lastIndex; i > 0; i--) {
            address prover = rgm.provers[i];
            minerCollect(epoch, prover, miner);
        }
    }

    /// @notice Player batch collect all provers in a epoch
    /// @param epoch the epoch height
    /// @param player the player account
    function playerBatchCollect(uint256 epoch, address player) external {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();
        require(epoch < currentEpoch, "R02");

        EpochPool storage ep = pools[epoch];
        RunningProver storage rgm = ep.playerUnclaimedProvers[player];

        uint lastIndex = rgm.unclaim;
        for (uint i = lastIndex; i > 0; i--) {
            address prover = rgm.provers[i];
            playerCollect(epoch, prover, player);
        }
    }

    /// @notice Prover/Game owner can deposit extra rewards for miner & player. Only support one token in epoch.
    /// @param prover the prover
    /// @param epoch the reward epoch
    /// @param token the token address
    /// @param amount the token amount
    function depositExtraProverRewards(address prover, uint256 epoch, address token, uint256 amount) external {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();
        require(epoch >= currentEpoch, "R04");

        ExtraProverReward storage epr = extraProverRewards[prover][epoch];

        // check reward is empty or expired, only prover owner can set reward token address
        if (epr.token == address(0)) {
            address owner = IProver(IAddresses(addresses).get(Contracts.Prover)).owner(prover);
            require(owner == msg.sender, "R05");
            epr.token = token;
        }

        IERC20(epr.token).transferFrom(msg.sender, address(this), amount);
        epr.remain += amount;

        emit DepositExtraProverRewards(prover, epoch, token, amount);
    }

    /// @notice Prover/Game owner can claim expired extra rewards
    /// @param prover the prover
    /// @param epoch the epoch
    function claimExtraProverRewards(address prover, uint256 epoch) external {
       uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();
        require(currentEpoch > epoch, "R06");

        ExtraProverReward storage epr = extraProverRewards[prover][epoch];
        require(epr.remain > 0, "R07");

        // check epoch had been claimed
        ProverPool storage gp = pools[epoch].proverPools[prover];
        require(gp.unclaimMinerLabor == 0 && gp.unclaimPlayerLabor == 0, "R08");

        address owner = IProver(IAddresses(addresses).get(Contracts.Prover)).owner(prover);
        IERC20(epr.token).transfer(owner, epr.remain);
        emit ClaimExtraProverRewards(prover, epoch, epr.remain);

        delete extraProverRewards[prover][epoch];
    }

    /// @notice private function about claim reward
    /// @param epoch the epoch height
    /// @param prover the prover address
    function _claimProverRewards(uint256 epoch, address prover) private {
        EpochPool storage ep = pools[epoch];
        ProverPool storage gp = ep.proverPools[prover];

        // check or collect prover total reward,
        // and release epoch token to reward
        if (gp.totalMinerReward == 0 && gp.totalPlayerReward == 0) {
            address vestingAddress = IAddresses(addresses).get(Contracts.Vesting);
            IVesting vesting = IVesting(vestingAddress);

            (uint256 amount,) = _doubleCobbDouglas(
                vesting.mine(epoch),
                0,
                gp.pozk,
                ep.totalPozk,
                gp.totalStaking,
                ep.totalProverStaking,
                alphaNumerator,
                alphaDenominator
            );

            // release epoch amount token to contract
            IERC20(IAddresses(addresses).get(Contracts.Token)).transferFrom(vestingAddress, address(this), amount);
            gp.unclaimReward = amount;

            // extra token reward
            ExtraProverReward storage epr = extraProverRewards[prover][epoch];
            gp.unclaimExtra = epr.remain;
            gp.extraRewardToken = epr.token;
            emit ClaimExtraProverRewards(prover, epoch, epr.remain);
            delete extraProverRewards[prover][epoch];

            // check or collect miner/player total reward,
            // miner percent: y, player percent: 100 - y
            // miner max per: p%，miner min per q%，
            // player max num (games max number for reward): t, current games number: x
            // (x - 1)  / (t - 1) * (p - q) + q = y => x * (p - q) / t + q = y
            uint256 x = gp.totalWorking / 2;
            uint256 y = minerMaxPer;
            if (x < playerMaxNum) {
                y = x * (minerMaxPer - minerMinPer) / playerMaxNum + minerMinPer;
            }
            gp.totalMinerReward = gp.unclaimReward * y / 100;
            gp.totalPlayerReward = gp.unclaimReward - gp.totalMinerReward;

            gp.totalMinerExtraReward = gp.unclaimExtra * y / 100;
            gp.totalPlayerExtraReward = gp.unclaimExtra - gp.totalMinerExtraReward;
        }
    }

    /// @notice private function about clear pool
    /// @param epoch the epoch height
    /// @param prover the prover address
    function _clearPool(uint256 epoch, address prover) private {
        EpochPool storage ep = pools[epoch];
        ProverPool storage gp = ep.proverPools[prover];

        // clear prover pool
        if (gp.unclaimMinerLabor == 0 && gp.unclaimPlayerLabor == 0) {
            // return the remained
            if (gp.unclaimReward > 0) {
                address vesting = IAddresses(addresses).get(Contracts.Vesting);
                IERC20(IAddresses(addresses).get(Contracts.Token)).transfer(vesting, gp.unclaimReward);
            }

            // return the extra
            if (gp.unclaimExtra > 0) {
                address owner = IProver(IAddresses(addresses).get(Contracts.Prover)).owner(prover);
                IERC20(gp.extraRewardToken).transfer(owner, gp.unclaimExtra);
            }

            delete ep.proverPools[prover];
            ep.unclaim -= 1;

            // clear epoch pool
            if (ep.unclaim == 0) {
                delete pools[epoch];
            }
        }
    }

    /// @notice The cobb-doublas function has the form:
    /// `reward * feeRatio ^ alpha * stakeRatio ^ (1-alpha)`
    /// This is equivalent to:
    /// `reward * stakeRatio * e^(alpha * (ln(feeRatio / stakeRatio)))`
    /// However, because `ln(x)` has the domain of `0 < x < 1`
    /// and `exp(x)` has the domain of `x < 0`,
    /// and fixed-point math easily overflows with multiplication,
    /// we will choose the following if `stakeRatio > feeRatio`:
    /// `reward * stakeRatio / e^(alpha * (ln(stakeRatio / feeRatio)))`
    function _doubleCobbDouglas(
        uint256 reward1,
        uint256 reward2,
        uint256 myLabor,
        uint256 totalLabor,
        uint256 myStake,
        uint256 totalStake,
        int256 numerator,
        int256 denominator
    ) private pure returns (uint256, uint256) {
        if (myStake == 0 || myLabor == 0) {
            return (0, 0);
        }

        if (myStake == totalStake || myLabor == totalLabor) {
            return (reward1, reward2);
        }

        int256 feeRatio = FixedMath.toFixed(myLabor, totalLabor);
        int256 stakeRatio = FixedMath.toFixed(myStake, totalStake);

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
        return (FixedMath.uintMul(n, reward1), FixedMath.uintMul(n, reward2));
    }

    /* function viewProverStatus(uint256 epoch, address prover) external view returns (uint256, uint256, uint256, uint256) { */
    /*     IProver gm = IProver(IAddresses(addresses).get(Contracts.Prover)); */
    /*     EpochPool storage ep = pools[epoch]; */
    /*     ProverPool storage gp = ep.proverPools[prover]; */

    /*     return ( */
    /*         gm.work(prover), */
    /*         gm.totalWork(), */
    /*         gp.totalStaking, */
    /*         ep.totalProverStaking */
    /*     ); */
    /* } */

    /* function viewMinerStatus(uint256 epoch, address prover, address miner) external view returns (uint256, uint256, uint256, uint256) { */
    /*     ProverPool storage gp = pools[epoch].proverPools[prover]; */

    /*     return ( */
    /*         gp.minerLabor[miner].working, */
    /*         gp.totalWorking / 2, */
    /*         gp.minerLabor[miner].staking, */
    /*         gp.totalMinerStaking */
    /*     ); */
    /* } */

    /* function viewPlayerStatus(uint256 epoch, address prover, address player) external view returns (uint256, uint256, uint256, uint256) { */
    /*     ProverPool storage gp = pools[epoch].proverPools[prover]; */

    /*     return ( */
    /*         gp.playerLabor[player].working, */
    /*         gp.totalWorking / 2, */
    /*         gp.playerLabor[player].staking, */
    /*         gp.totalPlayerStaking */
    /*     ); */
    /* } */
}
