import { Address } from "viem";
import { CHAINID, contractAddress } from "../constants";
import TokenABI from "@/constants/ABI/Token.json";
import ContractService from "./contract";

export const TokenContract = () => {
  const address = contractAddress[CHAINID].Token;
  return new ContractService(address, TokenABI);
};

export const Erc20Contract = (address: Address) => {
  return new ContractService(address, TokenABI);
};
