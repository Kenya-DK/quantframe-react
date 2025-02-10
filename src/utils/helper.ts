import { upperFirst } from "@mantine/hooks";
import { SubType } from "../api/types";

export interface GroupByDateSettings {
  labels?: string[];
  year?: boolean;
  month?: boolean;
  day?: boolean;
  hours?: boolean;
}
export interface TimeSpan {
  totalSeconds: number;
  totalMinutes: number;
  totalHours: number;
  totalDays: number;
  days: number;
  hours: number;
  minutes: number;
  seconds: number;
}
export const calculateTimeLeft = (endDate: Date): TimeSpan => {
  const now = new Date();
  const difference = endDate.getTime() - now.getTime();
  if (difference > 0)
    return {
      totalSeconds: Math.floor(difference / 1000),
      totalMinutes: Math.floor(difference / (1000 * 60)),
      totalHours: Math.floor(difference / (1000 * 60 * 60)),
      totalDays: Math.floor(difference / (1000 * 60 * 60 * 24)),
      days: Math.floor(difference / (1000 * 60 * 60 * 24)),
      hours: Math.floor((difference / (1000 * 60 * 60)) % 24),
      minutes: Math.floor((difference / 1000 / 60) % 60),
      seconds: Math.floor((difference / 1000) % 60),
    };
  else
    return {
      totalSeconds: 0,
      totalMinutes: 0,
      totalHours: 0,
      totalDays: 0,
      days: 0,
      hours: 0,
      minutes: 0,
      seconds: 0,
    };
};
export const getTimeLeftString = (timeLeft: TimeSpan): string => {
  let timeLeftString = "";
  if (timeLeft.days > 0) timeLeftString += `${timeLeft.days}d `;
  if (timeLeft.hours > 0) timeLeftString += `${timeLeft.hours}h `;
  if (timeLeft.minutes > 0) timeLeftString += `${timeLeft.minutes}m `;
  if (timeLeft.seconds > 0) timeLeftString += `${timeLeft.seconds}s`;
  if (timeLeftString === "") timeLeftString = "Expired";
  return timeLeftString;
};

/**
 * This function groups an array of items by date.
 *
 * @param {string} key - The key in the items to be used for grouping.
 * @param {Array<T>} items - The array of items to be grouped.
 * @param {GroupByDateSettings} settings - The settings for grouping which can include day, hours, month, and year.
 *
 * @returns {Array} - An array where the first element is an object with keys being the formatted date and values being the items falling under that date,
 * and the second element is an array of labels for each group.
 */
export const getGroupByDate = <T>(key: string, items: Array<T>, settings: GroupByDateSettings): [{ [key: string]: T[] }, string[]] => {
  const labels: string[] = [];
  const formatKey = (date: Date): string => {
    let key = "";
    if (settings.day) key += `${key.length > 0 ? " " : ""}` + date.getDate();
    if (settings.hours) key += `${key.length > 0 ? " " : ""}` + `${date.getHours()}:00`;
    if (settings.month) key += `${key.length > 0 ? " " : ""}` + date.getMonth();
    if (settings.year) key += `${key.length > 0 ? " " : ""}` + date.getFullYear();
    return key;
  };
  const groups = items.reduce((groups: { [key: string]: T[] }, item: T) => {
    const date = new Date((item as any)[key] || "");
    if (!groups[formatKey(date)]) {
      groups[formatKey(date)] = [];
      // labels.push(dayjs(date).format("DD/MM/YYYY HH:mm"));
      labels.push(formatKey(date));
    }
    groups[formatKey(date)].push(item);
    return groups;
  }, {});
  return [groups, settings.labels || labels];
};

type GroupBy<T> = Record<string, T[]>;
export const groupBy = <T, K extends keyof T>(key: K, array: T[]): GroupBy<T> => {
  return array.reduce((acc, cur) => {
    const groupByKey = cur[key] as unknown as string;
    (acc[groupByKey] = acc[groupByKey] || []).push(cur);
    return acc;
  }, {} as GroupBy<T>);
};

export const paginate = <T>(items: Array<T>, page: number, take: number) => {
  const startIndex = (page - 1) * take;
  const endIndex = page * take;
  return items.slice(startIndex, endIndex);
};

export const padTo2Digits = (num: number) => {
  return num.toString().padStart(2, "0");
};

// format number 1k, 1m, 1b
export const formatNumber = (num: number) => {
  if (num >= 1000000000) {
    return (num / 1000000000).toFixed(2).replace(/\.0$/, "") + " b.";
  }
  if (num >= 1000000) {
    return (num / 1000000).toFixed(2).replace(/\.0$/, "") + " m.";
  }
  if (num >= 1000) {
    return (num / 1000).toFixed(2).replace(/\.0$/, "") + " k.";
  }
  return num;
};

export const getCssVariable = (name: string) => {
  return getComputedStyle(document.documentElement).getPropertyValue(name);
};

export const GetSubTypeDisplay = (subType: SubType | undefined) => {
  if (!subType || Object.keys(subType).length == 0) return "";
  const { rank, variant, amber_stars, cyan_stars } = subType;
  let display = "";
  if (rank != undefined) display += `(R${rank})`;
  if (variant) display += ` [${upperFirst(variant)}]`;
  if (amber_stars) display += ` ${amber_stars}A`;
  if (cyan_stars) display += ` ${cyan_stars}C`;
  return display;
};

export const CreateTradeMessage = (prefix: string, items: { price: number; name: string }[], suffix: string) => {
  const groupByPrice = groupBy("price", items);
  const prices = Object.keys(groupByPrice)
    .map((key) => parseInt(key))
    .sort((a, b) => b - a);
  let message = `${prefix} `;
  for (const price of prices) {
    const names = groupByPrice[price].map((item) => item.name);
    message += `${names.join("")}${price}p `;
  }
  message = message.trim();
  message += suffix;
  return message;
};
