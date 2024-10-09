const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");

const myProver = "0x322813Fd9A801c5507c9de605d63CEA4f2CE6c44"; // localhsot
//const myProver = "0x5b92b011513f9aaf8f6541003dc088625e7438e8";
const ONE_TOKEN = 10000000000000000000n;

const ACCOUNT1 = new ethers.Wallet("0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d");
const ACCOUNT2 = new ethers.Wallet("0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a");
const ACCOUNT3 = new ethers.Wallet("0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6");

async function vesting() {
  // const amount = ONE_TOKEN * 500000000n; // mainnet
  const amount = ONE_TOKEN * 100000000n;
  const [t, _] = await attachContract("Token");
  const [v, _1] = await attachContract("Vesting");
  const vAddress = await v.getAddress();
  await t.transfer(vAddress, amount);
  await sleep();
  await v.approveForReward(amount);
  console.log("Vesting set ok");
}

async function dao() {
  const [e, _] = await attachContract("Epoch");
  await e.addDao(e.runner.address, true);
  console.log("DAO set ok");
}

async function prover() {
  const [c, _] = await attachContract("Prover");
  await c.register(myProver, 10000, 1, 20, myProver);
  await sleep();
  await c.approve(myProver, true, true);
  console.log("Prover set ok");
}

async function stakeWithTest(auto) {
  const [e, _e] = await attachContract("Epoch");
  await e.setNetworkMode(1); // 1 is permissioned
  const account = e.runner.address;
  console.log("Network mode to permissioned");

  const [t, _t] = await attachContract("Token");
  const [c, _c] = await attachContract("Stake");
  const ca = await c.getAddress();

  await t.approve(ca, ONE_TOKEN * 1000n);
  await sleep();
  console.log(await t.allowance(account, ca));
  await c.minerStake(myProver, ONE_TOKEN * 1000n);
  console.log("ACCOUNT1 stake ok");
  await sleep();

  await c.minerTest(
    1,
    "0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000016000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000003c0cf3cc8f230c8f0cf3ff0ef3c3330000000000000000000000000000000000000000000000000000000000001a8500000000000000000000000000000000000000000000000000000000000001c80000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003c00000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000200800000000000000000000000000000000000000000000000000000088600444000050002300000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000003c0cf3cc8f230c8f0cf3ff0ef3c3330000000000000000000000000000000000000000000000000000000000001a8500000000000000000000000000000000000000000000000000000000000001c80000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003c000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000002008000000000000000000000000000000000000000000000000000000886004440000500023",
    "0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000200800000000000000000000000000000000000000000000000000000088600444000050002300000000000000000000000000000000003c0cf3cc8f230c8f0cf3ff0ef3c3330000000000000000000000000000000000000000000000000000000000001a850000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003c00000000000000000000000000000000000000000000000000000000000001c80000000000000000000000000000000000000000000000000000200800000000000000000000000000000000000000000000000000000088600444000050002300000000000000000000000000000000003c0cf3cc8f230c8f0cf3ff0ef3c3330000000000000000000000000000000000000000000000000000000000001a850000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003c00000000000000000000000000000000000000000000000000000000000001c8"
  );
  console.log("MinerTest created");

  if (auto) {
    await c.minerTestSubmit(
      1,
      false,
      "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002245328eca4166f23a16d9cbfb401b94cd5c18637592c6b03f93a6085a025e29421bae53ff926f8e5d0d6d0cad2b8b02c8974d7780a6cb3b72bde92db48c96e5b2b84b3b91cb7663e49b872887d44bbeccc6842b0377d9515d3ec327d81a3f6b310ac2dfdc7356148f2392f6d0595786522f36371872569cd99bf4c889afa0041240c2dba7ebfc59e4618c1f92ed4fd19b0d62b26514ee52bcde296f1cd41c05b0eaf366b53d5d17087c56db0ea5ae501b63deb2e56ef23cc57fc367d5c4618292d14ec991d5223d1c7dd9f6551c223e575a1c179e6b72902bef260669135b4262920db933e7c36ded75bc72cc3213def73546db02366ef0adaf6c047a90005a62ad3672cca87ef2f760d8c21c1e5927550386c6077b1d04ba8a9939cb1c98aca0c2b77a766e72dff48c9dc35dc5f8c2a891656e65c9d762362fc160ca4942e1105a3aac14877ab83a478ebc973a55805c3ddbe288ae7fe908b7e0d2378f5e84f0cbd70aec089310c111b6b7e0f628012cf4e754b85e3fa754df12637ce9971fe05a2d54342b86331fde7213379db5431a8c29f37b457da0db493eada3b183ea11eacb93637a7345ea5fd56c75f8eddcce0553384410b06eca97f3434a1b89d4c0c9b2090d8124850c6cda0d4af29baa6a8b29978ae69d6f8969e3a5536ef250e031e0eca90d1c6b8f5c2f3028ce2335bc30d051f0d95eb52d89b41c4a0b45875"
    );
  }
}

async function stake() {
  let [account1, account2, account3, account4] = await ethers.getSigners();

  const [t, _] = await attachContract("Token");
  const [c, _1] = await attachContract("Stake");
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
  const [c, _] = await attachContract("Controller");
  const account = c.runner.address;
  await c.add(account);
  console.log("Controller set ok");
}

async function loopTask(times) {
  const accounts = await ethers.getSigners();

  const [e, _] = await attachContract("Epoch");
  // 2s/epoch
  await e.setPeriod(10);

  const [c, _1] = await attachContract("Task");
  const account = c.runner.address;
  let tid = await c.nextId();
  console.log("Set epoch period to 2s");
  for(var i = 0; i < times; i++) {
    let account = accounts[i % 4];
    await c.connect(account).create(myProver, account, 0, "0x");
    //await sleep();
    await c.connect(account).accept(tid, account);
    //await c.accept(i, account);
    //await sleep();
    await c.connect(account).submit(tid, "0x", "0x");
    //await c.submit(i, "0x", "0x");
    //await sleep();
    tid += 1n;
    console.log("Next tid:", tid);
  }
}

async function reward() {
  const [c, _] = await attachContract("Epoch");
  const [r, _1] = await attachContract("Reward");
  await c.getAndUpdate();
  const account = c.runner.address;

  await r.minerBatchCollect(0, account);
  await r.playerBatchCollect(0, account);

  console.log("Reward set ok");
}

async function unstake() {
  const [c, _] = await attachContract("Stake");
  await c.proverUnstake(myProver, ONE_TOKEN * 1000n);
  await c.minerUnstake(myProver, ONE_TOKEN * 100n);
  await c.playerUnstake(ONE_TOKEN * 10n);
  console.log("Unstake set ok");
}

async function main() {
  //await dao();
  await vesting();
  //await prover();
  await stakeWithTest(false);
  //await stake();
  //await controller();
  //await loopTask(10);
  //await reward();
  //await unstake();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
