import { Address } from "viem";
import network from "@/constants/networks.json";

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
      Addresses: network.testnet.Addresses.address as Address,
      Token: network.testnet.Token.address as Address,
      Vesting: network.testnet.Vesting.address as Address,
      Epoch: network.testnet.Epoch.address as Address,
      Stake: network.testnet.Stake.address as Address,
      Reward: network.testnet.Reward.address as Address,
      Prover: network.testnet.Prover.address as Address,
      Task: network.testnet.Task.address as Address,
      Controller: network.testnet.Controller.address as Address,
  }
};

export const ChainRpcUrls: Record<ChainId, string[]> = {
  [ChainId.MAINNET]: ["https://rpc.zypher.network/"],
};
