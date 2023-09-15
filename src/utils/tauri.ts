// import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { ComposedListener } from "./listener/Composed.listener";
import { isPermissionGranted, sendNotification } from "@tauri-apps/api/notification";

const listener = new ComposedListener();

/**
 * Registers a callback function to be called when a Tauri event with the given name is emitted.
 * The callback function will receive the event data as its argument.
 * @param event The name of the Tauri event to listen for.
 * @param callback The function to be called when the event is emitted.
 */
export const OnTauriEvent = <T>(event: string, callback: (data: T) => void) => {
  listener.add(event, callback);
}

// Handle events from rust side
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

/**
 * Registers a callback function to be called when a Tauri "update_data" event with the given type is emitted.
 * The callback function will receive an object with the operation and data properties as its argument.
 * @param type The type of the "update_data" event to listen for.
 * @param callback The function to be called when the event is emitted.
 */
export const OnTauriUpdateDataEvent = <T>(type: string, callback: (data: { operation: string, data: T }) => void) => {
  listener.add(`update_data:${type}`, callback);
}

/**
 * Removes the given callback function from the list of functions to be called when a Tauri event with the given name is emitted.
 * If no callback function is provided, all callbacks for the given event are removed.
 * @param event The name of the Tauri event to remove the callback from.
 * @param callback The callback function to remove (optional).
 */
export const OffTauriEvent = (event: string, callback?: (data: any) => void) => {
  listener.remove(event, callback);
}

/**
 * Emits a Tauri event with the given name and data (optional).
 * @param event The name of the Tauri event to emit.
 * @param data The data to pass along with the event (optional).
 */
export const SendTauriEvent = async (event: string, data?: any) => {
  listener.fire(event, data);
}

/**
 * Emits a Tauri "update_data" event with the given type and data (optional).
 * @param event The type of the "update_data" event to emit.
 * @param data The data to pass along with the event (optional).
 */
export const SendTauriUpdateDataEvent = async (event: string, data?: any) => {
  listener.fire(`update_data:${event}`, data);
}

/**
 * Sends a notification to the user with the given title and body.
 * Throws an error if permission to send notifications has not been granted.
 * @param title The title of the notification.
 * @param body The body of the notification.
 */
export const SendNotificationToWindow = async (title: string, body: string) => {
  let permissionGranted = await isPermissionGranted();
  if (!permissionGranted) throw new Error("Permission not granted");
  if (permissionGranted)
    sendNotification({ title: title, body: body });
}