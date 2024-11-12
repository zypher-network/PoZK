const { ethers } = require("hardhat");
const { expect } = require("chai");
const { deployAndSetupContracts, timeTravel, nextEpoch, mockTask, deployContract } = require("./utils");

describe("Dispute", function () {
  let wallet_0, wallet_1, wallet_2, wallet_3;
  let token;
  let epoch;
  let stake;
  let task;
  let reward;
  let demo;
  let demo_s;
  let tid;
  let disputeDeposit;

  beforeEach(async () => {
    [wallet_0, wallet_1, wallet_2, wallet_3] = await ethers.getSigners();
    const contracts = await deployAndSetupContracts(true);;
    token = contracts.token;
    epoch = contracts.epoch;
    stake = contracts.stake;
    task = contracts.task;
    reward = contracts.reward;
    demo = contracts.demo;

    const task_s = await task.getAddress();
    demo_s = await demo.getAddress();
    disputeDeposit = Number(await task.disputeDeposit());

    tid = await task.nextId();
    await task.create(demo_s, wallet_1.address, 0, "0x", "0x");
    await task.accept(tid, wallet_0.address, "");

    await expect(task.connect(wallet_2).dispute(tid)).to.be.revertedWith("T07");
    await token.transfer(wallet_1.address, disputeDeposit);
    await token.connect(wallet_1).approve(task_s, disputeDeposit);
    await task.connect(wallet_1).dispute(tid);

    await epoch.addDao(wallet_2.address, true);
  });

  describe("Different dispute", function () {
    it("Player win (give dao) and miner with slash", async function () {
      const b1 = Number(await token.balanceOf(wallet_1.address));
      await task.connect(wallet_2).adjudicate(tid, disputeDeposit - 5, 0, true);
      const b2 = Number(await token.balanceOf(wallet_1.address));
      expect(b2).to.equal(b1 + disputeDeposit - 5);
      const b22 = Number(await token.balanceOf(wallet_2.address));
      expect(b22).to.equal(5);

      const u1 = Number(await stake.claimable(wallet_1.address));
      expect(u1).to.equal(0);
      const s1 = Number(await stake.minerStaking(demo_s, wallet_0.address));

      await nextEpoch(epoch);

      const u2 = Number(await stake.claimable(wallet_1.address));
      expect(u2).to.equal(disputeDeposit);
      const s2 = Number(await stake.minerStaking(demo_s, wallet_0.address));
      expect(s2).to.equal(s1 - disputeDeposit);
    });

    it("Player win and miner with slash unstaking", async function () {
      const s0 = Number(await stake.minerStaking(demo_s, wallet_0.address));
      await stake.minerUnstake(demo_s, s0);

      const b1 = Number(await token.balanceOf(wallet_1.address));
      await task.connect(wallet_2).adjudicate(tid, disputeDeposit, 0, true);
      const b2 = Number(await token.balanceOf(wallet_1.address));
      expect(b2).to.equal(b1 + disputeDeposit);

      await nextEpoch(epoch);
      const u2 = Number(await stake.claimable(wallet_1.address));
      expect(u2).to.equal(disputeDeposit);
      const s2 = Number(await stake.claimable(wallet_0.address));
      expect(s2).to.equal(s0 - disputeDeposit);
    });

    it("Player win and miner with slash unstaking not enough", async function () {
      await stake.setMinStakeAmount(1);
      const s0 = Number(await stake.minerStaking(demo_s, wallet_0.address));
      await stake.minerUnstake(demo_s, s0 - disputeDeposit + 2); // 100 - 10 + 2 = 92, remain 8
      await nextEpoch(epoch);
      await stake.claim(wallet_0.address);
      await stake.minerUnstake(demo_s, disputeDeposit - 3); // unstaking 7, remain 1

      const b1 = Number(await token.balanceOf(wallet_1.address));
      await task.connect(wallet_2).adjudicate(tid, disputeDeposit, 0, true);
      const b2 = Number(await token.balanceOf(wallet_1.address));
      expect(b2).to.equal(b1 + disputeDeposit);

      await nextEpoch(epoch);
      const u2 = Number(await stake.claimable(wallet_1.address));
      expect(u2).to.equal(disputeDeposit - 2); // staking 1 + unstaking 7
      const s2 = Number(await stake.claimable(wallet_0.address));
      expect(s2).to.equal(0);
      const s22 = Number(await stake.minerStaking(demo_s, wallet_0.address));
      expect(s2).to.equal(0);
    });

    it("Player win but miner no slash (no one win)", async function () {
      const b1 = Number(await token.balanceOf(wallet_1.address));
      const s1 = Number(await stake.minerStaking(demo_s, wallet_0.address));

      await task.connect(wallet_2).adjudicate(tid, disputeDeposit, 0, false);
      await nextEpoch(epoch);

      const b2 = Number(await token.balanceOf(wallet_1.address));
      expect(b2).to.equal(b1 + disputeDeposit);
      const s2 = Number(await stake.minerStaking(demo_s, wallet_0.address));
      expect(s2).to.equal(s1);
    });

    it("Miner win", async function () {
      const stake_s = await stake.getAddress();
      await token.transfer(wallet_2.address, 100);
      await token.connect(wallet_2).approve(stake_s, 100);
      await stake.connect(wallet_2).playerStake(100);
      await nextEpoch(epoch);

      const b1 = Number(await token.balanceOf(wallet_1.address));
      const s1 = Number(await token.balanceOf(wallet_0.address));

      await task.connect(wallet_2).adjudicate(tid, 0, disputeDeposit - 5, false);
      const epoch1 = Number(await epoch.get());

      const b2 = Number(await token.balanceOf(wallet_1.address));
      expect(b2).to.equal(b1);
      const s2 = Number(await token.balanceOf(wallet_0.address));
      expect(s2).to.equal(s1 + disputeDeposit - 5);

      // check reward
      await nextEpoch(epoch);
      await reward.playerBatchCollect(epoch1, wallet_2.address);
      await reward.minerBatchCollect(epoch1, wallet_0.address);
      await nextEpoch(epoch);
      const reward1 = Number(await stake.claimable(wallet_2.address));
      const reward2 = Number(await stake.claimable(wallet_0.address));
      expect(reward1).to.equal(82);
      expect(reward2).to.equal(18);
    });
  });
});
