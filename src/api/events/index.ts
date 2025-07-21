import { listen } from "@tauri-apps/api/event";
import { ComposedListener } from "@utils/listener/Composed.listener";
import { TauriClient } from "..";

export class EventModule {
  private listener = new ComposedListener();
  constructor(private client: TauriClient) {
    this.Initializer();
  }
  private logEvent(event: string, operation: string | undefined, response: any) {
    this.client._loggingCount[event] = (this.client._loggingCount[event] || 0) + 1;
    if (this.client._logging.includes("*")) this.client._loggingCount["*"] = (this.client._loggingCount["*"] || 0) + 1;

    if (!this.client._logging.includes(event) && !this.client._logging.includes("*")) return;
    // Enhanced console theming
    let groupStyleBackground = "#257bebff";

    const groupStyle = `color: #ffffff; background: ${groupStyleBackground}; padding: 2px 8px; border-radius: 3px; font-weight: bold;`;
    const dataStyle = "color: #059669; font-weight: 600;";
    const responseStyle = "color: #0891b2; font-weight: 600;";
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
    console.group(`%cTauri Event - ${event}`, groupStyle);
    console.log(`%cTime:`, timeStyle, time);
    if (operation) console.log(`%cOperation: %c${operation}`, dataStyle, "color: #fff");
    if (response) console.log(`%cReceived:`, responseStyle, response);
    else console.log(`%cSuccess`, successStyle);
    console.groupEnd();
  }
  private async Initializer() {
    console.log("Event Module Initialized");

    listen("message_update", (eventIn: { payload: { event: string; operation: string; data: any } }) => {
      const { event, operation, data } = eventIn.payload;
      if (event && operation) this.listener.fire(event, { operation, data });
      this.logEvent(event, operation, data);
    });
    listen("message", (eventIn: { payload: { event: string; data: any } }) => {
      const { event, data } = eventIn.payload;
      if (event) this.listener.fire(event, data);
      this.logEvent(event, undefined, data);
    });
  }
  OnEvent<T>(event: string, callback: (data: T) => void) {
    this.listener.add(event, callback);
  }

  OffEvent<T>(event: string, callback: (data: T) => void) {
    this.listener.remove(event, callback);
    this.listener.clean();
  }

  FireEvent<T>(event: string, data: T) {
    this.listener.fire(event, data);
  }

  CleanEvent(event: string) {
    this.listener.clean(event);
  }
}
