import { GQLEpoch, GQLEpochReward, GQLUnstakingClaim } from "@/types/epoch";
import { useQuery } from "@apollo/client";
import { useAccount } from "wagmi";
import { GET_EPOCHES, GET_MINER_REWARDS } from "../queries/epoches";
import { useEffect } from "react";
import useSubgraphStore from "../state/subgraphStore";
import { useShallow } from "zustand/react/shallow";
import { GQLProver } from "@/types/IProver";
import { GET_PROVERS } from "../queries/provers";
import { GET_MINER_STAKING } from "../queries/staking";
import { GQLStaking } from "@/types/staking";
import BigNumberJs, { BM18 } from "@/lib/BigNumberJs";

const useInitSubgraph = () => {
  const { address } = useAccount();
  const { setData, gqlReward, gqlEpoches, gqlProvers, gqlStaking } = useSubgraphStore(useShallow(state => ({
    gqlEpoches: state.epoches,
    gqlReward: state.reward,
    gqlProvers: state.provers,
    gqlStaking: state.staking,
    setData: state.setData,
  })));
  const { data: { provers } = {}, refetch: refetchProvers } = useQuery<{ provers: GQLProver[] }>(GET_PROVERS);
  const { data: { epoches } = {}, refetch: refetchEpoches } = useQuery<{ epoches: GQLEpoch[] }>(GET_EPOCHES);
  const { data: { reward = null, unstakingClaims } = {}, refetch: refetchReward } = useQuery<{ reward: GQLEpochReward, unstakingClaims: GQLUnstakingClaim[] }>(GET_MINER_REWARDS, { variables: { id: address?.toLowerCase() }, skip: !address });
  const { data: { stakings } = {}, refetch: refetchStaking } = useQuery<{ stakings: GQLStaking[] }>(GET_MINER_STAKING, { variables: { account: address?.toLowerCase() }, skip: !address, fetchPolicy: 'no-cache' });

  useEffect(() => {
    setData('reward', reward);
  }, [reward]);

  useEffect(() => {
    setData('provers', provers ?? []);
  }, [provers]);

  useEffect(() => {
    setData('epoches', epoches ?? []);
  }, [epoches]);

  useEffect(() => {
    setData('staking', stakings ?? []);
  }, [stakings]);

  useEffect(() => {
    setData('claimedAmount', unstakingClaims ? unstakingClaims.reduce((prev, curr) => prev.plus(curr.amount), new BigNumberJs('0')).div(BM18).toFormat() : '0')
  }, [unstakingClaims]);

  useEffect(() => {
    gqlEpoches.pending && refetchEpoches().then(({ data }) => setData('epoches', data?.epoches ?? []));
    gqlReward.pending && refetchReward().then(({ data }) => {
      setData('reward', data?.reward ?? null);
      setData('claimedAmount', data?.unstakingClaims ? data.unstakingClaims.reduce((prev, curr) => prev.plus(curr.amount), new BigNumberJs('0')).div(BM18).toFormat() : '0')
    });
    gqlProvers.pending && refetchProvers();
    gqlStaking.pending && refetchStaking();
  }, [gqlProvers.pending, gqlEpoches.pending, gqlReward.pending, gqlStaking.pending]);
}

export default useInitSubgraph;
