const { ethers, upgrades, network, sleep } = require("hardhat");
const { randomBytes } = require("crypto");

async function timeTravel(seconds) {
  await network.provider.send('evm_increaseTime', [seconds]);
}

async function nextEpoch(epoch) {
  await timeTravel(60);
  await epoch.getAndUpdate();
}

async function mockTask(task, prover, miner, player) {
  let tid = await task.nextId();
  await task.create(prover, player, 0, "0x", "0x");
  await task.connect(miner).accept(tid, miner.address, "no-url");
  await task.submit(tid, randomBytes(32));
}

async function deployContract(name, params=[]) {
  const Factory = await ethers.getContractFactory(name);
  const contract = await Factory.deploy(...params);
  const address = await contract.getAddress();
  return [contract, address];
}

async function deployContractWithProxy(name, params=[]) {
  const Factory = await ethers.getContractFactory(name);
  const contract = await upgrades.deployProxy(Factory, params);
  await contract.waitForDeployment();
  const address = await contract.getAddress();
  return [contract, address];
}

async function deployAndSetupContracts(setup) {
  const [token, token_s] = await deployContract("Token", [20000]);
  const [addresses, addresses_s] = await deployContractWithProxy("Addresses", []);
  const [vesting, vesting_s] = await deployContractWithProxy("Vesting", [addresses_s, 100]);
  const [epoch, epoch_s] = await deployContractWithProxy("Epoch", [addresses_s, 60]);
  const [stake, stake_s] = await deployContractWithProxy("Stake", [addresses_s, 100]);
  const [reward, reward_s] = await deployContractWithProxy("Reward", [addresses_s, 1, 4, 1, 4, 1, 4, 90, 10, 10]);
  const [prover, prover_s] = await deployContractWithProxy("Prover", [addresses_s]);
  const [task, task_s] = await deployContractWithProxy("Task", [addresses_s, 10]); // 1/10 of miner staking
  const [controller, controller_s] = await deployContractWithProxy("Controller", [addresses_s]);

  const addressesContract = await ethers.getContractFactory("Addresses");
  const C = await addressesContract.attach(addresses);
  await C.batchSet(
    [
      0, // Contracts.Token,
      1, // Contracts.Vesting,
      2, // Contracts.Epoch,
      3, // Contracts.Stake,
      4, // Contracts.Reward,
      5, // Contracts.Prover,
      6, // Contracts.Task,
      7, // Contracts.Controller
    ],
    [
      token_s,
      vesting_s,
      epoch_s,
      stake_s,
      reward_s,
      prover_s,
      task_s,
      controller_s
    ]
  );

  let demo;
  if (setup) {
    const [runner] = await ethers.getSigners();

    // setup vesting
    const amount = 10000;
    await token.transfer(vesting_s, amount);
    await vesting.approveForReward(amount);

    // setup dao
    await epoch.addDao(runner.address, true);
    await epoch.setNetworkMode(2); // permissionless

    // setup prover
    const [demo1, demo_s] = await deployContract("DemoProver", []);
    demo = demo1;

    await prover.register(demo_s, 10000, 1, 10, demo_s);
    await prover.approve(demo_s, true, true);

    // setup staking
    await token.approve(stake_s, 300);
    await stake.proverStake(demo_s, 100);
    await stake.minerStake(demo_s, 100);
    await stake.playerStake(100);

    // create task
    await nextEpoch(epoch);
    await mockTask(task, demo_s, runner, runner.address);

    // sleep to next epoch
    await nextEpoch(epoch);
  }

  return {
    token: token,
    addresses: addresses,
    vesting: vesting,
    epoch: epoch,
    stake: stake,
    reward: reward,
    prover: prover,
    task: task,
    controller: controller,
    demo: demo
  };
}

module.exports = {
  timeTravel,
  nextEpoch,
  mockTask,
  deployContract,
  deployAndSetupContracts
};
