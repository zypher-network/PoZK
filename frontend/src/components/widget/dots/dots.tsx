"use client";
import { memo } from "react";
import "./dots.css";
const Dots = () => {
  return <span className="inline-block animate-ellipsis ml-[4px]" />;
};
export default memo(Dots);
