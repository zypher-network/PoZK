"use client";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import {
  ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart";
import { memo, useState } from "react";
import { Bar, BarChart, XAxis } from "recharts";
import Loading from "@/components/icon/loading.svg";
import NoData from '@/components/icon/no-data.svg';
import useGetStatistics from "@/components/hooks/useGetStatistics";
import dayjs from "dayjs";
import useSubgraphStore from "@/components/state/subgraphStore";
import { useAccount } from "wagmi";
export const description = "An interactive bar chart";

const chartConfig = {
  desktop: {
    label: "value",
    color: "#9277FD",
  },
} satisfies ChartConfig;

const Charts = () => {
  const { isConnected } = useAccount();
  const stats = useGetStatistics();
  const reset = useSubgraphStore(state => state.reset);
  const [activeKey, setActiveKey] = useState<'day' | 'month' | 'year'>('day');
  return (
    <Card>
      <CardHeader className="flex flex-row justify-between items-center">
        <CardTitle>Earnings Statistics</CardTitle>
        <div className="flex flex-row justify-end items-center gap-[10px]">
          <div className="flex flex-row items-center px-[20px] h-[36px] bg-[#252C3E] rounded-[8px]">
            {["Day", "Month", "Year"].map((key, index) => {
              return (
                <ActiveItem
                  key={key}
                  isLast={index === 2}
                  active={activeKey === key.toLowerCase()}
                  onClick={() => setActiveKey(key.toLowerCase() as any)}
                  item={key}
                />
              );
            })}
          </div>
          <div
            onClick={() => reset('epoches')}
            className="flex flex-row justify-center items-center h-[36px] w-[36px] bg-[#252C3E] cursor-pointer  rounded-[8px]
          group
          "
          >
            <Loading className="opacity-50 group-hover:opacity-100 group-transition-[opacity]" />
          </div>
        </div>
      </CardHeader>
      {
        isConnected ? (
          <ChartContainer
            config={chartConfig}
            className="aspect-auto h-[300px] w-full"
          >
            <BarChart
              // accessibilityLayer
              data={stats[activeKey]}
              margin={{
                left: 12,
                right: 12,
              }}
            >
              <XAxis
                dataKey="label"
                tickLine={false}
                axisLine={false}
                tickMargin={8}
                style={{
                  fill: "rgba(255, 255, 255, 0.5", // 设置标签字体颜色为深灰色
                }}
                // minTickGap={32}
                tickFormatter={value => dayjs(value).format('MMM D')}
              />
              <ChartTooltip
                content={
                  <ChartTooltipContent
                    // className="w-[150px]"
                    nameKey="views"
                    labelFormatter={(value) => {
                      return new Date(value).toLocaleDateString("en-US", {
                        month: "short",
                        day: "numeric",
                        year: "numeric",
                      });
                    }}
                  />
                }
              />
              <defs>
                <linearGradient
                  id="desktop-gradient"
                  x1="0%"
                  y1="0%"
                  x2="0%"
                  y2="100%"
                >
                  <stop offset="0%" stopColor="#674EFF" />
                  <stop offset="100%" stopColor="#9277FD" />
                </linearGradient>
              </defs>
              <Bar
                dataKey="value"
                fill="url(#desktop-gradient)"
                label={{ position: 'top' }}
              />
            </BarChart>
          </ChartContainer>
        ) : (
          <div className="flex h-[300px] w-full justify-center items-center">
            <div className="flex flex-col gap-1">
              <NoData />
              <div className="opacity-50 text-xl leading-normal">No Data</div>
            </div>
          </div>
        )
      }
    </Card>
  );
};

const CustomBar = (props: any) => {
  const { ...rest } = props;
  const cornerRadius = 4;

  return <rect {...rest} rx={cornerRadius} ry={cornerRadius} />;
};

const renderCustomizedLabel = (props: any) => {
  const { x, y, width, value } = props;
  const radius = 10;

  return (
    <g>
      <text
        x={x + width / 2}
        y={y - radius}
        fill="#fff"
        textAnchor="middle"
        dominantBaseline="middle"
      >
        {value}
      </text>
    </g>
  );
};
const ActiveItem = ({
  item,
  active,
  isLast,
  onClick,
}: {
  item: string;
  active: boolean;
  isLast: boolean;
  onClick: () => void;
}) => {
  return (
    <>
      <h5
        onClick={onClick}
        className={`text-[16px] text-nowrap text-[#fff] cursor-pointer ${
          active ? "font-medium" : "font-light opacity-50"
        }
hover:opacity-100
transition-[opacity]
          `}
      >
        {item}
      </h5>
      {isLast ? null : (
        <div
          className="
      min-w-[1px]
      h-[16px]
      mx-[24px]
      bg-[#fff] opacity-10"
        />
      )}
    </>
  );
};
export default memo(Charts);
