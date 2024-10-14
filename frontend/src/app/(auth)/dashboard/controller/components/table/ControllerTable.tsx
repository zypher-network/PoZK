"use client";
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { memo, useCallback, useMemo, useState } from "react";
import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from "@tanstack/react-table";
import { Address, zeroAddress } from "viem";
import { Button } from "@/components/ui/button";
import {
  CaretSortIcon,
  ChevronDownIcon,
  DotsHorizontalIcon,
} from "@radix-ui/react-icons";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useRecoilValue } from "recoil";
import { ControllerList, IControllerItem } from "../../../state/dashboardState";
import { Card } from "@/components/ui/card";
import SettingDialog from "./settingDialog";
import { IAction } from "@/types/IAction";
import { useToast } from "@/components/ui/use-toast";
import useControllerStore from "@/components/state/controllerStore";
import { useShallow } from "zustand/react/shallow";

const ControllerTable = () => {
  const { toast } = useToast();
  const [account, setAccount] = useState<Address | undefined>();
  const [action, setAction] = useState<IAction | undefined>();
  // const controllerList = useRecoilValue(ControllerList);
  const store = useControllerStore(useShallow(state => ({ fetching: state.fetching, list: state.controllers, active: state.active })));
  const [rowSelection, setRowSelection] = useState({});

  const controllerHandler = useCallback(
    (action: IAction, item: IControllerItem) => {
      setAction(action);
      setAccount(item.address);
    },
    []
  );
  const copyHandle = useCallback((key: string) => {
    navigator.clipboard.writeText(key || "");
    toast({
      variant: "success",
      title: "Copy Success!",
    });
  }, []);
  const columns: ColumnDef<IControllerItem>[] = useMemo(() => {
    return [
      {
        accessorKey: "status",
        header: "Status",
        cell: ({ row }) =>
          row.getValue("status") === "on" ? <p className="text-[#26d962]">{row.getValue("status")}</p> : <p className="text-sm text-[#9e9e9e]">{row.getValue("status")}</p>
      },
      {
        accessorKey: "address",
        header: ({ column }) => {
          return (
            <Button
              variant="ghost"
              onClick={() =>
                column.toggleSorting(column.getIsSorted() === "asc")
              }
            >
              Address
              <CaretSortIcon className="ml-2 h-4 w-4" />
            </Button>
          );
        },
        cell: ({ row }) => (
          <div className="lowercase">{row.getValue("address")}</div>
        ),
      },
      {
        id: "actions",
        header: "Actions",
        enableHiding: false,
        cell: ({ row }) => {
          const item = row.original;
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
                <DropdownMenuItem onClick={() => copyHandle(item.address)}>
                  Copy Address
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                {item.status === 'off' ? (
                  <DropdownMenuItem
                    onClick={() => controllerHandler("set", item)}
                  >
                    Set Controller
                  </DropdownMenuItem>
                ) : null}
                {/* <DropdownMenuItem
                  onClick={() => controllerHandler("delete", item)}
                >
                  Delete Controller
                </DropdownMenuItem> */}
                <DropdownMenuItem
                  onClick={() => controllerHandler("export", item)}
                >
                  Export Secret Key
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          );
        },
      },
    ];
  }, [store.list, store.active]);
  const table = useReactTable({
    data: store.list,
    columns: columns,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    onRowSelectionChange: setRowSelection,
    state: { rowSelection },
  });
  return (
    <>
      <Card className="mt-[40px] max-w-[700px]">
        <h3>Controller List</h3>
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header, index) => {
                  return (
                    <TableHead
                      key={header.id}
                      className={
                        index === headerGroup.headers.length - 1
                          ? "text-right"
                          : "text-left"
                      }
                    >
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                            header.column.columnDef.header,
                            header.getContext()
                          )}
                    </TableHead>
                  );
                })}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            <TableRow>
              <TableCell
                colSpan={3}
                className="h-[16px] bg-transparent"
              ></TableCell>
            </TableRow>
            {table.getRowModel().rows?.length ? (
              table.getRowModel().rows.map((row, index) => (
                <TableRow
                  key={row.id}
                  data-state={row.getIsSelected() && "selected"}
                >
                  {row.getVisibleCells().map((cell, index) => (
                    <TableCell
                      key={cell.id}
                      className={`bg-slate-800 p-0 h-[44px] ${
                        row.getVisibleCells().length - 1 === index
                          ? "justify-end"
                          : ""
                      }`}
                    >
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext()
                      )}
                    </TableCell>
                  ))}
                </TableRow>
              ))
            ) : (
              <TableRow>
                <TableCell
                  colSpan={columns.length}
                  className="bg-slate-800 h-24 justify-center"
                >
                  {store.fetching ? "Loading" : "No results."}
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </Card>
      <SettingDialog
        action={action ?? "add"}
        onOpenChange={(open) => {
          if (!open) {
            setAction(undefined);
            setAccount(undefined);
          }
        }}
        open={!!action}
        address={account ?? zeroAddress}
      />
    </>
  );
};
export default memo(ControllerTable);
