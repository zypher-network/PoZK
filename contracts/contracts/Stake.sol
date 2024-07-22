// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IAddresses.sol";
import "./interface/IEpoch.sol";
import "./interface/IVesting.sol";
import "./interface/IStake.sol";


/// @notice Stake Contract, including player, miner & prover,
/// every change will work in next epoch, and unstake can claim in next epoch
contract Stake is Initializable, OwnableUpgradeable, IStake {
    using SafeERC20 for IERC20;

    /// @notice Unit struct about staking/unstaking
    struct Staking {
        uint256 value;
        uint256 newValue;
        uint256 newEpoch;
    }

    /// @notice Unit struct about staking in a prover
    struct ProverStaking {
        /// @notice Prover self total staking
        Staking proverTotal;
        /// @notice Prover self staking list
        mapping(address => uint256) provers;
        /// @notice Miner total staking
        Staking minerTotal;
        /// @notice Miner staking list
        mapping(address => Staking) miners;
    }

    /// @notice Common Addresses contract
    address addresses;

    /// @notice Miner minStakeAmount
    uint256 public minStakeAmount;

    /// @notice Store all provers staking
    mapping(address => ProverStaking) private proversStaking;

    /// @notice Total players staking
    Staking private playerTotal;

    /// @notice Store all players staking
    mapping(address => Staking) private playersStaking;

    /// @notice Store miners/players unstaking list
    mapping(address => Staking) private unstakings;

    /// @notice Emit when prover staking change
    event ProverStakeChange(uint256 epoch, address prover, address account, int256 changed, uint256 total);

    /// @notice Emit when miner staking change
    event MinerStakeChange(uint256 epoch, address prover, address account, int256 changed, uint256 total);

    /// @notice Emit when player staking change
    event PlayerStakeChange(uint256 epoch, address account, int256 changed, uint256 total);

    /// @notice Initialize
    /// @param _addresses the Addresses contract
    function initialize(address _addresses) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;
    }

    /// @notice Set the Addresses contract
    /// @param _addresses the Addresses contract
    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    /// @notice Set minimum stake amount
    /// @param _minStakeAmount the minimum value of miner staking
    function setMinStakeAmount(uint256 _minStakeAmount) external onlyOwner {
        minStakeAmount = _minStakeAmount;
    }

    /// --------------- Prover --------------- ///

    /// @notice Get total prover staking
    /// @param prover the prover address
    /// @return total prover staking amount
    function proverTotalStaking(address prover) external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();

        Staking storage st = proversStaking[prover].proverTotal;

        if (currentEpoch >= st.newEpoch) {
            return st.newValue;
        } else {
            return st.value;
        }
    }

    /// @notice Get prover staking by account
    /// @param prover the prover address
    /// @param account the staking account
    /// @return the staking amount of this account
    function proverStaking(address prover, address account) external view returns (uint256) {
        return proversStaking[prover].provers[account];
    }

    /// @notice Stake by prover self(others)
    /// @param prover the prover address
    /// @param amount new staking amount
    function proverStake(address prover, uint256 amount) external {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        // transfer from account

        IERC20(IAddresses(addresses).get(Contracts.Token)).transferFrom(msg.sender, address(this), amount);

        // add to prover stakers
        ProverStaking storage gs = proversStaking[prover];
        gs.provers[msg.sender] += amount;

        // add to total staking
        Staking storage st = gs.proverTotal;
        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue += amount;

        emit ProverStakeChange(st.newEpoch, prover, msg.sender, int256(amount), st.newValue);
    }

    /// @notice Unstake by prover self(others)
    /// @param prover the prover address
    /// @param amount the unstaking amount
    function proverUnstake(address prover, uint256 amount) external {
        ProverStaking storage gs = proversStaking[prover];
        require(gs.provers[msg.sender] >= amount, "S01");

        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();
        gs.provers[msg.sender] -= amount;

        // transfer to account
        IERC20(IAddresses(addresses).get(Contracts.Token)).transfer(msg.sender, amount);

        // remove from total staking
        Staking storage st = gs.proverTotal;
        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue -= amount;

        emit ProverStakeChange(st.newEpoch, prover, msg.sender, -int256(amount), st.newValue);
    }

    /// --------------- Miner --------------- ///

    /// @notice Get total miner staking
    /// @param prover the prover address
    /// @return the total miner staking amount
    function minerTotalStaking(address prover) external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        Staking storage st = proversStaking[prover].minerTotal;

        uint256 minersVesting = IVesting(IAddresses(addresses).get(Contracts.Vesting)).minersTotal();

        if (currentEpoch >= st.newEpoch) {
            return st.newValue + minersVesting;
        } else {
            return st.value + minersVesting;
        }
    }

    /// @notice Get miner staking
    /// @param prover the prover address
    /// @param account miner account
    /// @return the miner staking amount
    function minerStaking(address prover, address account) public view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        Staking storage st = proversStaking[prover].miners[account];

        uint256 minerVesting = IVesting(IAddresses(addresses).get(Contracts.Vesting)).miner(account);

        if (currentEpoch >= st.newEpoch) {
            return st.newValue + minerVesting;
        } else {
            return st.value + minerVesting;
        }
    }

    /// @notice Check account is miner or not
    /// @param prover the prover address
    /// @param account the checking account
    /// @return account is miner or not
    function isMiner(address prover, address account) external view returns (bool) {
        uint256 staking = minerStaking(prover, account);
        return staking >= minStakeAmount;
    }

    /// @notice Stake by miner
    /// @param prover the prover address
    /// @param amount the new staking amount
    function minerStake(address prover, uint256 amount) external {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        // transfer from account
        IERC20(IAddresses(addresses).get(Contracts.Token)).transferFrom(msg.sender, address(this), amount);

        ProverStaking storage gs = proversStaking[prover];

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

        emit MinerStakeChange(st.newEpoch, prover, msg.sender, int256(amount), st.newValue);
    }

    /// @notice Unstake by miner
    /// @param prover the prover address
    /// @param amount the unstaking amount
    function minerUnStake(address prover, uint256 amount) external {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        ProverStaking storage gs = proversStaking[prover];
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

        emit MinerStakeChange(st.newEpoch, prover, msg.sender, -int256(amount), st.newValue);
    }

    /// --------------- Player --------------- ///

    /// @notice Get total player staking
    /// @return the total staking amount of players
    function playerTotalStaking() external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        Staking storage st = playerTotal;

        if (currentEpoch >= st.newEpoch) {
            return st.newValue;
        } else {
            return st.value;
        }
    }

    /// @notice Get player staking
    /// @param account the player account
    /// @return the staking amount of player
    function playerStaking(address account) external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        Staking storage st = playersStaking[account];

        if (currentEpoch >= st.newEpoch) {
            return st.newValue;
        } else {
            return st.value;
        }
    }

    /// @notice Stake by player
    /// @param amount the new staking amount of player
    function playerStake(uint256 amount) external {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        // transfer from account
        IERC20(IAddresses(addresses).get(Contracts.Token)).transferFrom(msg.sender, address(this), amount);

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

    /// @notice Unstake by player
    /// @param amount the unstaking amount
    function playerUnStake(uint256 amount) external {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

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

    /// @notice Get claimable unstaking amount
    /// @param account the claiming account
    /// @return the amount which can claim now
    function claimable(address account) external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        Staking storage su = unstakings[account];

        if (currentEpoch >= su.newEpoch) {
            return su.value + su.newValue;
        } else {
            return su.value;
        }
    }

    /// @notice Claim unstaking to account
    /// @param account the claiming account
    function claim(address account) external {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();
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
        IERC20(IAddresses(addresses).get(Contracts.Token)).transfer(account, amount);
    }
}
