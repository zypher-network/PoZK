"use client";

import GameApp from "./components/GameApp";
import PointRecord from "./components/PointRecord";

export default function MinerGame() {
  return (
    <div className="flex gap-[24px] pb-[54px]">
      <GameApp />
      <PointRecord />
    </div>
  );
}
