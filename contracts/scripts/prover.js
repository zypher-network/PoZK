const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");

const ONE_TOKEN = 10000000000000000000n;

// Testnet
// 2048
// const myProver = "0x01156bb7c9b003ce2e2d2b0ee934b2baf196b08c";
// const myVerifier = "0x01156bb7c9b003ce2e2d2b0ee934b2baf196b08c";
// const WORK = 10000;

// cr
const myProver = "0x6efdab245fb1905b3692c8e0c8702cec13a17121";
const myVerifier = "0x6efdab245fb1905b3692c8e0c8702cec13a17121";
const WORK = 5000;

// Shuffle20
// const myProver = "0xa6b720ee1f8975551a94f2d6bea74978aff60343";
// const myVerifier = "0xcfb2ac0013d3bdd186a21babf8c170b5b560e58d";
// const WORK = 5000;

// Shuffle52
// const myProver = "0xbc9b4e9d43830f747e65873a5e122ddd9c9d769b";
// const myVerifier = "0x17c3aef40495c2fcc9bc1880aeaaaf455fdfa5be";
// const WORK = 10000;

// Competition 1 & 2
// const myProver = "0x432d35f3717f195070c450f471311a221ef275cd";
// const myVerifier = "0x432D35F3717f195070C450F471311A221EF275Cd";
// const WORK = 5000;
// const myProver = "0xf227ab39cab4d4fbfb70390a46831d060c271dd5";
// const myVerifier = "0xf227AB39cAB4D4fBfb70390a46831D060C271Dd5";
// const WORK = 20000;

async function registerProver() {
  const [c, _c] = await attachContract("Prover");
  const [e, _e] = await attachContract("Epoch");
  console.log("Starting register prover");
  await c.register(myProver, 0, WORK, 1, 20, myVerifier);
  console.log("Waiting approve prover");
  await sleep();
  await c.approve(myProver, true, true);
  console.log("Waiting update epoch");
  //await sleep();
  // await e.getAndUpdate();
  // await sleep();
  console.log("Prover setted");
}

async function upgradeProver() {
  const [c, _c] = await attachContract("Prover");
  const [e, _e] = await attachContract("Epoch");
  console.log("Starting upgrade prover");
  await c.upgrade(myProver, 0, WORK, 1, 20, myVerifier);
  console.log("Waiting approve prover");
  await sleep();
  await c.approve(myProver, true, true);
  console.log("Prover setted");
}

async function stakeProver() {
  const [t, _] = await attachContract("Token");
  const [c, _1] = await attachContract("Stake");
  const ca = await c.getAddress();

  await t.approve(ca, ONE_TOKEN * 1000n);
  await sleep();
  await c.proverStake(myProver, ONE_TOKEN * 1000n);

  console.log("Prover staked");
}

async function stopProver() {
  const [c, _] = await attachContract("Prover");
  await c.stop(myProver);
  console.log("Prover stopped");
}

async function main() {
  await registerProver();
  await stakeProver();
  //await upgradeProver();
  //await stopProver();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
