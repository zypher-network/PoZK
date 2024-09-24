import { create } from 'zustand';
import pozk from '@/services/pozk';
import { GQLProver, UserProver } from '@/types/IProver';
import ContractService from '@/web3/contract/contract';
import TokenAbi from "@/web3/contract/abi/Token.json";
import useSubgraphStore from './subgraphStore';


type ProverStore = {
  provers: UserProver[];
  fetching: boolean;
  fetchUserProvers: (gqlProvers?: GQLProver[]) => Promise<void>;
}

const useProverStore = create<ProverStore>((set) => ({
  provers: [],
  fetching: false,
  fetchUserProvers: async (gqlProvers) => {
    set({ fetching: true, provers: [] });
    const provers: UserProver[] = [];
    const containers = await pozk.getProverContainers(1);
    const rawProvers = gqlProvers || useSubgraphStore.getState().provers.data || [];
    for (const prover of rawProvers) {
      const contract = new ContractService(prover.id, TokenAbi);
      const proverName = await contract.readContractData('name', []);
      provers.push({
        ...prover,
        name: proverName.toString(),
        containers: containers
          .filter(container => container.prover.toLowerCase() === [prover.id.toLowerCase(), `v${prover.version}`].join('-'))
          .map(container => ({
            id: container.image,
            running: true,
            created: new Date(container.created * 1000).toLocaleDateString("en-US"),
          })),
      })
    }
    set({ fetching: false, provers: provers });
  },
}))

export default useProverStore;
