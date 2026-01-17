import { invoke } from "@tauri-apps/api";
import { AppModule } from "./app";
import { AuctionModule } from "./auction";
import { AuthModule } from "./auth";
import { ChatModule } from "./chat";
import { DebugModule } from "./debug";
import { LiveScraperModule } from "./live_scraper";
import { OrderModule } from "./order";
import { StockModule } from "./stock";
import { TransactionModule } from "./transaction";
import { EventModule } from "./events";
import { StatisticModule } from "./statistic";
import { CacheModule } from "./cache";
import { ErrOrResult, QfSocketEventOperation } from "./types";
import { LogModule } from "./log";
import { AnalyticsModule } from "./analytics";
import { WFMSocket } from "../models/wfmSocket";
import { NotificationModule } from "./notification";

export class TauriClient {
  constructor() {
    this.app = new AppModule(this);
    this.auction = new AuctionModule(this);
    this.auth = new AuthModule(this);
    this.chat = new ChatModule(this);
    this.debug = new DebugModule(this);
    // this.items = new ItemModule(this);
    this.live_scraper = new LiveScraperModule(this);
    this.order = new OrderModule(this);
    this.stock = new StockModule(this);
    this.transaction = new TransactionModule(this);
    this.events = new EventModule();
    this.notification = new NotificationModule(this);
    this.statistic = new StatisticModule(this);
    this.cache = new CacheModule(this);
    this.log = new LogModule(this);
    this.analytics = new AnalyticsModule(this);
  }

  async sendInvoke<T>(command: string, data?: any): Promise<ErrOrResult<T>> {
    console.log(`Sending invoke: ${command}`, data);
    if (data) data = this.convertToCamelCase(data);
    return new Promise((resolve) => {
      invoke(command, data)
        .then((res) => {
          resolve([null, res] as ErrOrResult<T>);
        })
        .catch((err) => {
          resolve([err, null] as ErrOrResult<T>);
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

  // Modules
  app: AppModule;
  auction: AuctionModule;
  auth: AuthModule;
  chat: ChatModule;
  debug: DebugModule;
  // items: ItemModule;
  live_scraper: LiveScraperModule;
  order: OrderModule;
  stock: StockModule;
  transaction: TransactionModule;
  events: EventModule;
  notification: NotificationModule;
  statistic: StatisticModule;
  cache: CacheModule;
  analytics: AnalyticsModule;
  log: LogModule;
}

declare global {
  interface Window {
    api: TauriClient;
    wfmSocket: WFMSocket;
    data: any;
  }
}

window.api = new TauriClient();
// (window as any).api = api as
const OnTauriEvent = <T>(event: string, callback: (data: T) => void) => window.api.events.OnEvent(event, callback);
const OnTauriDataEvent = <T>(event: string, callback: (data: { operation: QfSocketEventOperation; data: T }) => void) =>
  window.api.events.OnEvent(event, callback);

const OffTauriEvent = <T>(event: string, callback: (data: T) => void) => window.api.events.OffEvent(event, callback);
const OffTauriDataEvent = <T>(event: string, callback: (data: { operation: QfSocketEventOperation; data: T }) => void) =>
  window.api.events.OffEvent(event, callback);

const SendTauriEvent = async (event: string, data?: any) => window.api.events.FireEvent(event, data);
const SendTauriDataEvent = async (event: string, operation: QfSocketEventOperation, data: any) =>
  window.api.events.FireEvent(event, { operation, data });

const WFMThumbnail = (thumb: string) => `https://warframe.market/static/assets/${thumb}`;
const SendNotificationToWindow = async (title: string, message: string) => window.api.notification.sendSystemNotification(title, message);

export {
  OnTauriEvent,
  OnTauriDataEvent,
  OffTauriEvent,
  OffTauriDataEvent,
  SendTauriEvent,
  SendTauriDataEvent,
  WFMThumbnail,
  SendNotificationToWindow,
};
export default window.api;
