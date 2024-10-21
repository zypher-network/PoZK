"use client";
import { Card, CardTitle } from "@/components/ui/card";
import { useMemo } from "react";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import ProverRow from "./ProverRow";
import { UserProver } from "@/types/IProver";
import ControllerBtn from "../../controllerBtn/controllerBtn";

interface IUserProvers {
  provers: UserProver[];
}

const UserProvers: React.FC<IUserProvers> = ({ provers }) => {
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
                    prover={prover.id}
                    running={container.running}
                    name={prover.name}
                    created={container.created}
                    overtime={prover.overtime}
                    version={prover.version}
                    needUpgrade={container.needUpgrade}
                  />
                )) :
                <ProverRow
                  key={prover.id}
                  prover={prover.id}
                  name={prover.name}
                  created={''}
                  overtime={''}
                  version={prover.version}
                />
            ))}
          </TableBody>
        </Table>
      </Card>
    </>
  );
};
export default UserProvers;
