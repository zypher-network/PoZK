import React, { useLayoutEffect } from 'react'
import { useShallow } from 'zustand/react/shallow';

import useProverStore from '@/components/state/proverStore';
import Loading from '@/components/icon/loading.svg';
import UserProvers from './table/UserProvers';
import useSubgraphStore from '@/components/state/subgraphStore';
import useSortProvers from '@/components/hooks/useSortProvers';

const Provers: React.FC = () => {
  const { data } = useSubgraphStore(state => state.provers);
  const provers = useSortProvers();
  const { fetching, fetchUserProvers } = useProverStore(useShallow(state => ({
    fetching: state.fetching,
    fetchUserProvers: state.fetchUserProvers,
    provers: state.provers,
  })));

  useLayoutEffect(() => {
    data.length && fetchUserProvers(data);
  }, [data]);

  return (
    <div>
      {
        fetching ? (
          <div className="flex items-center justify-center animate-spin">
            <Loading className='scale-y-[-1]' height={'40px'} width={'40px'} /> 
          </div>
        ) : (
          <div className="flex flex-col gap-6">
            {/* <Market provers={provers} /> */}
            <UserProvers provers={provers} />
          </div>
        )
      }
    </div>
  )
};

export default Provers;
