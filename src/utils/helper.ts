import { upperFirst } from "@mantine/hooks";
import { ItemWithMeta, ItemWithSubType, TauriTypes } from "$types";
import api from "@api/index";
import { resolveResource } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/core";
import { isCustomSound, stripCustomSoundPrefix } from "@utils/sound";

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
let cachedCustomSoundsPath: string | undefined;
export const PlaySound = async (fileName: string, volume: number = 1.0) => {
  try {
    let assetUrl: string;

    if (isCustomSound(fileName)) {
      const { join } = await import('@tauri-apps/api/path');
      if (!cachedCustomSoundsPath) {
        cachedCustomSoundsPath = await api.sound.getCustomSoundsPath();
      }
      const soundPath = await join(cachedCustomSoundsPath, stripCustomSoundPrefix(fileName));
      assetUrl = convertFileSrc(soundPath);
    } else {
      const resourcePath = await resolveResource(`resources/sounds/${fileName}`);
      assetUrl = convertFileSrc(resourcePath);
    }

    const audio = new Audio(assetUrl);
    audio.volume = volume;
    await audio.play();
  } catch (error) {
    console.error(`Error playing sound ${fileName}:`, error);
    // Fallback logic
    try {
      const resourcePath = await resolveResource(`resources/sounds/cat_meow.mp3`);
      const assetUrl = convertFileSrc(resourcePath);
      const audio = new Audio(assetUrl);
      audio.volume = volume;
      audio.play();
    } catch (fallbackError) {
      console.error("Error playing fallback sound:", fallbackError);
    }
  }
};
(window as any).PlaySound = PlaySound;
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
export const GroupByKey = <T, K extends keyof T>(key: K, array: T[]): GroupBy<T> => {
  // If the key contains a dot, it means it's a nested key
  if (key.toString().includes(".")) {
    return array.reduce((acc, cur) => {
      const keys = key.toString().split(".");
      let groupByKey = (cur as any)[keys[0]] as unknown as string;
      for (let i = 1; i < keys.length; i++) groupByKey = (groupByKey as any)[keys[i]] as unknown as string;
      (acc[groupByKey] = acc[groupByKey] || []).push(cur);
      return acc;
    }, {} as GroupBy<T>);
  } else
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
// Round to nearest base (default 5)
export const Round = (x: number, base = 5) => {
  return Math.round(x / base) * base;
};
export interface DisplaySettings {
  prefix?: string;
  value?: string | number;
  override?: boolean;
  suffix?: string;
}

export const ApplyTemplate = (template: string, data: Record<string, DisplaySettings>) => {
  return template.replace(/<([^>]+)>/g, (_, key) => {
    const { value, prefix, suffix } = data[key] || {};
    return String(`${prefix || ""}${value || ""}${suffix || ""}`);
  });
};

export const GetSubTypeDisplay = (value: ItemWithMeta, template: string, settings: Record<string, DisplaySettings> = {}): string => {
  return ApplyTemplate(template, GetSubTypeDisplayObject(value, settings));
};
const setValue = (display: Record<string, DisplaySettings>, settings: Record<string, DisplaySettings> = {}, key: string, value: string | number) => {
  display[key] = { ...settings[key], value };
};

const extractSubType = (value: ItemWithMeta): ItemWithSubType | undefined => {
  if (!value) return;
  if ("sub_type" in value && value.sub_type) return value.sub_type as TauriTypes.SubType;
  if ("properties" in value) return value as TauriTypes.SubType;
};

export const GetSubTypeDisplayObject = (value: ItemWithMeta, settings: Record<string, DisplaySettings> = {}): Record<string, DisplaySettings> => {
  const subType = extractSubType(value);
  if (!subType || Object.keys(subType).length === 0) return {};

  const display: Record<string, DisplaySettings> = {};

  // Rank
  if (subType.rank !== undefined) setValue(display, settings, "rank", subType.rank);

  // Variant / subtype
  if ("variant" in subType) setValue(display, settings, "variant", upperFirst(subType.variant ?? ""));

  if ("subtype" in subType) setValue(display, settings, "subtype", upperFirst(subType.subtype ?? ""));

  // Stars (normalize different property names)
  const amber = (subType as any).amber_stars ?? (subType as any).amberStars;
  if (amber !== undefined) setValue(display, settings, "amber_stars", amber);

  const cyan = (subType as any).cyan_stars ?? (subType as any).cyanStars;
  if (cyan !== undefined) setValue(display, settings, "cyan_stars", cyan);

  return display;
};

export const GetChatLinkName = async (
  value: ItemWithMeta,
  settings: Record<string, DisplaySettings> = {}
): Promise<Record<string, DisplaySettings>> => {
  if (!value) return { link: { value: "<Unknown Item>" } };

  let item =
    ("wfm_id" in value && value.wfm_id && (await api.cache.getTradableItemById(value.wfm_id))) ||
    ("wfm_weapon_id" in value && value.wfm_weapon_id && (await api.cache.getRivenWeaponsById(value.wfm_weapon_id))) ||
    ("wfm_id" in value && value.wfm_id && (await api.cache.getRivenWeaponsById(value.wfm_id)));

  const display: Record<string, DisplaySettings> = {};

  // Price
  const price = ("price" in value ? value.price : undefined) ?? ("list_price" in value ? value.list_price : undefined);

  if (price !== undefined) setValue(display, settings, "price", price);

  if ("name" in value) setValue(display, settings, "name", `${value.name}`);

  // Chat link
  if (item && item.unique_name) {
    const chatLink = await api.cache.get_chat_link(item.unique_name!);
    display["link"] = { value: chatLink.link };
    if (chatLink.suffix) display["type"] = { value: chatLink.suffix };
  }

  // Mod name
  if ("mod_name" in value) setValue(display, settings, "mod_name", `${value.mod_name}`);

  // Subtype display
  Object.assign(display, GetSubTypeDisplayObject(value, settings));

  return display;
};

export const GetChatLinkNameMultiple = async (
  value: ItemWithMeta[],
  settings?: Record<string, DisplaySettings>
): Promise<Record<string, DisplaySettings>[]> => {
  let results: Record<string, DisplaySettings>[] = [];
  for (let item of value) results.push(await GetChatLinkName(item, settings));
  return results;
};
export const GetItemDisplay = (
  value: ItemWithMeta,
  tradableItems: TauriTypes.CacheTradableItem[] = [],
  weapons: TauriTypes.CacheRivenWeapon[] = []
) => {
  if (!value) return "Unknown Item";
  let fullName = undefined;
  if ("weapon_name" in value && !fullName) fullName = value.weapon_name;
  if ("item_name" in value && !fullName) fullName = value.item_name;
  if ("wfm_id" in value && !fullName) fullName = tradableItems?.find((i) => i.wfm_id === value.wfm_id)?.name;
  if ("wfm_id" in value && !fullName) fullName = weapons?.find((i) => i.wfm_id === value.wfm_id)?.name;
  if ("name" in value && !fullName) fullName = value.name;
  if ("wfm_url" in value && !fullName) fullName = tradableItems?.find((i) => i.wfm_url_name === value.wfm_url)?.name;

  if ("properties" in value && value.properties && "mod_name" in value.properties) fullName += ` ${value.properties.mod_name}`;
  if ("mod_name" in value) fullName += ` ${value.mod_name}`;
  return fullName || "Unknown Item";
};

export const decodeHtmlEntities = (input: string): string => {
  try {
    const doc = new DOMParser().parseFromString(input, "text/html");
    return doc.documentElement.textContent || "";
  } catch {
    return input;
  }
};

// At the top of your component or in a separate utils file
export const getSafePage = (requestedPage: number | undefined, totalPages: number | undefined): number => {
  const page = requestedPage ?? 1;
  const maxPages = totalPages ?? 1;
  return Math.min(page, maxPages);
};
