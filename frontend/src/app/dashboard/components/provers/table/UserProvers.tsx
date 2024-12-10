"use client";
import { useAccount } from "wagmi";
import { Card, CardTitle } from "@/components/ui/card";
import { useMemo } from "react";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import ProverRow from "./ProverRow";
import { UserProver } from "@/types/IProver";
import ControllerBtn from "../../controllerBtn/controllerBtn";
import NoData from '@/components/icon/no-data.svg';

interface IUserProvers {
  provers: UserProver[];
}

const UserProvers: React.FC<IUserProvers> = ({ provers }) => {
  const { isConnected } = useAccount();
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

  return (
    <>
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
                        version={prover.version}
                        needUpgrade={container.needUpgrade}
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
