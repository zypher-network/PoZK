import { Address } from "viem";

export enum ChainId {
  OPBNBTEST = "5611",
}
export const CHAINID = ChainId.OPBNBTEST;
type IContractList = {
  Addresses: Address;
  Token: Address;
  Vesting: Address;
  Epoch: Address;
  Stake: Address;
  Reward: Address;
  ProverMarket: Address;
  TaskMarket: Address;
  Controller: Address;
};
export const contractAddress: Record<ChainId, IContractList> = {
  [ChainId.OPBNBTEST]: {
    Addresses: "0x86f23Be54493B1BF89eae9322ED3A78B7d6D3497",
    Token: "0x5211e2c5Bc985caB4755c939eF94FDF28355A5EA",
    Vesting: "0x72E8C1C2c5eCA1Bc145efD61d03E8428C93d050b",
    Epoch: "0xA6b210712135d208bB7545f935a8F15d7b6CBd2e",
    Stake: "0x003C1F8F552EE2463e517FDD464B929F8C0bFF06",
    Reward: "0xD4f85C18434E332A26A90C20EEB3b6bdb1dA1b1a",
    ProverMarket: "0x1c23e9F06b10f491e86b506c025080C96513C9f5",
    TaskMarket: "0x27DE7777C1c643B7F3151F7e4Bd3ba5dacc62793",
    Controller: "0xB362781Da5dbF9EeE57dc15FC0F526Ac83a9eD74",
  },
};
export const gamesList = {
  [ChainId.OPBNBTEST]: {
    game1: {
      address: "0x48a7fb14fd5711cf057bc7392973680231e8aebb" as Address,
      version: "1",
    },
  },
};
export const ChainRpcUrls: Record<ChainId, string[]> = {
  [ChainId.OPBNBTEST]: ["https://opbnb-testnet-rpc.bnbchain.org/"],
};
