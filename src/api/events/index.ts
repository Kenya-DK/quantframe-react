import { listen } from "@tauri-apps/api/event";
import { ComposedListener } from "../../utils/listener/Composed.listener";
import { TauriClient } from "..";

export class EventModule {
  private listener = new ComposedListener();
  constructor(private readonly client: TauriClient) {
    this.Initializer();
  }
  private async Initializer() {
    listen("message", (eventIn: { payload: { event: string, data: any } }) => {
      console.log("Message Api: ", eventIn.payload);

      const { event, data } = eventIn.payload;
      if (event)
        this.listener.fire(event, data);
    })
    listen("message_update", (eventIn: { payload: { event: string, operation: string, data: any } }) => {
      console.log("Message update api: ", eventIn.payload);

      const { event, operation, data } = eventIn.payload;
      if (event && operation)
        this.listener.fire(event, { operation, data });
    })
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
