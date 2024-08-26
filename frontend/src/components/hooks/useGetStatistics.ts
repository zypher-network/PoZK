import { useShallow } from "zustand/react/shallow";
import useSubgraphStore from "../state/subgraphStore";
import { useMemo } from "react";
import dayjs from "dayjs";
import BigNumberJs, { BM18 } from "@/lib/BigNumberJs";

type StatDay = {
  label: string;
  value: number;
}

const useGetStatistics = () => {
  const { epoches, reward } = useSubgraphStore(useShallow(state => ({ epoches: state.epoches, reward: state.reward })));
  const dayEpoches = useMemo(() => {
    const epochMap: Record<string, string[]> = new Array(100).fill('').reduce((prev, _, idx) => {
      prev[dayjs().subtract(+idx, 'day').format('YYYY-MM-DD')] = [];
      return prev;
    }, []);
    for (const epoch of epoches.data) {
      const date = dayjs(epoch.endAt ? +epoch.endAt * 1000 : undefined).format('YYYY-MM-DD');
      epochMap[date].push(epoch.id);
    }
    return epochMap;
  }, [epoches.data]);

  const epochRewards = useMemo(() => {
    return (reward.data?.claimList ?? [])
      .reduce((prev, curr) => {
        prev[curr.epoch] = curr.claim ?? curr.estimate;
        return prev;
      }, {} as Record<string, string>);
  }, [reward.data]);

  const dayData = useMemo(() => {
    const day: StatDay[] = [];
    for (const date in dayEpoches) {
      const epoches = dayEpoches[date];
      const statDay = {
        label: date,
        value: epoches.reduce((prev, curr) => prev.plus(epochRewards[curr] ?? '0'), new BigNumberJs('0')).div(BM18).toNumber(),
      };
      day.push(statDay);
      if (day.length === 15) {
        break;
      }
    }
    day.reverse();
    return day;
  }, [dayEpoches, epochRewards]);

  const monthData = useMemo(() => {
    const epochMap: Record<string, number> = new Array(10).fill('').reduce((prev, _, idx) => {
      prev[dayjs().subtract(+idx, 'month').format('YYYY-MM')] = 0;
      return prev;
    }, {});
    for (const date in dayEpoches) {
      const epoches = dayEpoches[date];
      const [y, m] = date.split('-');
      const value = epoches.reduce((prev, curr) => prev.plus(epochRewards[curr] ?? '0'), new BigNumberJs('0')).div(BM18).toNumber();
      if (epochMap[`${y}-${m}`] !== undefined) {
        epochMap[`${y}-${m}`] = epochMap[`${y}-${m}`] + value;
      }
    }
    return Object.entries(epochMap).map(([label, value]) => ({ label, value })).reverse();
  }, [dayEpoches, epochRewards]);
  return { day: dayData, month: monthData, year: [] };
}

export default useGetStatistics;
