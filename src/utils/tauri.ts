// import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { ComposedListener } from "./listener/Composed.listener";

const listener = new ComposedListener();
export const OnTauriEvent = <T>(event: string, callback: (data: T) => void) => {
  listener.add(event, callback);
}

(async () => {
  listen("message", (eventIn: { payload: { event: string, data: any } }) => {
    console.log("message", eventIn.payload);

    const { event, data } = eventIn.payload;
    if (event) {
      listener.fire(event, data);
    }
  });

  OnTauriEvent<{ type: string, operation: string, data: any }>("update_data", ({ type, operation, data }) => {
    listener.fire(`update_data:${type}`, { operation, data });
  });
})();

export const OnTauriUpdateDataEvent = <T>(type: string, callback: (data: { operation: string, data: T }) => void) => {
  listener.add(`update_data:${type}`, callback);
}

export const OffTauriEvent = (event: string, callback?: (data: any) => void) => {
  listener.remove(event, callback);
}