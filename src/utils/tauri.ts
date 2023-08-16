// import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { ComposedListener } from "./listener/Composed.listener";

const listener = new ComposedListener();

(async () => {
  listen("message", (eventIn: { payload: { event: string, data: any } }) => {
    console.log("message", eventIn.payload);

    const { event, data } = eventIn.payload;
    if (event) {
      listener.fire(event, data);
    }
  });
})();


export const OnTauriEvent = (event: string, callback: (data: any) => void) => {
  listener.add(event, callback);
}
export const OffTauriEvent = (event: string, callback?: (data: any) => void) => {
  listener.remove(event, callback);
}