const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");

const ONE_TOKEN = 10000000000000000000n;

// Testnet
// 2048
const myProver = "0x8acf473885f975023e7a9b7c043da53d690cbc6e";
const myVerifier = "0x8aCF473885f975023e7A9B7C043Da53D690cbc6E";
const WORK = 10000;

// cr
// const myProver = "0x66e9ce70bb3431958e0ce352d1b5d85772e57e06";
// const myVerifier = "0x66e9CE70bb3431958e0CE352d1B5D85772E57E06";
// const WORK = 5000;

// Shuffle20
// const myProver = "0x0eaca2011742c5156f217f1b1d0784fe5fbf2428";
// const myVerifier = "0x33682F75895E986546A09D60F7ef5Ee6a53383d8";
// const WORK = 5000;

// Shuffle52
// const myProver = "0xa6b720ee1f8975551a94f2d6bea74978aff60343";
// const myVerifier = "0xCFB2AC0013d3bDD186A21BABf8c170b5b560e58d";
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
  await c.register(myProver, WORK, 1, 20, myVerifier, false);
  // await c.upgrade(myProver, WORK, 1, 20, myVerifier, false);
  console.log("Waiting approve prover");
  await sleep();
  await c.approve(myProver, true, true);
  console.log("Waiting update epoch");
  //await sleep();
  // await e.getAndUpdate();
  // await sleep();
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
