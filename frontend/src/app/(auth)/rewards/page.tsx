"use client";
import Banner from "./components/banner/banner";
import EpochPhase from "./components/EpochPhase/EpochPhase";

export default function Rewards() {
  return (
    <div className="flex flex-col gap-[24px] pb-[54px]">
      <Banner />
      <EpochPhase />
    </div>
  );
}
