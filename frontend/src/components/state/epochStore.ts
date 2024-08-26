import { UserEpoch } from '@/types/epoch';
import { create } from 'zustand';

type EpochStore = {
  selected: UserEpoch | null;
  fetching: boolean;
  setSelectEpoch: (epoch: UserEpoch | null) => void;
}

const useEpochStore = create<EpochStore>((set) => ({
  selected: null,
  fetching: false,
  setSelectEpoch: (epoch) => { set({ selected: epoch })}, 
}))

export default useEpochStore;
