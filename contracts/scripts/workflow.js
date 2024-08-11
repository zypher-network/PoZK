const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");

//const myProver = "0x48a7fb14fd5711cf057bc7392973680231e8aebb";
const myProver = "0x3Aa5ebB10DC797CAC828524e59A333d0A371443c";
const ONE_TOKEN = 10000000000000000000n;

const ACCOUNT1 = new ethers.Wallet("0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d");
const ACCOUNT2 = new ethers.Wallet("0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a");
const ACCOUNT3 = new ethers.Wallet("0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6");

async function vesting() {
  const amount = ONE_TOKEN * 500000000n;
  const t = await attachContract("Token");
  const v = await attachContract("Vesting");
  const vAddress = await v.getAddress();
  //await t.transfer(vAddress, amount);
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
  let [account1, account2, account3, account4] = await ethers.getSigners();

  const t = await attachContract("Token");
  const c = await attachContract("Stake");
  const ca = await c.getAddress();
  await c.setMinStakeAmount(ONE_TOKEN * 100n);

  await t.approve(ca, ONE_TOKEN * 11100n);
  await c.proverStake(myProver, ONE_TOKEN * 10000n);
  await c.minerStake(myProver, ONE_TOKEN * 1000n);
  await c.playerStake(ONE_TOKEN * 100n);
  console.log("ACCOUNT1 stake ok");

  await t.connect(account1).transfer(account2.address, ONE_TOKEN * 1100n);
  await t.connect(account2).approve(ca, ONE_TOKEN * 1100n);
  await c.connect(account2).minerStake(myProver, ONE_TOKEN * 1000n);
  await c.connect(account2).playerStake(ONE_TOKEN * 100n);
  console.log("ACCOUNT2 stake ok");

  await t.connect(account1).transfer(account3.address, ONE_TOKEN * 1100n);
  await t.connect(account3).approve(ca, ONE_TOKEN * 1100n);
  await c.connect(account3).minerStake(myProver, ONE_TOKEN * 1000n);
  await c.connect(account3).playerStake(ONE_TOKEN * 100n);
  console.log("ACCOUNT3 stake ok");

  await t.connect(account1).transfer(account4.address, ONE_TOKEN * 1100n);
  await t.connect(account4).approve(ca, ONE_TOKEN * 1100n);
  await c.connect(account4).minerStake(myProver, ONE_TOKEN * 1000n);
  await c.connect(account4).playerStake(ONE_TOKEN * 100n);
  console.log("ACCOUNT4 stake ok");

  console.log("Stake set ok");
}

async function controller() {
  const c = await attachContract("Controller");
  const account = c.runner.address;
  await c.add(account);
  console.log("Controller set ok");
}

async function loopTask(times) {
  const accounts = await ethers.getSigners();

  const e = await attachContract("Epoch");
  // 2s/epoch
  await e.setPeriod(10);

  const c = await attachContract("TaskMarket");
  const account = c.runner.address;
  let tid = await c.nextId();
  console.log("Set epoch period to 2s");
  for(var i = 0; i < times; i++) {
    let account = accounts[i % 4];
    await c.connect(account).create(myProver, account, 0, "0x");
    //await sleep(6);
    await c.connect(account).accept(tid, account);
    //await c.accept(i, account);
    //await sleep(8);
    await c.connect(account).submit(tid, "0x", "0x");
    //await c.submit(i, "0x", "0x");
    //await sleep(10);
    tid += 1n;
    console.log("Next tid:", tid);
  }
}

async function reward() {
  const c = await attachContract("Epoch");
  const r = await attachContract("Reward");
  await c.getAndUpdate();
  const account = c.runner.address;

  await r.minerBatchCollect(0, account);
  await r.playerBatchCollect(0, account);

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
  //await vesting();
  //await prover();
  //await stake();
  //await controller();
  //await loopTask(10);
  await reward();
  //await unstake();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
