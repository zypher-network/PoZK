import {
  TableCell,
  TableRow,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import { usePostStake } from "../../../hook/useStake";
import { UserProver } from "@/types/IProver";
import BigNumberJs, { BM18 } from "@/lib/BigNumberJs";
import useGetMinerStaking from "@/components/hooks/useGetMinerStaking";

interface IMarketRow {
  prover: UserProver;
  minStakeAmount?: string;
}

const MarketRow: React.FC<IMarketRow> = ({ prover, minStakeAmount = '--' }) => {
  const { setStakeItemHandler } = usePostStake();
  const stakingAmount = useGetMinerStaking(prover.id);
  return (
    <TableRow>
      <TableCell className="font-medium capitalize">{prover.name}</TableCell>
      <TableCell>{minStakeAmount}</TableCell>
      <TableCell className="font-medium">{new BigNumberJs(stakingAmount).div(BM18).toFormat()}</TableCell>
      <TableCell>{new BigNumberJs(stakingAmount).div(BM18).gte(minStakeAmount) ? "✅" : ""}</TableCell>
      <TableCell className="justify-end">
        <Button
          variant="outline"
          className="border-[#eed918] text-[#eed918] text-[16px] py-0 px-4 h-8"
          onClick={() => setStakeItemHandler(prover.id, "Stake")}
        >
          Stake
        </Button>
      </TableCell>
    </TableRow>
  );
}

export default MarketRow;
