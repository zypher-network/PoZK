import { Address } from "viem";
import useSubgraphStore from "../state/subgraphStore";
import { useMemo } from "react";

const useGetMinerStaking = (prover: Address) => {
  const { data } = useSubgraphStore(state => state.staking);
  const stakingAmount = useMemo(() => data.find(staking => staking.prover === prover)?.newAmount ?? '0', [data]);
  return stakingAmount;
}

export default useGetMinerStaking;
