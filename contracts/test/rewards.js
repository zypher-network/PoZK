const { ethers } = require("hardhat");
const { expect } = require("chai");
const { deployAndSetupContracts, timeTravel, nextEpoch, mockTask, deployContract } = require("./utils");

describe("Rewards", function () {
  let wallet_0, wallet_1, wallet_2, wallet_3;
  let token;
  let epoch;
  let prover;
  let stake;
  let stake_s;
  let task;
  let reward;
  let demo;
  let demo_s;

  const checkClaim = async function(e, a, r1, r2) {
    await reward.minerBatchCollect(e, a);
    await nextEpoch(epoch);
    const reward1 = Number(await stake.claimable(a));
    expect(reward1).to.equal(r1);
    await reward.playerBatchCollect(e, a);
    await nextEpoch(epoch);
    const reward2 = Number(await stake.claimable(a));
    expect(reward2).to.equal(r2);

    if (reward2 > 0) {
      await stake.claim(a);
    }
  };

  const checkProverClaim = async function(e, p, a, r1, r2) {
    await reward.minerCollect(e, p, a);
    await nextEpoch(epoch);
    const reward1 = Number(await stake.claimable(a));
    expect(reward1).to.equal(r1);
    await reward.playerCollect(e, p, a);
    await nextEpoch(epoch);
    const reward2 = Number(await stake.claimable(a));
    expect(reward2).to.equal(r2);

    if (reward2 > 0) {
      await stake.claim(a);
    }
  };

  beforeEach(async () => {
    [wallet_0, wallet_1, wallet_2, wallet_3] = await ethers.getSigners();
    const contracts = await deployAndSetupContracts(true);;
    token = contracts.token;
    epoch = contracts.epoch;
    prover = contracts.prover;
    stake = contracts.stake;
    stake_s = await stake.getAddress();
    task = contracts.task;
    reward = contracts.reward;
    demo = contracts.demo;
    demo_s = await demo.getAddress();
  });

  describe("Rewards 1p:1m:1p", function () {
    it.skip("Should get reward when deploy & setup", async function () {
      // miner: 10% ~ 90%, number = 1. y = (x - 1)  / (t - 1) * (p - q) + q = y => x * (p - q) / t + q
      // miner: x * (p - q) / t + q = 1 * (90 - 10) / 10 + 10 = 18
      // player: 82
      const currentEpoch = Number(await epoch.get());
      await checkClaim(currentEpoch - 1, wallet_0.address, 18, 100);

      // task to 5
      const epoch5 = Number(await epoch.get());
      for (i = 0; i < 5; i++) {
        await mockTask(task, demo_s, wallet_0, wallet_0.address);
      }
      await nextEpoch(epoch);
      await checkClaim(epoch5, wallet_0.address, 50, 100);

      // task to 10
      const epoch10 = Number(await epoch.get());
      for (i = 0; i < 10; i++) {
        await mockTask(task, demo_s, wallet_0, wallet_0.address);
      }
      await nextEpoch(epoch);
      await checkClaim(epoch10, wallet_0.address, 90, 100);

      // task to 11
      const epoch11 = Number(await epoch.get());
      for (i = 0; i < 11; i++) {
        await mockTask(task, demo_s, wallet_0, wallet_0.address);
      }
      await nextEpoch(epoch);
      await checkClaim(epoch11, wallet_0.address, 90, 100);
    });
  });

  describe("Rewards p:m:p", function () {
    beforeEach(async () => {
      const reward1 = Number(await stake.claimable(wallet_0.address));
      if (reward1 > 0) {
        await stake.claim(wallet_0.address);
      }
    });

    it.skip("Rewards with 1p:1m:2p", async function () {
      const epoch1 = Number(await epoch.get());
      await mockTask(task, demo_s, wallet_0, wallet_0.address);
      await mockTask(task, demo_s, wallet_0, wallet_1.address);
      await nextEpoch(epoch);
      await checkClaim(epoch1, wallet_0.address, 26, 100);

      await token.transfer(wallet_1.address, 100);
      await token.transfer(wallet_2.address, 200);
      await token.connect(wallet_1).approve(stake_s, 100);
      await stake.connect(wallet_1).playerStake(100);
      await token.connect(wallet_2).approve(stake_s, 200);
      await stake.connect(wallet_2).playerStake(200);
      await nextEpoch(epoch);

      const epoch2 = Number(await epoch.get());
      await mockTask(task, demo_s, wallet_0, wallet_1.address);
      await mockTask(task, demo_s, wallet_0, wallet_1.address);
      await mockTask(task, demo_s, wallet_0, wallet_2.address);
      await nextEpoch(epoch);
      await checkClaim(epoch2, wallet_0.address, 34, 34); // no player reward, remain 66
      await checkClaim(epoch2, wallet_1.address, 0, 26);  // same staking: 35 + 20 = 64
      await checkClaim(epoch2, wallet_2.address, 0, 36);  // work weight > staking: 26 + 36 = 62
    });

    it.skip("Rewards with 1p:2m:1p", async function () {
      await token.transfer(wallet_1.address, 100);
      await token.connect(wallet_1).approve(stake_s, 100);
      await stake.connect(wallet_1).minerStake(demo_s, 100);
      await nextEpoch(epoch);

      const epoch1 = Number(await epoch.get());
      await mockTask(task, demo_s, wallet_0, wallet_0.address);
      await mockTask(task, demo_s, wallet_1, wallet_0.address);
      await nextEpoch(epoch);
      await checkClaim(epoch1, wallet_0.address, 13, 87);
      await checkClaim(epoch1, wallet_1.address, 13, 13); // 13 + 13 = 26

      await token.approve(stake_s, 100);
      await stake.minerStake(demo_s, 100);
      await nextEpoch(epoch);

      const epoch2 = Number(await epoch.get());
      await mockTask(task, demo_s, wallet_0, wallet_0.address);
      await mockTask(task, demo_s, wallet_1, wallet_0.address);
      await mockTask(task, demo_s, wallet_1, wallet_0.address);
      await nextEpoch(epoch);
      await checkClaim(epoch2, wallet_0.address, 19, 85); // 16(+3), 90(-5)
      await checkClaim(epoch2, wallet_1.address, 13, 13); // 9(+4) 16 + 9 = 25
    });

    it("Rewards with 2p:1m:1p", async function () {
      const [_, demo2_s] = await deployContract("DemoProver", []);
      await prover.register(demo2_s, 10000, 1, 10, demo2_s);
      await prover.approve(demo2_s, true, true);
      await token.approve(stake_s, 200);
      await stake.minerStake(demo2_s, 100);
      await nextEpoch(epoch);

      // no staking
      const epoch1 = Number(await epoch.get());
      await mockTask(task, demo_s, wallet_0, wallet_0.address);
      await mockTask(task, demo2_s, wallet_0, wallet_0.address);
      await nextEpoch(epoch);
      await checkProverClaim(epoch1, demo_s, wallet_0.address, 18, 100);
      await checkProverClaim(epoch1, demo2_s, wallet_0.address, 0, 0);

      // with staking
      await stake.proverStake(demo2_s, 100);
      await nextEpoch(epoch);

      const epoch2 = Number(await epoch.get());
      await mockTask(task, demo_s, wallet_0, wallet_0.address);
      await mockTask(task, demo2_s, wallet_0, wallet_0.address);
      await nextEpoch(epoch);
      // await checkClaim(epoch2, wallet_0.address, 18, 100);
      await checkProverClaim(epoch2, demo_s, wallet_0.address, 9, 50);
      await checkProverClaim(epoch2, demo2_s, wallet_0.address, 9, 50);
    });
  });
});
