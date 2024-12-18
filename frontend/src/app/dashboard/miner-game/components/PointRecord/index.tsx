import React, { Fragment, useMemo } from 'react'
import BigNumberJs, { BM18 } from '@/lib/BigNumberJs';
import dayjs from "dayjs";

import NoData from '@/components/icon/no-data.svg';
import useSubgraphStore from '@/components/state/subgraphStore';

import './index.css';

interface IPointRecord {}

const tmpData = new Array(14).fill('').map(() => ({
  date: new Date(),
  points: Math.ceil(Math.random() * 10000)
}));

const PointRecord: React.FC<IPointRecord> = (props) => {
  const subgraphData = useSubgraphStore();

  const record = useMemo(() => {
    if (!subgraphData.reward.data) return [];
    const provers = subgraphData.provers.data
      .filter(prover => prover.name.toLowerCase().includes('competition'))
      .map(prover => [prover.id.toLowerCase(), prover.name]);
    
    const epochDate = subgraphData.epoches.data.reduce((prev, curr) => {
      prev[curr.id] = dayjs(+curr.startAt * 1000).format('MMM DD.YYYY');
      return prev;
    }, {} as Record<string, string>);
    return subgraphData.reward.data.claimList
      .filter(claim => provers.some(prover => prover[0].includes(claim.prover.toLowerCase())) && Boolean(claim.claim))
      .map(claim => ({
        points: new BigNumberJs(claim.estimate ?? claim.claim).div(BM18).toFormat(),
        date: epochDate[claim.epoch],
        name: provers.find(prover => prover[0].includes(claim.prover.toLowerCase()))?.[1] ?? '',
      }))
  }, [subgraphData]);
  return (
    <div className="h-[560px] flex flex-col gap-0 bg-[#11182B] border-[#2E3751] rounded-[20px] border flex-grow-0 flex-shrink-0 basis-[500px]">
      <div className="px-6 pt-6">
        <div className="flex border-[#1F2D4E] border-b justify-between px-5 pb-3 opacity-70 font-light text-base">
          <div className="flex-grow-0 flex-shrink-0 basis-[100px]">Date</div>
          <div className="flex-1 text-left pl-2">Prover</div>
          <div>Points</div>
        </div>
      </div>
      <div className="flex-1 overflow-y-auto pl-6 pt-2 records mr-3 mt-2 mb-5 pr-2">
       <div className="flex flex-col gap-4 w-full h-full">
        {record.length ?
          record.map((data, idx) => (
            <div key={idx} className="h-16 w-ful bg-[#0A1223] rounded-[10px] p-5 font-light flex justify-between items-center text-lg leading-5">
              <div className="flex-grow-0 flex-shrink-0 basis-[100px]">{data.date}</div>
              <div className="text-[#FACC16] flex-1 text-left pl-2">{data.name}</div>
              <div className="text-[#FACC16]">{data.points}</div>
            </div>
          )) : (
            <div className="flex h-full w-full pt-6 justify-center items-center">
              <div className="flex flex-col gap-1">
                <NoData />
                <div className="opacity-50 text-xl leading-normal">No Data</div>
              </div>
            </div>
          )
        }
       </div>
      </div>
    </div>
  )
};

export default PointRecord;
