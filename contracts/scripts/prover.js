const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");

const ONE_TOKEN = 10000000000000000000n;

// 2048
// const myProver = "0x5b92b011513f9aaf8f6541003dc088625e7438e8";
// const myVerifier = "0x5b92b011513f9aaf8f6541003dc088625e7438e8";

// cr
// const myProver = "0xef1e764c386ec95ed233035661dd4269be8fd8e7";
// const myVerifier = "0xef1e764c386ec95ed233035661dd4269be8fd8e7";

// Shuffle20
// const myProver = "0xfb530825bC8edCA2b13597F3B2b91310d43099a1";
// const myVerifier = "0x407441d85e8F54772f84Ac1f47570C7Cf6Dac080";

// Shuffle52
// const myProver = "0x6558c36b5736466c472231A26A4B47512Bd936Da";
// const myVerifier = "0xc90459cB8a9Ab5EFCd5aEe271f3F343DA4a3eDBE";

// const myProver = "0xb216af68a82538ff12edc8ac9eec3e91eaa54e9e";
const myProver = "0x614e0cccba48c2bb4da3f05704874f80e3a551d5";
const myVerifier = "0x614e0cccba48c2bb4da3f05704874f80e3a551d5";

async function registerProver() {
  const [c, _] = await attachContract("Prover");
  await c.register(myProver, 10000, 1, 10, myVerifier);
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
  //await registerProver();
  await stakeProver();
  //await stopProver();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
