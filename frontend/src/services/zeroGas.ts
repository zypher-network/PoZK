import { CHAINID, ZeroGasUrls } from "@/web3/constants";

type ResponseGetBalance = {
  amount: string;
  wallet: string;
}

type ResponseCreate = {
  tx_hash: string;
  wallet: string;
}

class ZeroGas {
  endpoints = {
    balanceOf: '/balanceof/',
    create: '/create',
  }

  async getBalance (address: string) {
    try {
      const response = await fetch(`${ZeroGasUrls[CHAINID]}${this.endpoints.balanceOf}${address}`);
      const result = await response.json() as ResponseGetBalance;
      return result;
    } catch (error) {
      console.log(error);
    }
    return null;
  }

  async createAAWallet (address: string) {
    try {
      const response = await fetch(`${ZeroGasUrls[CHAINID]}${this.endpoints.create}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          owner: address,
        }),
      });
      const result = await response.json() as ResponseCreate;
      return result;
    } catch (error) {
      console.log(error);
    }
    return null;
  }
}

export default new ZeroGas();
