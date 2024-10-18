import useSubgraphStore from "../state/subgraphStore";
import useProverStore from "../state/proverStore";
import BigNumberJs from "@/lib/BigNumberJs";

const useSortProvers = () => {
  const { data } = useSubgraphStore(state => state.staking);
  const provers = useProverStore(state => state.provers);
  const getStakingAmount = (prover: string) => {
    return data.find(staking => staking.prover === prover)?.newAmount ?? '0';
  }
  return provers.slice().sort((x, y) => new BigNumberJs(getStakingAmount(x.id)).gt(getStakingAmount(y.id)) ? -1 : 1);
}

export default useSortProvers;
