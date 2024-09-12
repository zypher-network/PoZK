import { Address } from 'viem';
import { create } from 'zustand';
import { CHAINID, contractAddress } from '@/web3/constants';
import ContractService from '@/web3/contract/contract';
import StakeAbi from "@/web3/contract/abi/Stake.json";
import TokenAbi from "@/web3/contract/abi/Token.json";
import BigNumberJs, { BM18 } from '@/lib/BigNumberJs';


type BalanceStore = {
  payToken: {
    value: string;
    format: string;
    allowance: string;
  },
  minStake: {
    value: string;
    format: string;
  };
  fetching: boolean;
  updateBalance: (address: Address) => Promise<void>;
}

const useBalanceStore = create<BalanceStore>((set, get) => ({
  payToken: {
    value: '0',
    format: '0',
    allowance: '0',
  },
  minStake: {
    value: '0',
    format: '0',
  },
  fetching: false,
  updateBalance: async (address: Address) => {
    set({ fetching: true });
    const tokenContract = new ContractService(contractAddress[CHAINID].Token, TokenAbi);
    const stakeContract = new ContractService(contractAddress[CHAINID].Stake, StakeAbi);
    // const params = [
    //   {
    //     address: contractAddress[CHAINID].Token,
    //     abi: TokenAbi,
    //     functionName: "allowance",
    //     args: [address, contractAddress[CHAINID].Stake],
    //   },
    //   {
    //     address: contractAddress[CHAINID].Token,
    //     abi: TokenAbi,
    //     functionName: "balanceOf",
    //     args: [address],
    //   },
    //   {
    //     address: contractAddress[CHAINID].Stake,
    //     abi: StakeAbi,
    //     functionName: "minStakeAmount",
    //   },
    // ];
    const allowance = await tokenContract.readContractData('allowance', [address, contractAddress[CHAINID].Stake]);
    const balanceOf = await tokenContract.readContractData('balanceOf', [address]);
    const minStake = await stakeContract.readContractData('minStakeAmount', []);
    const allowanceValue = allowance.toString();
    const payTokenValue = balanceOf.toString();
    const minStakeValue = minStake.toString();
    set({
      fetching: false,
      payToken: {
        value: payTokenValue,
        format: new BigNumberJs(payTokenValue).dividedBy(BM18).toFixed(),
        allowance: allowanceValue,
      },
      minStake: {
        value: minStakeValue,
        format: new BigNumberJs(minStakeValue).dividedBy(BM18).toFixed(),
      },
    });
  },
}))

export default useBalanceStore;
