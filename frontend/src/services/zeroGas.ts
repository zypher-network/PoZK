
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

class ZeroGas {
  endpoints = {
    balanceOf: '/api/balanceof/'
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
}

export default new ZeroGas();
