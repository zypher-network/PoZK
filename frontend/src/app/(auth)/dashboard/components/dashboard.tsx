"use client";

import Banner from "./banner/banner";
import Charts from "./charts/charts";
// import { useController } from "../hook/useController";
import Provers from "./provers";
import { useControllerRouter } from "../hook/useControllerRouter";
import StakeDialog from "./dialog/StakeDialog/StakeDialog";

const DashboardPage = () => {
  // useController();
  useControllerRouter();
  return (
    <div className="flex flex-col gap-[24px] pb-[54px]">
      <Banner />
      <Charts />
      <Provers />
      <StakeDialog />
    </div>
  );
};
export default DashboardPage;
