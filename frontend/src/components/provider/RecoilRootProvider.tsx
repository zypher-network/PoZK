"use client";
import { memo, ReactNode } from "react";
import { RecoilRoot } from "recoil";

const RecoilRootProvider = ({ children }: { children: ReactNode }) => {
  return <RecoilRoot>{children}</RecoilRoot>;
};
export default memo(RecoilRootProvider);
