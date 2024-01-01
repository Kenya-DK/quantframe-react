import { createStyles, useMantineTheme } from "@mantine/core";
import { Wfm } from "../types";

export interface GroupByDateSettings {
  labels?: string[]
  year?: boolean;
  month?: boolean;
  day?: boolean;
  hours?: boolean;
}
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
  const labels: string[] = []
  const formatKey = (date: Date): string => {
    let key = "";
    if (settings.day)
      key += `${key.length > 0 ? " " : ""}` + date.getDate();
    if (settings.hours)
      key += `${key.length > 0 ? " " : ""}` + `${date.getHours()}:00`;
    if (settings.month)
      key += `${key.length > 0 ? " " : ""}` + date.getMonth();
    if (settings.year)
      key += `${key.length > 0 ? " " : ""}` + date.getFullYear();
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
}

type GroupBy<T> = Record<string, T[]>;
export const groupBy = <T, K extends keyof T>(key: K, array: T[]): GroupBy<T> => {
  return array.reduce((acc, cur) => {
    const groupByKey = cur[key] as unknown as string;
    (acc[groupByKey] = acc[groupByKey] || []).push(cur);
    return acc;
  }, {} as GroupBy<T>);

}

export const paginate = <T>(items: Array<T>, page: number, take: number) => {
  const startIndex = (page - 1) * take;
  const endIndex = page * take;
  return items.slice(startIndex, endIndex);
}
export const padTo2Digits = (num: number) => {
  return num.toString().padStart(2, '0');
}

/**
 * Returns the CSS class for the given order status, which can be used to style the order status in the UI.
 * @param status - The order status to get the CSS class for.
 * @returns The CSS class for the given order status.
 */
const useStyles = createStyles(() => {
  const boxShadow = `inset 4px 0 0 0`;

  return {
    default: {
      ['td:first-of-type']: {
        boxShadow: `${boxShadow} ${getOrderStatusColorCode("")};`,
      },
    },
    tolowprofile: {
      ['td:first-of-type']: {
        boxShadow: `${boxShadow} ${getOrderStatusColorCode(Wfm.OrderStatus.ToLowProfile)};`,
      },
    },
    pending: {
      ['td:first-of-type']: {
        boxShadow: `${boxShadow} ${getOrderStatusColorCode(Wfm.OrderStatus.Pending)};`,
      },
    },
    live: {
      ['td:first-of-type']: {
        boxShadow: `${boxShadow} ${getOrderStatusColorCode(Wfm.OrderStatus.Live)};`,
      },
    },
    inactive: {
      ['td:first-of-type']: {
        boxShadow: `${boxShadow} ${getOrderStatusColorCode(Wfm.OrderStatus.Inactive)};`,
      },
    },
    no_offers: {
      ['td:first-of-type']: {
        boxShadow: `${boxShadow} ${getOrderStatusColorCode(Wfm.OrderStatus.NoOffers)};`,
      },
    },
    no_buyers: {
      ['td:first-of-type']: {
        boxShadow: `${boxShadow} ${getOrderStatusColorCode(Wfm.OrderStatus.NoBuyers)};`,
      },
    }

  }
});
export const getOrderStatusColorCode = (status: string) => {
  const theme = useMantineTheme();
  switch (status) {
    case Wfm.OrderStatus.Inactive:
      return theme.colors.red[7];
    case Wfm.OrderStatus.Live:
      return theme.colors.green[7];
    case Wfm.OrderStatus.Pending:
      return theme.colors.violet[7];
    case Wfm.OrderStatus.ToLowProfile:
      return theme.colors.orange[7];
    case Wfm.OrderStatus.NoOffers:
      return theme.colors.pink[7];
    case Wfm.OrderStatus.NoBuyers:
      return theme.colors.yellow[7];
    default:
      return theme.colors.gray[2];
  }
};
export const getOrderStatusColorClass = (status: string) => {
  const { classes } = useStyles();
  switch (status) {
    case Wfm.OrderStatus.Inactive:
      return classes.inactive;
    case Wfm.OrderStatus.Live:
      return classes.live;
    case Wfm.OrderStatus.Pending:
      return classes.pending;
    case Wfm.OrderStatus.ToLowProfile:
      return classes.tolowprofile;
    case Wfm.OrderStatus.NoOffers:
      return classes.no_offers;
    default:
      return classes.default;
  }
};
export const getUserStatusColor = (status: Wfm.UserStatus) => {
  switch (status) {
    case Wfm.UserStatus.Ingame:
      return "mediumpurple";
    case Wfm.UserStatus.Online:
      return "green";
    case Wfm.UserStatus.Invisible:
    default:
      return "darkred";
  }
};