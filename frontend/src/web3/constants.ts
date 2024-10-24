import { Address } from "viem";

export enum ChainId {
  TESTNET = "9901", // "19546", // zytron testnet
  MAINNET = "9901" // zytron mainnet
}

export const CHAINID = ChainId.MAINNET; // default network

type IContractList = {
  Addresses: Address;
  Token: Address;
  Vesting: Address;
  Epoch: Address;
  Stake: Address;
  Reward: Address;
  Prover: Address;
  Task: Address;
  Controller: Address;
};
export const contractAddress: Record<ChainId, IContractList> = {
  [ChainId.MAINNET]: {
      Addresses: "0xfE8595EdE3DA06936f14C14eE7A475490E33a59d",
      Token: "0x07cb9813aEa7678B70D7Ae418A7d8680E432ea2e",
      Vesting: "0x81DB6405eDb330D4cBe4B2279397A5677010bD65",
      Epoch: "0x5393205CdAb1C4f45B045F10c07B29bFf2339b81",
      Stake: "0xa8c392Fd78fd61292529864873cD42309881903f",
      Reward: "0x93CF679371fCca140275fd5b974487c87c5bAAAb",
      Prover: "0x7937D8A26F867Ec563dC324Aca279Ce41e2eBDb9",
      Task: "0xcc8240470F63D0b8055bf03f44D8817dAfCa0DB7",
      Controller: "0xB7779c7af4727A1312fF0d99a0869F155d2F7753",
  }
};
export const gamesList = {
  [ChainId.MAINNET]: {
    game1: {
      address: "0x5b92b011513f9aaf8f6541003dc088625e7438e8=" as Address,
      version: "1",
    },
  }
};
export const ChainRpcUrls: Record<ChainId, string[]> = {
  [ChainId.MAINNET]: ["https://rpc.zypher.network/"],
};
