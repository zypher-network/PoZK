"use client";
import { memo } from "react";
import ControllerTable from "./table/ControllerTable";
import CardWrap from "@/components/widget/cardWrap/cardWrap";
import Link from "next/link";
import { FaPlus } from "react-icons/fa";
import { Card } from "@/components/ui/card";
import useControllerStore from "@/components/state/controllerStore";

const ControllerPage = () => {
  const active = useControllerStore(state => state.active);
  return (
    <CardWrap
      title="Controller List"
      titleFr={
        <Link href="/dashboard/controller/add">
          <Card
            className="bg-primary px-[16px] py-[0] gap-[4px] text-[16px] font-light
      flex flex-row justify-center items-center h-[36px] mt-0 cursor-pointer
      w-[100px]
      hover:bg-primary/90"
          >
            <FaPlus />
            Add
          </Card>
        </Link>
      }
    >
      {active ? null : (
        <p className="text-[#ffeb3b] bg-[#303511] rounded-sm py-[4px] px-[20px] max-w-[700px]">
          Please set a Controller Handler
        </p>
      )}
      <ControllerTable />
    </CardWrap>
  );
};
export default memo(ControllerPage);
