import { atom } from "recoil";
export enum IKey {
  "Epoch/Phase" = "Epoch/Phase",
  "Start/End Date" = "Start/End Date",
  "Total Uptime" = "Total Uptime",
  "Estimated Reward" = "Estimated Reward",
  "Active" = "Active",
}
export const headerList: IKey[] = [
  IKey["Epoch/Phase"],
  IKey["Start/End Date"],
  IKey["Total Uptime"],
  IKey["Estimated Reward"],
  IKey["Active"],
];
export type IEpochPhase = {
  [K in keyof typeof IKey]: K extends "Epoch/Phase"
    ? { isOn: boolean; label: string }
    : string;
};

export const EpochPhaseData = atom<IEpochPhase[]>({
  key: "EpochPhaseData",
  default: [],
});

export const ChooseIndex = atom<number | undefined>({
  key: "ChooseIndex",
  default: undefined,
});
