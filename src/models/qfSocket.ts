import { SocketBase } from "./socketBase";

export class QfSocket extends SocketBase {
  public constructor(host: string, token: string, deviceId: string) {
    super(host, token, deviceId);
  }

  // Override the OnEvent method
  protected OnEvent = (data: Record<string, any>) => {
    try {
      const { event, payload } = data as { event: string, payload: any };
      let payloadData = payload;
      console.log("Received event", payloadData);
      if (payload === "string")
        this.FireEvent(event, JSON.parse(payload));
      else
        this.FireEvent(event, payload);
    } catch (error) {
      console.error("Error while processing event", error);
    }
  }

  // Override the emit method
  public SendEvent = async (event: string, payload: Record<string, any>) => {
    this.emit({
      Event: event,
      Payload: btoa(JSON.stringify(payload))
    });
  }
}
export const qfSocket = new QfSocket("ws://localhost:9999", "Quantframe", "DEVICEID");