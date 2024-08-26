"use client";
import { useRouter } from "next/navigation";
import { memo, useCallback } from "react";
import { FaAngleLeft } from "react-icons/fa";

const Back = () => {
  const route = useRouter();
  const onClick = useCallback(() => {
    route.back();
  }, []);
  return <FaAngleLeft className="cursor-pointer" onClick={onClick} />;
};
export default memo(Back);
