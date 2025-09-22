import { invoke } from "@tauri-apps/api/core";
import { AppModule } from "./app";
import { AuthModule } from "./auth";
import { EventModule } from "./events";
import { TauriTypes } from "$types";
import { UserModule } from "./user";
import { AnalyticsModule } from "./analytics";
import { AlertModule } from "./alert";
import { DashboardModule } from "./dashboard";
import { CacheModule } from "./cache";
import { LogModule } from "./log";
import { LiveScraperModule } from "./live_scraper";
import { StockItemModule } from "./stack_item";
import { DebugModule } from "./debug";
import { OrderModule } from "./order";
import { WishListModule } from "./wish_list";
import { StockRivenModule } from "./stack_riven";
import { AuctionModule } from "./auction";
import { ChatModule } from "./chat";

export class TauriClient {
  _logging: string[] = [];
  _loggingCount: Record<string, number> = {};

  // private _logs: string[] = [];
  constructor() {
    this.app = new AppModule(this);
    this.dashboard = new DashboardModule(this);
    this.alert = new AlertModule(this);
    this.events = new EventModule(this);
    this.auth = new AuthModule(this);
    this.user = new UserModule(this);
    this.cache = new CacheModule(this);
    this.analytics = new AnalyticsModule(this);
    this.live_scraper = new LiveScraperModule(this);
    this.log = new LogModule(this);
    this.stock_item = new StockItemModule(this);
    this.stock_riven = new StockRivenModule(this);
    this.wish_list = new WishListModule(this);
    this.debug = new DebugModule(this);
    this.order = new OrderModule(this);
    this.chat = new ChatModule(this);
    this.auction = new AuctionModule(this);
    this._logging = localStorage.getItem("tauri_logs") ? JSON.parse(localStorage.getItem("tauri_logs")!) : ["*"];
  }

  private logInvoke(command: string, data?: any, response?: any, error?: any) {
    this._loggingCount[command] = (this._loggingCount[command] || 0) + 1;
    if (this._logging.includes("*")) this._loggingCount["*"] = (this._loggingCount["*"] || 0) + 1;

    if (!this._logging.includes(command) && !this._logging.includes("*") && !error) return;
    // Enhanced console theming
    let groupStyleBackground = "#257bebff";
    if (error) groupStyleBackground = "#dc2626"; // Error background

    const groupStyle = `color: #ffffff; background: ${groupStyleBackground}; padding: 2px 8px; border-radius: 3px; font-weight: bold;`;
    const dataStyle = "color: #059669; font-weight: 600;";
    const responseStyle = "color: #0891b2; font-weight: 600;";
    const errorStyle = "color: #dc2626; font-weight: bold; background: #fef2f2; padding: 2px 4px; border-radius: 3px;";
    const successStyle = "color: #16a34a; font-weight: bold; background: #f0fdf4; padding: 2px 4px; border-radius: 3px;";
    const timeStyle = "color: #6b7280; ";

    const time = new Date().toLocaleTimeString("da-DK", {
      day: "2-digit",
      month: "2-digit",
      year: "numeric",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    });
    console.group(`%cTauri Invoke - ${command}`, groupStyle);
    console.log(`%cTime:`, timeStyle, time);
    if (data) console.log(`%cData:`, dataStyle, data);
    if (response) console.log(`%cResponse:`, responseStyle, response);
    if (error) console.error(`%cError:`, errorStyle, error);
    else console.log(`%cSuccess`, successStyle);
    console.groupEnd();
  }

  getLogging(search: string): { command: string; count: number }[] {
    let filteredLogs = this._logging;
    if (search) filteredLogs = this._logging.filter((log) => log.toLowerCase().includes(search.toLowerCase()));
    return filteredLogs.map((command) => {
      return { command, count: this._loggingCount[command] || 0 };
    });
  }

  private saveLogging() {
    localStorage.setItem("tauri_logs", JSON.stringify(this._logging));
  }

  async addLog(command: string) {
    if (!this._logging.includes(command)) this._logging.push(command);
    this.saveLogging();
  }

