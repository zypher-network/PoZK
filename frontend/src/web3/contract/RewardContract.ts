import { CHAINID, contractAddress } from "../constants";
import RewardABI from "@/constants/ABI/Reward.json";
import ContractService from "./contract";

export const RewardContract = () => {
  const address = contractAddress[CHAINID].Reward;
  return new ContractService(address, RewardABI);
};
