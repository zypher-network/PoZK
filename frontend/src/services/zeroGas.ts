
const baseUrl = 'https://gas.zypher.network' as const;

type ResponseGetBalance = {
  code: number;
  data: {
    amount: string;
    wallet: string;
  };
  msg: string | null;
  uid: string;
}

type ResponseCreate = {
  code: number;
  data: {
    tx_hash: string;
    wallet: string;
  };
  msg: string | null;
  uid: string;
}

class ZeroGas {
  endpoints = {
    balanceOf: '/api/balanceof/',
    create: '/api/create',
  }

  async getBalance (address: string) {
    try {
      const response = await fetch(`${baseUrl}${this.endpoints.balanceOf}${address}`);
      const result = await response.json() as ResponseGetBalance;
      return result;
    } catch (error) {
      console.log(error);
    }
    return null;
  }

  async createAAWallet (address: string) {
    try {
      const response = await fetch(`${baseUrl}${this.endpoints.create}`, {
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
