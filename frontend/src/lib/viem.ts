import { Addressish } from "@/types/core";
import { getAddress } from "viem";

export const getAddressSafe = (address: Addressish) => {
  if (address) {
    try {
      getAddress(address);
      return true;
    } catch {
      return false;
    }
  }
};
