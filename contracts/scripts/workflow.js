const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");

const myProver = "0x3Aa5ebB10DC797CAC828524e59A333d0A371443c";
const ONE_TOKEN = 10000000000000000000n;

async function vesting() {
  const amount = ONE_TOKEN * 1000000n;
  const t = await attachContract("Token");
  const v = await attachContract("Vesting");
  const vAddress = await v.getAddress();
  await t.transfer(vAddress, amount);
  await v.approveForReward(amount);
  console.log("Vesting set ok");
}

async function prover() {
  const c = await attachContract("ProverMarket");
  await c.register(myProver, 10000, 1, 10, myProver);
  await c.approve(myProver, true, true);
  console.log("Prover set ok");
}

async function stake() {
  const t = await attachContract("Token");
  const c = await attachContract("Stake");
  const ca = await c.getAddress();
  await c.setMinStakeAmount(ONE_TOKEN * 100n);
  await t.approve(ca, ONE_TOKEN * 11100n);
  await c.proverStake(myProver, ONE_TOKEN * 10000n);
  await c.minerStake(myProver, ONE_TOKEN * 1000n);
  await c.playerStake(ONE_TOKEN * 100n);
  console.log("Stake set ok");
}

async function controller() {
  const c = await attachContract("Controller");
  const account = c.runner.address;
  await c.add(account);
  console.log("Controller set ok");
}

async function loopTask(times) {
  const e = await attachContract("Epoch");
  // 2s/epoch
  await e.setPeriod(2);

  const c = await attachContract("TaskMarket");
  const account = c.runner.address;
  let tid = await c.nextId();
  console.log("Set epoch period to 2s");
  for(var i = 0; i < times; i++) {
    await c.create(myProver, account, 0, "0x");
    await c.accept(tid, account);
    await c.submit(tid, "0x", "0x");
    console.log("Task OK: ", i);
    sleep(2);
    tid += 1n;
  }
}

async function reward() {
  const c = await attachContract("Epoch");
  const r = await attachContract("Reward");
  await c.getAndUpdate();
  const account = c.runner.address;

  await r.minerBatchCollect(4, account);
  await r.playerBatchCollect(4, account);

  console.log("Reward set ok");
}

async function unstake() {
  const c = await attachContract("Stake");
  await c.proverUnstake(myProver, ONE_TOKEN * 1000n);
  await c.minerUnstake(myProver, ONE_TOKEN * 100n);
  await c.playerUnstake(ONE_TOKEN * 10n);
  console.log("Unstake set ok");
}

async function main() {
  await vesting();
  await prover();
  await stake();
  await controller();
  await loopTask(10);
  await reward();
  await unstake();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
