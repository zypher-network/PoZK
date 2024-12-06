import { GQLProver } from '@/types/IProver';
import { GQLEpoch, GQLEpochReward } from '@/types/epoch';
import { GQLStaking } from '@/types/staking';
import { create } from 'zustand';

type FieldKeys = 'epoches' | 'provers' | 'reward' | 'staking' | 'claimedAmount';
type FieldValues = GQLEpoch[] | GQLProver[] | GQLStaking[] | GQLEpochReward | string | null;
type SubgraphStore = {
  epoches: {
    data: GQLEpoch[];
    pending: boolean;
  };
  provers: {
    data: GQLProver[];
    pending: boolean;
  };
  reward: {
    data: GQLEpochReward | null;
    pending: boolean;
  };
  staking: {
    data: GQLStaking[];
    pending: boolean;
  },
  claimedAmount: {
    data: string;
    pending: boolean;
  },
  setData: (key: FieldKeys, data: FieldValues) => void;
  reset: (key: FieldKeys) => void;
}

const useSubgraphStore = create<SubgraphStore>((set, get) => ({
  epoches: {
    data: [],
    pending: false,
  },
  provers: {
    data: [],
    pending: false,
  },
  reward: {
    data: null,
    pending: false,
  },
  staking: {
    data: [],
    pending: false,
  },
  claimedAmount: {
    data: '0',
    pending: false,
  },
  setData (key, data) {
    set({
      [key]: {
        data,
        pending: false,
      }
    })
  },
  reset (key) {
    set({
      [key]: {
        data: get()[key].data,
        // data: key === 'reward' ? null : [],
        pending: true,
      }
    })
  }
}))

export default useSubgraphStore;
