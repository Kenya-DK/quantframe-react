// import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { ComposedListener } from "./listener/Composed.listener";
import { invoke } from "@tauri-apps/api";
import { ProgressReport } from "../types";
import { notifications } from "@mantine/notifications";
import i18next from "i18next";

const listener = new ComposedListener();
const progress: { [key: string]: ProgressReport } = {};

/**
 * Registers a callback function to be called when a Tauri event with the given name is emitted.
 * The callback function will receive the event data as its argument.
 * @param event The name of the Tauri event to listen for.
 * @param callback The function to be called when the event is emitted.
 */
export const OnTauriEvent = <T>(event: string, callback: (data: T) => void) => {
  console.log("OnTauriEvent", event, callback);
  listener.add(event, callback);
}

// Handle events from rust side
(async () => {
  listen("message", (eventIn: { payload: { event: string, data: any } }) => {
    console.log("Message", eventIn.payload);

    const { event, data } = eventIn.payload;
    if (event) {
      listener.fire(event, data);
    }
  });

  OnTauriEvent<{ type: string, operation: string, data: any }>("Client:Update", ({ type, operation, data }) => {
    listener.fire(`Client:Update:${type}`, { operation, data });
  });
})();

/**
 * Registers a callback function to be called when a Tauri "Client:Update" event with the given type is emitted.
 * The callback function will receive an object with the operation and data properties as its argument.
 * @param type The type of the "Client:Update" event to listen for.
 * @param callback The function to be called when the event is emitted.
 */
export const OnTauriUpdateDataEvent = <T>(type: string, callback: (data: { operation: string, data: T }) => void) => {
  listener.add(`Client:Update:${type}`, callback);
}

// This function allows you to listen for a socket event with the given event name.
// When the event is triggered, the provided callback function will be called with the event data.
export const OnSocketEvent = <T>(event: string, callback: (data: T) => void) => {
  // The 'add' method adds a listener for the event with the given name.
  // When the event is triggered, the callback function will be called with the event data.
  listener.add(`Client:Socket:${event}`, callback);
}

// This function allows you to stop listening for a socket event with the given event name.
// It removes the provided callback function from the list of listeners for the event.
export const OffSocketEvent = <T>(event: string, callback: (data: T) => void) => {
  // The 'remove' method removes the listener for the event with the given name.
  // After this method is called, the callback function will no longer be called when the event is triggered.
  listener.remove(`Client:Socket:${event}`, callback);
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
 * Emits a Tauri "Client:Update" event with the given type and data (optional).
 * @param event The type of the "Client:Update" event to emit.
 * @param data The data to pass along with the event (optional).
 */
export const SendTauriUpdateDataEvent = async (event: string, data?: any) => {
  listener.fire(`Client:Update:${event}`, data);
}

// This function sends a socket event with the given event name and data.
// The event name is prefixed with "Client:Socket:" to distinguish it from other types of events.
export const SendSocketEvent = async (event: string, data?: any) => {
  console.log("SendSocketEvent", event, data);

  // The 'fire' method triggers the event with the given name and data.
  listener.fire(`Client:Socket:${event}`, data);
}

/**
 * Sends a notification to the user with the given title and body.
 * Throws an error if permission to send notifications has not been granted.
 * @param title The title of the notification.
 * @param body The body of the notification.
 */
export const SendNotificationToWindow = async (title: string, message: string, icon?: string, sound?: string) => {
  await invoke("show_notification", { title, message, icon, sound })
}


/**
 * Sends a notification to Discord using a specified webhook.
 * 
 * @param title - The title of the notification.
 * @param content - The content of the notification.
 * @param webhook - The webhook URL to send the notification to.
 * @param user_ids - Optional. An array of user IDs to mention in the notification.
 */
export const SendDiscordNotification = async (title: string, content: string, webhook: string, user_ids?: string[]) => {
  await invoke("send_message_to_discord", { title, content, webhook, user_ids })
}

OnTauriEvent("Client:Update:Progress", (data: ProgressReport) => {
  const { id, title, i18n_key, isCompleted, values } = data;
  let notification = {
    id,
    title,
    message: i18next.t(`progress.${i18n_key}`, values),
    autoClose: isCompleted,
    withCloseButton: false,
  }
  if (!progress[data.id])
    notifications.show(notification);
  else
    notifications.update(notification);
  if (isCompleted)
    delete progress[data.id];
  else
    progress[data.id] = data;
});
