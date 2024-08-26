import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { memo } from "react";
import SettingOutline from "@/components/icon/setting_outline.svg";

import Link from "next/link";
import NetworkListTable from "./networkListTable/networkListTable";

const NetworkList = () => {
  return (
    <Card>
      <CardHeader className="flex flex-row justify-between items-center pb-[24px]">
        <CardTitle>Your Provers</CardTitle>
        <Link href="/dashboard/controller">
          <Card
            className="bg-primary px-[16px] py-[0] gap-[4px] text-[16px] font-light
        flex flex-row justify-end items-center h-[36px] mt-0 cursor-pointer
        hover:bg-primary/90"
          >
            <SettingOutline />
            Controller
          </Card>
        </Link>
      </CardHeader>
      {/* {} */}
      {/* 前面是正在运行的   下载没运行的     其他 可 staking     */}
      {/* list    Controller  - 一台设备 */}
      <NetworkListTable isSetting={false} />
    </Card>
  );
};
export default memo(NetworkList);
// IP 名字没了
// controller ->   Controller 合约   添加和删除需要和合约交互   修改不需要（修改后端设置）
