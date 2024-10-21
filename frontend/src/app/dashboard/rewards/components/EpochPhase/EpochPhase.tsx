"use client";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { memo, useMemo, useState } from "react";
import dayjs from 'dayjs';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

import { FaAngleDoubleDown } from "react-icons/fa";
import useEpochStore from "@/components/state/epochStore";
import { UserEpoch } from "@/types/epoch";
import { Address } from "viem";
import BigNumberJs, { BM18 } from "@/lib/BigNumberJs";
import { calcDuration } from "@/lib/day";
import { Button } from "@/components/ui/button";
import Claim from "../dialog/Claim/Claim";
import { headerList } from "../../state/state";
import useSubgraphStore from "@/components/state/subgraphStore";
import { useShallow } from "zustand/react/shallow";

const EpochPhase = () => {
  const [viewAll, setViewAll] = useState(false);
  const setEpoch = useEpochStore(state => state.setSelectEpoch);
  const { epoches, reward } = useSubgraphStore(useShallow(state => ({ reward: state.reward, epoches: state.epoches })));

  const displayEpoches = useMemo<UserEpoch[]>(() => {
    if (!reward.pending) {
      const userEpoches: UserEpoch[] = [];
      for (const epoch of epoches.data) {
        const { id } = epoch;
        let totalEstimate = '0';
        let claimable = false;
        let provers: Address[] = [];
        for (const claimItem of (reward.data?.claimList ?? [])) {
          if (claimItem.epoch === id) {
            totalEstimate = new BigNumberJs(claimItem.claim ?? claimItem.estimate).plus(totalEstimate).toString(10);
            provers.push(claimItem.prover);
            if (!claimItem.claim) claimable = true;
          }
        }

        userEpoches.push({
          ...epoch,
          estimate: totalEstimate,
          claimable,
          provers,
          epoch: id,
        })
      }
      return userEpoches;
    }
    return [];
  }, [epoches, reward]);
  return (
    <Card>
      <CardHeader className="flex flex-row justify-between items-center pb-[24px]">
        <CardTitle>Epoch/phase</CardTitle>
        <Card
          onClick={() => setViewAll(state => !state)}
          className="bg-primary px-[16px] py-[0] gap-[4px] text-[16px] font-light
      flex flex-row justify-end items-center h-[36px] mt-0 cursor-pointer
        hover:bg-primary/90"
        >
          <FaAngleDoubleDown />
          View All
        </Card>
      </CardHeader>
      <Table>
        <TableHeader>
          <TableRow className="border-[#1F2D4E]">
            {headerList.map((v, index) => (
              <TableHead
                key={v}
                className={`${
                  [0, 1, 2].includes(index) ? "text-left" : "text-right"
                }`}
              >
                {v}
              </TableHead>
            ))}
          </TableRow>
        </TableHeader>

        <TableBody>
          <TableRow>
            <TableCell
              colSpan={headerList.length}
              className="h-[16px] bg-transparent"
            ></TableCell>
          </TableRow>
          {/* <TableRow>
          <TableCell
            colSpan={headerList.length}
            className="h-[200px] bg-transparent"
          >
            <NoData />
          </TableCell>
        </TableRow> */}
          {(!viewAll ? displayEpoches.slice(0, 4) : displayEpoches).map((item) => (
            <TableRow key={item.id}>
              <TableCell>
                <div className="flex flex-row items-center gap-[40px]">
                  {`Epoch ${item.id}`}
                  {!item.endAt ? (
                    <div
                      className="h-[36px] px-[16px] bg-[#82c01e] rounded-[8px] text-[#0A1223] 
                    font-light
                    text-[18px]
                    leading-[36px]
                    "
                    >
                      Current
                    </div>
                  ) : null}
                </div>
              </TableCell>
              <TableCell
                className="justify-start text-[#FFFFFF]"
              >
                <div className="flex flex-col gap-1">
                  <div>{dayjs(+item.startAt * 1000).format('MMM DD.YYYY hh:mm:ss A')} -</div>
                  {item.endAt && (<div>{dayjs(+item.endAt * 1000).format('MMM DD.YYYY hh:mm:ss A')}</div>)}
                </div>
              </TableCell>
              <TableCell
                className="justify-start text-[#FFFFFF]"
              >
                {item.endAt ? calcDuration(new Date(+item.startAt * 1000), new Date(+item.endAt * 1000)) : '--'}
              </TableCell>
              <TableCell
                className="justify-end text-[#FACC16]"
              >
                {new BigNumberJs(item.estimate).div(BM18).toFormat()}
              </TableCell>
              <TableCell className="justify-end">
                {!item.endAt || item.estimate === '0' ? (
                  <Button variant={"outline"} className="opacity-0 pointer-events-none">
                    Collected
                  </Button>
                ) : (
                  item.claimable ? (
                    <Button variant={"outline"} onClick={() => setEpoch(item)}>
                      Collect
                    </Button>
                  ) : (
                    <Button variant={"ghost"} disabled={true}>
                      Collected
                    </Button>
                  )
                )}
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
      <Claim />
    </Card>
  );
};
export default memo(EpochPhase);
