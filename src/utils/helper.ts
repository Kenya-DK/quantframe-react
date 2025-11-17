import { upperFirst } from "@mantine/hooks";
import { ItemWithMeta, ItemWithSubType, TauriTypes } from "$types";
import api from "@api/index";

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
export const GroupByDate = <T>(key: string, items: Array<T>, settings: GroupByDateSettings): [{ [key: string]: T[] }, string[]] => {
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

export const GetSubTypeDisplay = (value: ItemWithMeta) => {
  let subType: ItemWithSubType | undefined;
  if (!value) return undefined;
  if ("sub_type" in value && !subType) subType = value.sub_type as TauriTypes.SubType;
  if ("properties" in value && !subType) subType = value as TauriTypes.SubType;

  if (!subType || Object.keys(subType).length == 0) return "";
  let display = "";
  if (subType.rank != undefined) display += `(R${subType.rank})`;
  if ("variant" in subType) display += ` ${upperFirst(subType.variant || "")}`;
  if ("subtype" in subType) display += ` ${upperFirst(subType.subtype || "")}`;

  // if (subType.variant ) display += ` [${upperFirst(subType.variant)}]`;
  if ("amber_stars" in subType) display += ` ${subType.amber_stars}A`;
  if ("amberStars" in subType) display += ` ${subType.amberStars}A`;
  if ("cyan_stars" in subType) display += ` ${subType.cyan_stars}C`;
  if ("cyanStars" in subType) display += ` ${subType.cyanStars}C`;
  return display;
};

export const GetChatLinkName = async (value: ItemWithMeta): Promise<TauriTypes.ChatLink> => {
  if (!value) return { link: "<Unknown Item>", suffix: "", prefix: "" };

  let findItem = undefined;
  if ("wfm_id" in value) findItem = await api.cache.getTradableItemById(value.wfm_id || "");
  if (!findItem) return { link: "<Unknown Item>", suffix: "", prefix: "" };

  let chatLink = await api.cache.get_chat_link(findItem!.unique_name || "");
  if (chatLink.suffix != "") chatLink.suffix = "<SP>" + chatLink.suffix;
  let subTypeDisplay = GetSubTypeDisplay(value);
  if (subTypeDisplay != "") chatLink.suffix += "<SP>" + subTypeDisplay;
  return chatLink;
};
export const GetChatLinkNameMultiple = async (value: ItemWithMeta[]): Promise<TauriTypes.ChatLink[]> => {
  let results: TauriTypes.ChatLink[] = [];
  for (let item of value) results.push(await GetChatLinkName(item));
  return results;
};
export const GetItemDisplay = (value: ItemWithMeta, tradableItems?: TauriTypes.CacheTradableItem[]) => {
  if (!value) return "Unknown Item";
  let fullName = undefined;
  if ("weapon_name" in value && !fullName) fullName = value.weapon_name;
  if ("item_name" in value && !fullName) fullName = value.item_name;
  if ("wfm_id" in value && !fullName) fullName = tradableItems?.find((i) => i.wfm_id === value.wfm_id)?.name || value.wfm_id || "Unknown Item";
  if ("wfm_url" in value && !fullName)
    fullName = tradableItems?.find((i) => i.wfm_url_name === value.wfm_url)?.name || value.wfm_url || "Unknown Item";

  if ("properties" in value && value.properties && "mod_name" in value.properties) fullName += ` ${value.properties.mod_name}`;
  if ("mod_name" in value) fullName += ` ${value.mod_name}`;
  return fullName || "Unknown Item";
};
// At the top of your component or in a separate utils file
export const getSafePage = (requestedPage: number | undefined, totalPages: number | undefined): number => {
  const page = requestedPage ?? 1;
  const maxPages = totalPages ?? 1;
  return Math.min(page, maxPages);
};
