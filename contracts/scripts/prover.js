const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");

const ONE_TOKEN = 10000000000000000000n;

// Testnet (opbnbtestnet)
// 2048
const myProver = "0x0eaca2011742c5156f217f1b1d0784fe5fbf2428";
const myVerifier = "0x0eaca2011742c5156f217f1b1d0784fe5fbf2428";

// cr
// const myProver = "0xbc9b4e9d43830f747e65873a5e122ddd9c9d769b";
// const myVerifier = "0xbc9b4e9d43830f747e65873a5e122ddd9c9d769b";

// Shuffle20
// const myProver = "0x6708d16d1197b4a68df93a27c785208de7819e1e";
// const myVerifier = "0xe2fc3851169c692a1b8b5654d1d58919c7198015";

// Shuffle52
// const myProver = "0x3e7e24fec0f9c6ce2ca63a1a4d829ff5fdfa3423";
// const myVerifier = "0x4f447d512776fc467d4b16ddabcec76a2707d122";

// Competition 1 & 2
// const myProver = "0x1248e1031c4d81a678c63811d7bf714b1a18220b";
// const myVerifier = "0x1248e1031c4d81a678c63811d7bf714b1a18220b";
// const myProver = "0x9b0b9bfcd3e1e3715bead7639d93b9c87a74b32a";
// const myVerifier = "0x9b0b9bfcd3e1e3715bead7639d93b9c87a74b32a";

async function registerProver() {
  const [c, _c] = await attachContract("Prover");
  const [e, _e] = await attachContract("Epoch");
  await c.register(myProver, 10000, 1, 20, myVerifier);
  // await c.upgrade(myProver, 5000, 1, 20, myVerifier);
  console.log("Waiting approve prover");
  await sleep();
  await c.approve(myProver, true, true);
  console.log("Waiting update epoch");
  await sleep();
  await e.getAndUpdate();
  await sleep();
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
  //await stopProver();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
