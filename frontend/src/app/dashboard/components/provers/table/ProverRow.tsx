import { Address } from "viem";
import { DotsHorizontalIcon } from "@radix-ui/react-icons";
import {
  TableCell,
  TableRow,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import Recommendation from "../../dialog/Recommendation/Recommendation";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { usePostStake } from "../../../hook/useStake";
import { calcDuration } from "@/lib/day";
import BigNumberJs, { BM18 } from "@/lib/BigNumberJs";
import useGetMinerStaking from "@/components/hooks/useGetMinerStaking";
import { useQuery } from "@apollo/client";
import { GET_TASKS } from "@/components/queries/tasks";
import { useAccount } from "wagmi";
import useBalanceStore from "@/components/state/balanceStore";
import { useMemo } from "react";

interface IProverRow {
  name: string;
  stop: boolean;
  running?: boolean;
  created: string;
  prover: Address;
  version: string;
  overtime: string;
  needUpgrade?: boolean;
}

const ProverRow: React.FC<IProverRow> = ({ running = false, stop, name, created, prover, version, overtime, needUpgrade = false }) => {
  const { setStakeItemHandler } = usePostStake();
  const minStake = useBalanceStore(state => state.minStake);
  const { address } = useAccount();
  const { data } = useQuery(GET_TASKS, { variables: { address: address?.toLowerCase() ?? '', prover: prover.toLowerCase() }, skip: !address })
  const stakingAmount = useGetMinerStaking(prover);
  const isMiner = useMemo(() => new BigNumberJs(stakingAmount).div(BM18).gte(minStake.format), [stakingAmount, minStake.format])

  const renderAction = () => {
    if (running) {
      if (isMiner) {
        return (
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" className="h-8 w-8 p-0">
                <span className="sr-only">Open menu</span>
                <DotsHorizontalIcon className="h-4 w-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuLabel>Actions</DropdownMenuLabel>
              <DropdownMenuItem
                onClick={() => setStakeItemHandler(prover, "Stake")}
              >
                Stake
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem
                onClick={() => setStakeItemHandler(prover, "UnStake")}
              >
                UnStake
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )
      }
      return (
        <div className="py-1 cursor-pointer text-black px-3 rounded text-sm bg-[#82c01e]" onClick={() => setStakeItemHandler(prover, "Stake")}>Stake</div>
      );
    }
    if (stop && new BigNumberJs(stakingAmount).gt('0')) {
      return (
        <div className="py-1 cursor-pointer text-white px-3 rounded text-sm bg-[#E44042]" onClick={() => setStakeItemHandler(prover, "UnStake")}>UnStake</div>
      )
    }
    return null;
  }

  if (stop && new BigNumberJs(stakingAmount).lte('0')) {
    return null;
  }

  return (
    <TableRow>
      <TableCell className="capitalize">{stop ? 'Disabled' : (running ? 'Running' : 'Ping')}</TableCell>
      <TableCell className="font-medium capitalize">{name}</TableCell>
      <TableCell>{created ? calcDuration(new Date(created)) : '--'}</TableCell>
      <TableCell>{new BigNumberJs(stakingAmount).div(BM18).toFormat()}</TableCell>
      <TableCell>{data?.tasks?.length ?? '--'}</TableCell>
      <TableCell>{stop ? "" : (isMiner ? "✅" : "")}</TableCell>
      <TableCell className="justify-end gap-2">
        {!stop && !running && (
          <Recommendation
            name={name}
            image={prover}
            tag={version}
            overtime={overtime}
            needUpgrade={needUpgrade}
          />
        )}
        {renderAction()}
      </TableCell>
    </TableRow>
  );
}

export default ProverRow;
