import { IControllerItem } from '@/app/dashboard/state/dashboardState';
import pozk from '@/services/pozk';
import { create } from 'zustand';

type ControllerStore = {
  controllers: IControllerItem[];
  active: string;
  fetching: boolean;
  page: number;
  fetch: (page: number) => Promise<void>;
}

const useControllerStore = create<ControllerStore>((set) => ({
  controllers: [],
  active: '',
  fetching: false,
  page: 1,
  fetch: async (page: number) => {
    set({ fetching: true });
    try {
      const controllers = await pozk.getControllers(page);
      const active = await pozk.getActiveController();
      set({
        fetching: false,
        active,
        controllers: (active ? [active] : [])
          .concat(controllers.filter(address => address !== active))
          .map(address => ({
            status: address === active ? 'on' : 'off',
            address,
          }))
      });
    } catch (error) {
      set({ fetching: false });
      throw error;
    }
  },
}))

export default useControllerStore;
