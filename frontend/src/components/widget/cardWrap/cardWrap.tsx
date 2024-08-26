"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import Back from "../back/back";
import { memo, ReactNode } from "react";

const CardWrap = ({
  children,
  title,
  titleFr,
}: {
  children: ReactNode;
  title: string;
  titleFr?: ReactNode;
}) => {
  return (
    <Card className="mb-[48px]">
      <CardHeader className="flex flex-row justify-between items-center pb-[48px]">
        <CardTitle className="flex items-center">
          <Back />
          {title}
        </CardTitle>
        {titleFr}
      </CardHeader>
      <CardContent>{children}</CardContent>
    </Card>
  );
};
export default memo(CardWrap);
