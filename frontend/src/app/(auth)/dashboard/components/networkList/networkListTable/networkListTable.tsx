"use client";
import { memo, useMemo } from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  // TableHeaderRow,
  TableRow,
} from "@/components/ui/table";
import NoData from "@/components/widget/NoData/NoData";
import Recommendation from "../../dialog/Recommendation/Recommendation";
import { Button } from "@/components/ui/button";
import { useToSettingPath } from "../../../hook/toSettingPath";
enum IKey {
  "Status" = "Status",
  "IP" = "IP",
  "Prover" = "Prover",
  "Time Connected" = "Time Connected",
  "Staking Amount" = "Staking Amount",
  "Points Earned" = "Points Earned",
  "Active" = "Active",
}
const headerList: IKey[] = [
  IKey["Status"],
  IKey["IP"],
  IKey["Prover"],
  IKey["Time Connected"],
  IKey["Staking Amount"],
  IKey["Points Earned"],
];

type IRequired = {
  [K in keyof typeof IKey]: K extends "IP"
    ? { name: string; link: string }
    : string;
};
// type INetworkItem = {
//   [K in keyof typeof IKey as K extends "Active" ? never : K]: string;
// } & {
//   Active?: string; // 或者 Active?: boolean;
// };
type INetworkItem = {
  [K in keyof typeof IKey]: K extends "IP"
    ? { name: string; link: string }
    : K extends "Active"
    ? undefined
    : string;
};
const data: INetworkItem[] = [
  {
    Status: "afsadf",
    IP: {
      name: "afsadf2222",
      link: "adaf",
    },
    Prover: "afsadf",
    "Time Connected": "afsadf",
    "Staking Amount": "afsadf",
    "Points Earned": "afsadf",
    Active: undefined,
  },
  {
    Status: "afsadf2222",
    IP: {
      name: "afsadf2222",
      link: "adaf",
    },
    Prover: "afsadf2222",
    "Time Connected": "afsadf2222",
    "Staking Amount": "afsadf2222",
    "Points Earned": "afsadf2222",
    Active: undefined,
  },
];
const NetworkListTable = ({ isSetting }: { isSetting: boolean }) => {
  const toSettingPath = useToSettingPath("/dashboard/setting");
  const HeaderList = useMemo((): IKey[] => {
    if (isSetting) {
      return [...headerList, IKey.Active];
    }
    return headerList;
  }, [JSON.stringify(headerList), isSetting]);

  return (
    <Table>
      <TableHeader>
        <TableRow className="border-[#1F2D4E]">
          {HeaderList.map((v, index) => (
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
            colSpan={HeaderList.length}
            className="h-[16px] bg-transparent"
          ></TableCell>
        </TableRow>
        <TableRow>
          <TableCell
            colSpan={HeaderList.length}
            className="h-[200px] bg-transparent"
          >
            <NoData />
          </TableCell>
        </TableRow>
        {data.map((item, index) => (
          <TableRow key={index}>
            {HeaderList.map((v, rowIndex) => {
              if (typeof item[v] === "string") {
                return (
                  <TableCell
                    className={`
            ${[0, 1, 2].includes(rowIndex) ? "justify-start" : "justify-end"}
            ${
              v === "Staking Amount" || v === "Points Earned"
                ? "text-[#FACC16]"
                : "text-[#FFFFFF]"
            }
            `}
                    key={v}
                  >
                    {item[v]}
                  </TableCell>
                );
              }
              if (v === IKey.Active) {
                return (
                  <TableCell
                    key={v}
                    className="flex flex-row items-center justify-end gap-[16px]"
                  >
                    <Button
                      variant="outline"
                      onClick={() => toSettingPath({ action: "edit" })}
                    >
                      Edit
                    </Button>
                    <Button
                      variant="outline"
                      className="text-[#E44042] border-[#E44042]"
                      onClick={() => toSettingPath({ action: "delete" })}
                    >
                      Delete
                    </Button>
                  </TableCell>
                );
              }
              if (v === IKey.IP) {
                return (
                  <TableCell key={v}>
                    <div className="flex flex-row items-center gap-[8px]">
                      {item[v].name}
                      {/* <Recommendation image={""} tag={""} /> */}
                    </div>
                  </TableCell>
                );
              }
              return null;
            })}
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
};
export default memo(NetworkListTable);
