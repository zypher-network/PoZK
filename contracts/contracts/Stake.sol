// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

import "./Epoch.sol";
import "./Vesting.sol";

/**
 * @title Stake Contract
 */
contract Stake is Ownable {
    using SafeERC20 for IERC20;

    /// uint for staking/unstaking
    struct Staking {
        uint256 value;
        uint256 newValue;
        uint256 newEpoch;
    }

    /// staking in a game
    struct GameStaking {
        /// game self total staking
        Staking gamerTotal;
        /// game self staking list
        mapping(address => uint256) gamers;
        /// miner total staking
        Staking minerTotal;
        /// miner staking list
        mapping(address => Staking) miners;
    }

    /// miner minStakeAmount
    uint256 public minStakeAmount;

    /// game => game staking
    mapping(address => GameStaking) private gamesStaking;

    /// player total staking
    Staking private playerTotal;
    /// player => staking
    mapping(address => Staking) private playersStaking;

    /// miners/players unstaking list
    mapping(address => Staking) private unstakings;

    // TODO
    address epoch;
    address token;
    address vesting;

    event GameStakeChange(uint256 epoch, address game, address account, int256 changed, uint256 total);
    event MinerStakeChange(uint256 epoch, address game, address account, int256 changed, uint256 total);
    event PlayerStakeChange(uint256 epoch, address account, int256 changed, uint256 total);

    constructor() Ownable(msg.sender) {}

    /// set minimum stake amount
    function setMinStakeAmount(uint256 _minStakeAmount) external onlyOwner {
        minStakeAmount = _minStakeAmount;
    }

    /// --------------- Game --------------- ///

    /// get total game staking
    function gameTotalStaking(address game) external view returns (uint256) {
        uint256 currentEpoch = Epoch(epoch).get();

        Staking storage st = gamesStaking[game].gamerTotal;

        if (currentEpoch >= st.newEpoch) {
            return st.newValue;
        } else {
            return st.value;
        }
    }

    /// get game staking by account
    function gameStaking(address game, address account) external view returns (uint256) {
        return gamesStaking[game].gamers[account];
    }

    // stake by game self
    function gameStake(address game, uint256 amount) external {
        uint256 currentEpoch = Epoch(epoch).getAndUpdate();

        // transfer from account
        IERC20(token).transferFrom(msg.sender, address(this), amount);

        // add to game stakers
        GameStaking storage gs = gamesStaking[game];
        gs.gamers[msg.sender] += amount;

        // add to total staking
        Staking storage st = gs.gamerTotal;
        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue += amount;

        emit GameStakeChange(st.newEpoch, game, msg.sender, int256(amount), st.newValue);
    }

    /// unstake by game self
    function gameUnstake(address game, uint256 amount) external {
        GameStaking storage gs = gamesStaking[game];
        require(gs.gamers[msg.sender] >= amount, "S01");

        uint256 currentEpoch = Epoch(epoch).getAndUpdate();
        gs.gamers[msg.sender] -= amount;

        // transfer to account
        IERC20(token).transfer(msg.sender, amount);

        // remove from total staking
        Staking storage st = gs.gamerTotal;
        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue -= amount;

        emit GameStakeChange(st.newEpoch, game, msg.sender, -int256(amount), st.newValue);
    }

    /// --------------- Miner --------------- ///

    /// get total miner staking
    function minerTotalStaking(address game) external view returns (uint256) {
        uint256 currentEpoch = Epoch(epoch).get();
        Staking storage st = gamesStaking[game].minerTotal;

        uint256 minersVesting = Vesting(vesting).minersTotal();

        if (currentEpoch >= st.newEpoch) {
            return st.newValue + minersVesting;
        } else {
            return st.value + minersVesting;
        }
    }

    /// get miner staking
    function minerStaking(address game, address account) public view returns (uint256) {
        uint256 currentEpoch = Epoch(epoch).get();
        Staking storage st = gamesStaking[game].miners[account];

        uint256 minerVesting = Vesting(vesting).miner(account);

        if (currentEpoch >= st.newEpoch) {
            return st.newValue + minerVesting;
        } else {
            return st.value + minerVesting;
        }
    }

    // check account is miner or not
    function isMiner(address game, address account) external view returns (bool) {
        uint256 staking = minerStaking(game, account);
        return staking >= minStakeAmount;
    }

    // stake by miner
    function minerStake(address game, uint256 amount) external {
        uint256 currentEpoch = Epoch(epoch).getAndUpdate();

        // transfer from account
        IERC20(token).transferFrom(msg.sender, address(this), amount);

        GameStaking storage gs = gamesStaking[game];

        // add to staking
        Staking storage sm = gs.miners[msg.sender];
        if (currentEpoch >= sm.newEpoch) {
            sm.value = sm.newValue;
            sm.newEpoch = currentEpoch + 1;
        }
        sm.newValue += amount;
        require(sm.newValue >= minStakeAmount, "S03");

        // add to total staking
        Staking storage st = gs.minerTotal;
        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue += amount;

        emit MinerStakeChange(st.newEpoch, game, msg.sender, int256(amount), st.newValue);
    }

    // unstake by miner
    function minerUnStake(address game, uint256 amount) external {
        uint256 currentEpoch = Epoch(epoch).getAndUpdate();

        GameStaking storage gs = gamesStaking[game];
        Staking storage sm = gs.miners[msg.sender];

        // update new staking
        if (currentEpoch >= sm.newEpoch) {
            sm.value = sm.newValue;
            sm.newEpoch = currentEpoch + 1;
        }
        require(sm.newValue >= amount, "S01");

        // remove from staking
        sm.newValue -= amount;
        require(sm.newValue == 0 || sm.newValue >= minStakeAmount, "S04");

        // append to unstaking
        Staking storage su = unstakings[msg.sender];
        if (currentEpoch >= su.newEpoch) {
            su.value += su.newValue;
            su.newValue = 0;
            su.newEpoch = currentEpoch + 1;
        }
        su.newValue += amount;

        // remove from total staking
        Staking storage st = gs.minerTotal;
        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue -= amount;

        emit MinerStakeChange(st.newEpoch, game, msg.sender, -int256(amount), st.newValue);
    }

    /// --------------- Player --------------- ///

    /// get total player staking
    function playerTotalStaking() external view returns (uint256) {
        uint256 currentEpoch = Epoch(epoch).get();
        Staking storage st = playerTotal;

        if (currentEpoch >= st.newEpoch) {
            return st.newValue;
        } else {
            return st.value;
        }
    }

    /// get player staking
    function playerStaking(address account) external view returns (uint256) {
        uint256 currentEpoch = Epoch(epoch).get();
        Staking storage st = playersStaking[account];

        if (currentEpoch >= st.newEpoch) {
            return st.newValue;
        } else {
            return st.value;
        }
    }

    /// stake by player
    function playerStake(uint256 amount) external {
        uint256 currentEpoch = Epoch(epoch).getAndUpdate();

        // transfer from account
        IERC20(token).transferFrom(msg.sender, address(this), amount);

        // add to staking
        Staking storage sp = playersStaking[msg.sender];
        if (currentEpoch >= sp.newEpoch) {
            sp.value = sp.newValue;
            sp.newEpoch = currentEpoch + 1;
        }
        sp.newValue += amount;

        // add to total staking
        Staking storage st = playerTotal;
        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue += amount;

        emit PlayerStakeChange(st.newEpoch, msg.sender, int256(amount), st.newValue);
    }

    /// unstake by player
    function playerUnStake(uint256 amount) external {
        uint256 currentEpoch = Epoch(epoch).getAndUpdate();

        Staking storage sp = playersStaking[msg.sender];

        // update new staking
        if (currentEpoch >= sp.newEpoch) {
            sp.value = sp.newValue;
            sp.newEpoch = currentEpoch + 1;
        }
        require(sp.newValue >= amount, "S01");

        // remove from staking
        sp.newValue -= amount;

        // append to unstaking
        Staking storage su = unstakings[msg.sender];
        if (currentEpoch >= su.newEpoch) {
            su.value += su.newValue;
            su.newValue = 0;
            su.newEpoch = currentEpoch + 1;
        }
        su.newValue += amount;

        // remove from total staking
        Staking storage st = playerTotal;
        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue -= amount;

        emit PlayerStakeChange(st.newEpoch, msg.sender, -int256(amount), st.newValue);
    }

    /// --------------- Unstaking --------------- ///

    /// get claimable unstaking amount
    function claimable(address account) external view returns (uint256) {
        uint256 currentEpoch = Epoch(epoch).get();
        Staking storage su = unstakings[account];

        if (currentEpoch >= su.newEpoch) {
            return su.value + su.newValue;
        } else {
            return su.value;
        }
    }

    /// claim unstaking to account
    function claim(address account) external {
        uint256 currentEpoch = Epoch(epoch).getAndUpdate();
        Staking storage su = unstakings[account];

        uint256 amount = su.value;
        if (currentEpoch >= su.newEpoch) {
            amount += su.newValue;
            su.newValue = 0;
            su.newEpoch = currentEpoch + 1;
        }
        su.value = 0;
        require(amount > 0, "S02");

        // transfer amount to account
        IERC20(token).transfer(account, amount);
    }
}
