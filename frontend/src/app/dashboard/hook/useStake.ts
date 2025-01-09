"use client";
import { useFailedRoute } from "@/components/hooks/useFailedRoute";
import { useToast } from "@/components/ui/use-toast";
import { CHAINID, contractAddress } from "@/web3/constants";
import { StakeContract } from "@/web3/contract/StakeContract";
import { Erc20Contract } from "@/web3/contract/TokenContract";
import { useCallback } from "react";
import { Address } from "viem";
import { useAccount } from "wagmi";
import BigNumberJs, { BM18 } from "@/lib/BigNumberJs";
import { useRecoilState, useSetRecoilState } from "recoil";
import { StakeFormBtnLabel, StakeItem } from "../state/dashboardState";
import useProverStore from "@/components/state/proverStore";
import useBalanceStore from "@/components/state/balanceStore";
import { useShallow } from "zustand/react/shallow";
import useSubgraphStore from "@/components/state/subgraphStore";

export const usePostStake = () => {
  const reset = useSubgraphStore(state => state.reset);
  const { provers, refetch } = useProverStore(useShallow(state => ({ provers: state.provers, refetch: state.fetchUserProvers })));
  const { payToken, updateBalance } = useBalanceStore(useShallow(state => ({ payToken: state.payToken, updateBalance: state.updateBalance })))
  const Failed = useFailedRoute();
  const { address } = useAccount();
  const { toast } = useToast();
  const [stakeItem, setStakeItem] = useRecoilState(StakeItem);
  const setBtnLabel = useSetRecoilState(StakeFormBtnLabel);
  const setStakeItemHandler = useCallback(
    (prover: Address, key: "Stake" | "UnStake") => {
      const item = provers.find(p => p.id === prover);
      if (item) {
        setStakeItem({
          key,
          item,
        });
      }
    },
    [setStakeItem, provers]
  );
  const minerStake = useCallback(
    async (prover: Address, amount: string) => {
      try {
        const erc20 = Erc20Contract(contractAddress[CHAINID].Token);
        // const prover = item[IGameConfigKey.prover];
        const inputAmount = new BigNumberJs(amount).times(BM18);
        if (new BigNumberJs(payToken.allowance).lt(inputAmount)) {
          await erc20.writeContractMethod("approve", [
            contractAddress[CHAINID].Stake,
            inputAmount.toFixed(),
          ]);
          setBtnLabel("Confirm");
        }
        const contract = StakeContract();
        const contractRes = await contract.writeContractMethod("minerStake", [
          prover,
          inputAmount.toFixed(),
        ]);
        if (contractRes) {
          toast({
            title: "Staking Success",
            variant: "success",
          });
        }
        return true;
      } catch (error: any) {
        await Failed(error);
        return false;
      }
    },
    [setBtnLabel, toast, Failed]
  );
  const minerUnStake = useCallback(
    async (prover: Address, amount: string) => {
      try {
        const inputAmount = new BigNumberJs(amount).times(BM18);
        const contract = StakeContract();
        const contractRes = await contract.writeContractMethod("minerUnstake", [
          prover,
          BigInt(inputAmount.toString(10)),
        ]);
        if (contractRes) {
          toast({
            title: "UnStaking Success",
            variant: "success",
          });
        }
        return true;
      } catch (error: any) {
        console.log(error);
        await Failed(error);
        return false;
      }
    },
    [Failed, toast]
  );
  const stakeHandler = useCallback(
    async (prover: Address, amount: string) => {
      let res = false;
      if (stakeItem?.key === "Stake") {
        res = await minerStake(prover, amount);
      }
      if (stakeItem?.key === "UnStake") {
        res = await minerUnStake(prover, amount);
      }
      if (res) {
        if (address) {
          reset('staking');
          updateBalance(address);
        }
        setStakeItem(undefined);
      }
    },
    [minerStake, minerUnStake, setStakeItem, stakeItem?.key]
  );

  const claim = useCallback(async () => {
    const contract = StakeContract();
    const contractRes = await contract.writeContractMethod("claim", [address]);
  }, []);
  return {
    claim,
    stakeHandler,
    setStakeItemHandler,
  };
};
