import { CHAINID, contractAddress } from "../constants";
import ControllerABI from "./abi/Controller.json";
import ContractService from "./contract";
export const ControllerContract = () => {
  const address = contractAddress[CHAINID].Controller;
  return new ContractService(address, ControllerABI);
};
