import { create } from 'zustand';
import pozk from '@/services/pozk';
import { GQLProver, UserProver } from '@/types/IProver';
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
      provers.push({
        ...prover,
        containers: containers
          .filter(container => container.prover.toLowerCase() === prover.id.toLowerCase())
          .map(container => ({
            id: container.image,
            running: true,
            needUpgrade: container.tag != `v${prover.version}`,
            created: new Date(container.created * 1000).toLocaleDateString("en-US"),
          })),
      })
    }
    provers.sort((x, y) => x.containers.length > y.containers.length ? -1 : 1);
    set({ fetching: false, provers: provers });
  },
}))

export default useProverStore;
