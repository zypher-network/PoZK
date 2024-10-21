import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { memo } from "react";
import NetworkListTable from "../components/networkList/networkListTable/networkListTable";
import Link from "next/link";
import { FaPlus } from "react-icons/fa";
import Back from "@/components/widget/back/back";
import generateMetadata from "@/lib/generateMetadata";

export const metadata = generateMetadata({ tit: "Networks Setting" });

const Setting = () => {
  return (
    <Card>
      <CardHeader className="flex flex-row justify-between items-center pb-[24px]">
        <CardTitle className="flex items-center">
          <Back />
          Networks Setting
        </CardTitle>
        <Link href="/dashboard/setting/add">
          <Card
            className="bg-primary px-[16px] py-[0] gap-[4px] text-[16px] font-light
        flex flex-row justify-end items-center h-[36px] mt-0 cursor-pointer hover:bg-primary/90"
          >
            <FaPlus /> Add
          </Card>
        </Link>
      </CardHeader>
      <NetworkListTable isSetting={true} />
    </Card>
  );
};
export default memo(Setting);
