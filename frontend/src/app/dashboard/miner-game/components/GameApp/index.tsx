import React, { useEffect, useState } from 'react'
import cx from 'classnames'

import Loading from '@/components/icon/load.svg';

interface IGameApp {
}

const GameApp: React.FC<IGameApp> = (props) => {
  const [imgUrl, setImgUrl] = useState('');

  const handlePreload = async () => {
    try {
      const response = await fetch('/rewards/miner-game.gif');
      const imgBlob = await response.blob();
      setImgUrl(URL.createObjectURL(imgBlob));
    } catch (error) {
    }
  }

  useEffect(() => {
    handlePreload();
    return () => {
      imgUrl && URL.revokeObjectURL(imgUrl);
    }
  }, []);
  return (
    <div className="h-[560px] bg-[#11182B] border-[#2E3751] flex-grow-0 flex-shrink basis-[950px] rounded-[20px] border p-[6px]">
      <div
        className={cx(
          'size-full flex justify-center items-center border-4 border-[#051027] rounded-2xl',
          {
            'filter grayscale': Boolean(imgUrl),
          }
        )}
        style={{
          // backgroundImage: `url(${imgUrl})`,
          backgroundImage: imgUrl ? `url(/rewards/miner-game-preview.png)` : '',
          backgroundSize: 'cover',
        }}
      >
        {
          imgUrl === '' && (
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
