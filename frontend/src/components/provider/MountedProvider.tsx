"use client";
import { ReactNode } from "react";
import { useMounted } from "../hooks/useMounted";
import dayjs from 'dayjs';
import duration from 'dayjs/plugin/duration';
import isToday from 'dayjs/plugin/isToday';
dayjs.extend(duration);
dayjs.extend(isToday)

const MountedProvider = ({ children }: { children: ReactNode }) => {
  const mounted = useMounted();
  if (!mounted) {
    return <></>;
  }
  return <>{children}</>;
};
export default MountedProvider;
