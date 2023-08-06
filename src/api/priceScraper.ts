import axios from 'axios'
// @ts-ignore no type definitions for this package
import axiosTauriAdapter from 'axios-tauri-adapter'
import { PriceHistoryDto } from "../types"
import { groupBy } from "../utils"
import { writeTextFile, copyFile, BaseDirectory, exists, removeFile } from '@tauri-apps/api/fs';
import * as Papa from 'papaparse';

import api from "."
const axiosInstance = axios.create({
  adapter: axiosTauriAdapter,
  baseURL: 'https://relics.run',
})
export default class PriceScraper {
  public constructor() { }
  private static getPriceHistory = async (platform: string, dayStr: string) => {
    const url = platform != "pc" ? `history/${platform}/price_history_${dayStr}.json` : `history/price_history_${dayStr}.json`
    const { data, status } = await axiosInstance.get(url, {})
    if (status.toString()[0] !== "2") return null
    return data as Record<string, PriceHistoryDto[]>
  }

  private static isValidPriceHistorys = (priceHistory: PriceHistoryDto[]) => {
    if (priceHistory.length == 0)
      return false

    if (priceHistory[0].mod_rank && priceHistory.length == 6)
      return true
    if (!priceHistory[0].mod_rank && priceHistory.length == 3)
      return true
    return false
  }

  public static list = async (priceHistoryDays: number): Promise<PriceHistoryDto[]> => {
    if (await exists('pricehistory.csv', { dir: BaseDirectory.AppConfig }))
      await copyFile('pricehistory.csv', "pricehistoryBackop.csv", { dir: BaseDirectory.AppConfig });
    type Item = { item_name: string, url_name: string }
    const wfmItemJson = await api.items.getTradableItems();
    const itemNameList = wfmItemJson.filter(x => !x.url_name.includes("relic")).map((item: Item) => item.url_name)
    const urlLookup: Record<string, string> = {};
    wfmItemJson.forEach((item: Item) => { urlLookup[item.item_name] = item.url_name; });
    console.log(itemNameList.length, urlLookup.length);

    // Get Last x days from now in array format
    const lastXDays = [...Array(priceHistoryDays + 1).keys()].map((i) => {
      const d = new Date()
      d.setDate(d.getDate() - (i + 1))
      return d.toISOString().split("T")[0]
    })

    // Get all orders for last x days
    let rows: PriceHistoryDto[] = []
    for (const dayStr of lastXDays) {
      const priceHistory = await PriceScraper.getPriceHistory("pc", dayStr)
      if (!priceHistory) continue
      for (const [key, historys] of Object.entries(priceHistory)) {
        if (!PriceScraper.isValidPriceHistorys(historys)) continue;
        rows = rows.filter(x => x.mod_rank != 0 && x.order_type != "Closed").concat(historys.map((x) => {
          return {
            name: urlLookup[key],
            datetime: x.datetime,
            order_type: x.order_type,
            volume: x.volume,
            min_price: x.min_price,
            max_price: x.max_price,
            range: x.max_price - x.min_price,
            median: x.median,
            avg_price: x.avg_price,
            mod_rank: x.mod_rank,
            item_id: x.item_id
          }
        }));
      }
    }
    // Write historys to csv

    const countByName = groupBy("name", rows);
    const popularItems = Object.entries(countByName).map((a) => {
      if (a[1].length >= 21)
        return a[0];
    }
    ).filter((x) => x).map((x) => x as string);
    rows = rows.filter((x) => popularItems.includes(x.name));

    // Sort by name
    rows = rows.sort((a, b) => a.name > b.name ? 1 : -1)
    const csv = Papa.unparse(rows);
    await writeTextFile('pricehistory.csv', csv, { dir: BaseDirectory.AppConfig });
    if (await exists('pricehistoryBackop.csv', { dir: BaseDirectory.AppConfig }))
      await removeFile('pricehistoryBackop.csv', { dir: BaseDirectory.AppConfig });
    return rows
  }
}