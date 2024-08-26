import { Card } from "@/components/ui/card";
import Link from "next/link";
import SettingOutline from "@/components/icon/setting_outline.svg";

const ControllerBtn = () => {
  return (
    <div className="flex justify-between">
      <div />
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
    </div>
  );
};
export default ControllerBtn;
