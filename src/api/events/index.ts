import { listen } from "@tauri-apps/api/event";
import { ComposedListener } from "@utils/listener/Composed.listener";

export class EventModule {
  private listener = new ComposedListener();
  // private debug_filter: string[] = ["*"];
  private debug_filter: string[] = ["WFM:UpdateOrders", "Stock:UpdateStockItems"];
  private _colors = [
    'color: #000',
    'color: #000',
  ];
  constructor() {
    this.Initializer();
  }
  private async Initializer() {
    listen("message", (eventIn: { payload: { event: string, data: any } }) => {
      const { event, data } = eventIn.payload
      if (event)
        this.listener.fire(event, data);
      if (!this.debug_filter.includes(event) && !this.debug_filter.includes("*")) return;
      console.group("%cTauri Event", this._colors[0]);
      console.log(`%cEvent: %c${event}`, this._colors[0], "color: #000");
      console.log(`%cPayload:`, this._colors[0], data);;
      console.groupEnd();
    })
    listen("message_update", (eventIn: { payload: { event: string, operation: string, data: any } }) => {
      const { event, operation, data } = eventIn.payload;
      if (event && operation)
        this.listener.fire(event, { operation, data });
      if (!this.debug_filter.includes(event) && !this.debug_filter.includes("*")) return;
      console.group("%cTauri Event Data", this._colors[1]);
      console.log(`%cEvent: %c${event}`, this._colors[1], "color: #000");
      console.log(`%cOperation: %c${operation}`, this._colors[1], "color: #000");
      console.log(`%cPayload:`, this._colors[1], data,);;
      console.groupEnd();
    })
  }

  AddDebugFilter(filter: string) {
    if (this.debug_filter.includes(filter)) return;
    this.debug_filter.push(filter);
  }

  RemoveDebugFilter(filter: string) {
    this.debug_filter = this.debug_filter.filter((f) => f !== filter);
  }

  ClearDebugFilter() {
    this.debug_filter = [""];
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
