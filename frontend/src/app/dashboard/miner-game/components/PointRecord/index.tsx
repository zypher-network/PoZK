import React from 'react'
import dayjs from "dayjs";

import './index.css';

interface IPointRecord {}

const tmpData = new Array(14).fill('').map(() => ({
  date: new Date(),
  points: Math.ceil(Math.random() * 10000)
}));

const PointRecord: React.FC<IPointRecord> = (props) => {
  return (
    <div className="h-[560px] flex flex-col gap-0 flex-grow flex-shrink bg-[#11182B] border-[#2E3751] rounded-[20px] border">
      <div className="px-6 pt-6">
        <div className="flex border-[#1F2D4E] border-b justify-between px-5 pb-3 opacity-70 font-light text-base">
          <div>Date</div>
          <div>Points</div>
        </div>
      </div>
      <div className="flex-1 overflow-y-auto pl-6 pt-2 records mr-3 mt-2 mb-5 pr-2">
       <div className="flex flex-col gap-4 w-full">
        {tmpData.map((data, idx) => (
          <div key={idx} className="h-16 w-ful bg-[#0A1223] rounded-[10px] p-5 font-light flex justify-between items-center text-lg leading-5">
            <div>{dayjs(data.date).format('MMM DD.YYYY HH:mm:ss')}</div>
            <div className="text-[#FACC16]">{data.points}</div>
          </div>
        ))}
       </div>
      </div>
    </div>
  )
};

export default PointRecord;