  async removeLog(command: string) {
    this._logging = this._logging.filter((log) => log !== command);
    this.saveLogging();
  }

  async sendInvoke<T>(command: string, data?: any): Promise<T> {
    if (data) data = this.convertToCamelCase(data);
    return new Promise((resolve, reject) => {
      invoke(command, data)
        .then((res) => {
          this.logInvoke(command, data, res);
          resolve(res as T);
        })
        .catch((err) => {
          this.logInvoke(command, data, undefined, err);
          reject(err);
        });
    });
  }

  convertToCamelCase(payload: Record<string, any>): Record<string, any> {
    const newPayload: any = {};
    for (const key in payload) {
      if (Object.prototype.hasOwnProperty.call(payload, key)) {
        const newKey = this.toCamelCase(key);
        newPayload[newKey] = payload[key];
      }
    }
    return newPayload;
  }

  toCamelCase(text: string): string {
    // Split the string by underscore
    const words = text.split("_");

    // Capitalize each word after the first
    const capitalizedWords = words.map((word, index) => (index === 0 ? word : word.charAt(0).toUpperCase() + word.slice(1)));

    // Join the words back together
    const camelCaseText = capitalizedWords.join("");

    return camelCaseText;
  }
  objectToParameters(obj: any): Array<string> {
    const searchParams: string[] = [];
    const entries = Object.entries(obj);
    for (let index = 0; index < entries.length; index++) {
      const element = entries[index];
      // Skip undefined and empty arrays
      if (element[1] === undefined || element[1] === "") continue;
      if (Array.isArray(element[1])) {
        const array = element[1] as any[];
        if (array.length <= 0) continue;

        searchParams.push(array.map((item) => `${element[0]}=${item}`).join("&"));
      } else searchParams.push(`${element[0]}=${element[1]}`);
    }
    return searchParams;
  }
  convertToTauriQuery(query: TauriTypes.StockItemControllerGetListParams) {
    let queryParams: any = { ...query };
    queryParams.pagination = { page: query.page, limit: query.limit };
    return queryParams;
  }
  // Modules
  app: AppModule;
  dashboard: DashboardModule;
  alert: AlertModule;
  events: EventModule;
  cache: CacheModule;
  auth: AuthModule;
  log: LogModule;
  order: OrderModule;
  auction: AuctionModule;
  analytics: AnalyticsModule;
  user: UserModule;
  live_scraper: LiveScraperModule;
  stock_item: StockItemModule;
  stock_riven: StockRivenModule;
  wish_list: WishListModule;
  debug: DebugModule;
  chat: ChatModule;
}

declare global {
  interface Window {
    api: TauriClient;
    data: any;
  }
}

window.api = new TauriClient();
const OnTauriEvent = <T>(event: string, callback: (data: T) => void) => window.api.events.OnEvent(event, callback);
const OnTauriDataEvent = <T>(event: string, callback: (data: { operation: TauriTypes.EventOperations; data: T }) => void) =>
  window.api.events.OnEvent(event, callback);

const OffTauriEvent = <T>(event: string, callback: (data: T) => void) => window.api.events.OffEvent(event, callback);
const OffTauriDataEvent = <T>(event: string, callback: (data: { operation: TauriTypes.EventOperations; data: T }) => void) =>
  window.api.events.OffEvent(event, callback);

const SendTauriEvent = async (event: string, data?: any) => window.api.events.FireEvent(event, data);
const SendTauriDataEvent = async (event: string, operation: TauriTypes.EventOperations, data: any) =>
  window.api.events.FireEvent(event, { operation, data });
const WFMThumbnail = (thumb: string) => `https://warframe.market/static/assets/${thumb}`;
const AddMetric = (metric: string, value: number | string) => {
  window.api.analytics.add_metric(metric, value);
};
export { WFMThumbnail, OnTauriEvent, OffTauriEvent, SendTauriEvent, OnTauriDataEvent, OffTauriDataEvent, SendTauriDataEvent, AddMetric };
export default window.api;
