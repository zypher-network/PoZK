import { CHAINID, contractAddress } from "../constants";
import StakeABI from "./abi/Stake.json";
import ContractService from "./contract";
export const StakeContract = () => {
  const address = contractAddress[CHAINID].Stake;
  return new ContractService(address, StakeABI);
};
