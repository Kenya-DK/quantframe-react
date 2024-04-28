import { SocketBase } from "./socketBase";

export class QfSocket extends SocketBase {
  public constructor(host: string, token: string, deviceId: string) {
    super(host, token, deviceId);
  }

  // Override the OnEvent method
  protected OnEvent = (data: Record<string, any>) => {
    try {
      const { id, payload } = data as { id: string, payload: any };
      const payloadData = JSON.parse(payload);
      console.log("Received event", payloadData);
      this.FireEvent(id, payloadData);
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
export const qfSocket = new QfSocket("ws://localhost:7891", "Quantframe", "DEVICEID");