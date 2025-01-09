"use client";
import { useAccount } from "wagmi";
import { Card, CardTitle } from "@/components/ui/card";
import { useMemo, useState } from "react";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import ProverRow from "./ProverRow";
import { UserProver } from "@/types/IProver";
import ControllerBtn from "../../controllerBtn/controllerBtn";
import NoData from '@/components/icon/no-data.svg';
import { useQuery } from "@apollo/client";
import { GET_MINER_TESTS, IMinerTests } from "@/components/queries/minerTests";
import MinerTestTipDialog from "./MinerTestTipDialog";
import useBalanceStore from "@/components/state/balanceStore";

interface IUserProvers {
  provers: UserProver[];
}

const UserProvers: React.FC<IUserProvers> = ({ provers }) => {
  const [dialog, setDialog] = useState({ open: false, type: '', testId: '' });
  const { isConnected, address } = useAccount();
  const result = useQuery<IMinerTests>(GET_MINER_TESTS, {
    variables: {
      address: address?.toLowerCase(),
    },
    skip: !Boolean(address),
    pollInterval: 10000,
  })

  const Label = useMemo(() => {
    return [
      "Status",
      "Prover",
      "Time Connected",
      "Staking Amount",
      "Total Tasks",
      "Is Miner",
      "Action", // delete
    ];
  }, []);

  const minerTestsResult = useMemo(() => {
    const provers: Record<string, { isTesting: boolean, isPending: boolean, id: string }> = {};
    for (const test of (result.data?.minerTests ?? [])) {
      const isTesting = 1000 * Number(test.overtimeAt) >= (Date.now() - 5000);
      provers[test.prover.toLowerCase()] = {
        isTesting: isTesting && !Boolean(test.result),
        isPending: test.result === null && !isTesting,
        id: test.id,
      }
    }
    return provers;
  }, [result]);

  const handleMinerTestDialog = (prover: string, type: 'retry' | 'cancel') => {
    const testId = minerTestsResult[prover].id;
    if (testId) {
      setDialog({
        type,
        testId,
        open: true,
      })
    }
  }

  const handleDialogClose = () => {
    setDialog({
      type: '',
      testId: '',
      open: false,
    })
    result.refetch();
  }

  return (
    <>
      <MinerTestTipDialog
        open={dialog.open}
        type={dialog.type as any}
        testId={dialog.testId}
        onClose={handleDialogClose}
      />
      <Card>
        <div className="flex items-center justify-between pb-2">
          <CardTitle>
            Your Provers
          </CardTitle>
          <ControllerBtn />
        </div>
        <Table>
          <TableHeader>
            <TableRow className="border-[#1F2D4E]">
              {Label.map((v) => (
                <TableHead
                  key={v}
                  className={`${v === "Action" ? "text-right" : "text-left"}`}
                >
                  {v}
                </TableHead>
              ))}
            </TableRow>
          </TableHeader>

          {
            isConnected ? (
              <TableBody>
                <TableRow>
                  <TableCell
                    colSpan={7}
                    className="h-[16px] bg-transparent"
                  ></TableCell>
                </TableRow>
                {provers.map(prover => (
                  prover.containers.length ?
                    prover.containers.map(container => (
                      <ProverRow
                        key={container.id}
                        stop={prover.stop}
                        prover={prover.id}
                        running={container.running}
                        name={prover.name}
                        created={container.created}
                        overtime={prover.overtime}
                        ptype={prover.ptype}
                        types={prover.types}
                        version={prover.version}
                        needUpgrade={container.needUpgrade}
                        testResult={minerTestsResult[prover.id]}
                        retryMinerTest={() => handleMinerTestDialog(prover.id, 'retry')}
                        cancelMinerTest={() => handleMinerTestDialog(prover.id, 'cancel')}
                      />
                    )) :
                    <ProverRow
                      key={prover.id}
                      stop={prover.stop}
                      prover={prover.id}
                      name={prover.name}
                      created={''}
                      overtime={prover.overtime}
                      ptype={prover.ptype}
                      types={prover.types}
                      version={prover.version}
                    />
                ))}
              </TableBody>
            ) : (
              <TableBody>
                <TableRow>
                  <TableCell
                    colSpan={7}
                    className="h-[160px] bg-transparent"
                  >
                    <div className="flex h-full w-full pt-6 justify-center items-center">
                      <div className="flex flex-col gap-1">
                        <NoData />
                        <div className="opacity-50 text-xl leading-normal">No Data</div>
                      </div>
                    </div>
                  </TableCell>
                </TableRow>
              </TableBody>
            )
          }
        </Table>
      </Card>
    </>
  );
};
export default UserProvers;
