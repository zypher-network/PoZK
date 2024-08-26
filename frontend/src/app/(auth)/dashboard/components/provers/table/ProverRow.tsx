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

interface IProverRow {
  name: string;
  running?: boolean;
  created: string;
  prover: Address;
  version: string;
  overtime: string;
}

const ProverRow: React.FC<IProverRow> = ({ running = false, name, created, prover, version, overtime }) => {
  const { setStakeItemHandler } = usePostStake();
  const { address } = useAccount();
  const { data } = useQuery(GET_TASKS, { variables: { address: address?.toLowerCase() ?? '', prover: prover.toLowerCase() }, skip: !address })
  const stakingAmount = useGetMinerStaking(prover);
  return (
    <TableRow>
      <TableCell className="capitalize">{running ? 'Running' : 'Ping'}</TableCell>
      <TableCell className="font-medium capitalize">{name}</TableCell>
      <TableCell>{created ? calcDuration(new Date(created)) : '--'}</TableCell>
      <TableCell>{new BigNumberJs(stakingAmount).div(BM18).toFormat()}</TableCell>
      <TableCell>{data?.tasks?.length ?? '--'}</TableCell>
      <TableCell className="justify-end gap-2">
        {new BigNumberJs(stakingAmount).gt(0) && !running && (
          <Recommendation
            name={name}
            image={prover}
            tag={version}
            overtime={overtime}
          />
        )}
        {/* <Button
          variant="outline"
          className="border-[#e78a09] text-[#e78a09] text-[16px] py-0 px-4 h-8"
        >
          Delete
        </Button> */}
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
      </TableCell>
    </TableRow>
  );
}

export default ProverRow;
