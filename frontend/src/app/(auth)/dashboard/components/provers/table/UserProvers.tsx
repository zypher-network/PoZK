"use client";
import { Card, CardTitle } from "@/components/ui/card";
import { useMemo } from "react";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import ProverRow from "./ProverRow";
import { UserProver } from "@/types/IProver";
import useSubgraphStore from "@/components/state/subgraphStore";

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
      "Action", // delete
    ];
  }, []);

  return (
    <>
      <Card>
        <CardTitle>Your Provers</CardTitle>
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
                colSpan={2}
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
