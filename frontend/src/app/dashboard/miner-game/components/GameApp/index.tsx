import React, { useEffect, useMemo, useState } from 'react'
import cx from 'classnames'

import Loading from '@/components/icon/load.svg';
import useProverStore from '@/components/state/proverStore';
import useSubgraphStore from '@/components/state/subgraphStore';
import useBalanceStore from '@/components/state/balanceStore';

interface IGameApp {
}

const GameApp: React.FC<IGameApp> = (props) => {
  const [imgUrl, setImgUrl] = useState('');
  const [loading, setLoading] = useState(false);
  const provers = useProverStore(state => state.provers);
  const { data } = useSubgraphStore(state => state.staking);
  const minStake = useBalanceStore(state => state.minStake);

  const handlePreload = async () => {
    try {
      setLoading(true);
      const response = await fetch('/rewards/miner-game.gif');
      const imgBlob = await response.blob();
      const imgUrl = URL.createObjectURL(imgBlob)
      setImgUrl(imgUrl);
    } catch (error) {
      console.log(error);
    } finally {
      setLoading(false);
    }
  }

  const competitionProvers = useMemo(() =>
    provers
      .filter(prover => prover.name.toLowerCase().includes('competition')),
    [provers],
  );

  const isCompetitionStaked = useMemo(() => {
    return competitionProvers.some(({ id }) =>
      Number(data.find(staking => staking.prover === id)?.newAmount ?? '0') >= Number(minStake.value)
    );
  }, [competitionProvers, data]);

  useEffect(() => {
    if (isCompetitionStaked && !imgUrl) {
      handlePreload();
    }
  }, [isCompetitionStaked]);

  useEffect(() => {
    return () => {
      imgUrl && URL.revokeObjectURL(imgUrl);
    }
  }, []);
  return (
    <div className="h-[560px] bg-[#11182B] border-[#2E3751] flex-grow-0 flex-shrink basis-[950px] rounded-[20px] border p-[6px]">
      <div
        className={cx(
          'size-full flex justify-center items-center border-4 border-[#051027] rounded-2xl',
          { 'filter grayscale': !Boolean(imgUrl) }
        )}
        style={{
          backgroundImage: imgUrl ? `url(${imgUrl})` : 'url(/rewards/miner-game-preview.png)',
          backgroundSize: 'cover',
        }}
      >
        {
          loading && (
            <div className="flex flex-col justify-center items-center gap-0">
              <Loading className="animate-spin" />
              <div className="text-xl leading-normal text-white opacity-30">
                Loading Data...
              </div>
            </div>
          )
        }
      </div>
    </div>
  )
};

export default GameApp;
