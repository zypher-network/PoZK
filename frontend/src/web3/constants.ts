import { Address } from "viem";
import network from "@/constants/networks.json";

export enum ChainId {
  TESTNET = "5611", // opbnb testnet
  MAINNET = "9901" // zytron mainnet
}

export const CHAINID = ChainId.TESTNET; // default network

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
  [ChainId.TESTNET]: {
      Addresses: network.testnet.Addresses.address as Address,
      Token: network.testnet.Token.address as Address,
      Vesting: network.testnet.Vesting.address as Address,
      Epoch: network.testnet.Epoch.address as Address,
      Stake: network.testnet.Stake.address as Address,
      Reward: network.testnet.Reward.address as Address,
      Prover: network.testnet.Prover.address as Address,
      Task: network.testnet.Task.address as Address,
      Controller: network.testnet.Controller.address as Address,
  },
  [ChainId.MAINNET]: {
      Addresses: network.mainnet.Addresses.address as Address,
      Token: network.mainnet.Token.address as Address,
      Vesting: network.mainnet.Vesting.address as Address,
      Epoch: network.mainnet.Epoch.address as Address,
      Stake: network.mainnet.Stake.address as Address,
      Reward: network.mainnet.Reward.address as Address,
      Prover: network.mainnet.Prover.address as Address,
      Task: network.mainnet.Task.address as Address,
      Controller: network.mainnet.Controller.address as Address,
  }
};

export const ChainRpcUrls: Record<ChainId, string[]> = {
  [ChainId.TESTNET]: ["https://opbnb-testnet-rpc.bnbchain.org"],
  [ChainId.MAINNET]: ["https://rpc.zypher.network/"],
};

export const SubgraphUrls: Record<ChainId, string> = {
  [ChainId.TESTNET]: "https://pozk-subgraph.zypher.dev/subgraphs/name/testnet/pozk/",
  [ChainId.MAINNET]: "https://pozk-subgraph.zypher.dev/subgraphs/name/testnet/pozk/",
}

export const ZeroGasUrls: Record<ChainId, string> = {
    [ChainId.TESTNET]: "https://gas.zypher.dev",
    [ChainId.MAINNET]: "https://gas.zypher.network",
}
