"use client";

import { IGameConfigKey, UserProver } from "@/types/IProver";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Card, CardTitle } from "@/components/ui/card";
import { useMemo } from "react";
import ControllerBtn from "../../controllerBtn/controllerBtn";
import MarketRow from './MarketRow';
import useBalanceStore from '@/components/state/balanceStore';

export const ProversHeader = [
  { label: "Prover Name", key: IGameConfigKey.name },
  { label: "Min Stake Amount", key: IGameConfigKey.minStakeAmountStr },
  { label: "Staking", key: IGameConfigKey.minerStakingStr },
  { label: "Is Miner", key: IGameConfigKey.isMinerStr },
];

interface IMarket {
  provers: UserProver[];
}

const Market: React.FC<IMarket> = ({ provers }) => {
  const minStake = useBalanceStore(state => state.minStake);
  const Label = useMemo(() => {
    return [...ProversHeader, { label: "Action", key: undefined }];
  }, []);

  return (
    <Card>
      <div className="flex items-center justify-between">
        <CardTitle>
          Market
        </CardTitle>
        <ControllerBtn />
      </div>
      <Table>
        <TableHeader>
          <TableRow className="border-[#1F2D4E]">
            {Label.map((v) => (
              <TableHead
                key={v.label}
                className={`${
                  v.label === "Action" ? "text-right" : "text-left"
                }`}
              >
                {v.label}
              </TableHead>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow>
            <TableCell
              colSpan={5}
              className="h-[16px] bg-transparent"
            ></TableCell>
          </TableRow>
          {provers.map(prover => <MarketRow key={prover.id} prover={prover} minStakeAmount={minStake.format} />)}
        </TableBody>
      </Table>
    </Card>
  );
};
export default Market;
