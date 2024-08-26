import { GQLEpoch } from '@/types/epoch';
import dayjs from 'dayjs';

export const calcDuration = (dayStart: Date, dayEnd?: Date) => {
  const [day, hr, min] = dayjs.duration(((dayjs(dayEnd).unix()) - dayjs(dayStart).unix()) * 1000).format('D_HH_mm').split('_');
  return `${day}Day - ${hr}Hr - ${min}min`;
}

export const isTodaysEpoch = (epoch: GQLEpoch) => {
  return dayjs(epoch.endAt ? +epoch.endAt * 1000 : undefined).isToday();
}