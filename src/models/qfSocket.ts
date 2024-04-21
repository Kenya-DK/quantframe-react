import { SocketBase } from "./socketBase";

export class QfSocket extends SocketBase {
  static jwt: { name: string } = { name: "QuantframeMain" };
  public constructor(host: string, token: string) {
    super(host, token);
  }

  // Override the OnEvent method
  protected OnEvent = (data: Record<string, any>) => {
    try {
      const { id, payload } = data as { id: string, payload: any };
      const payloadData = JSON.parse(atob(payload));
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
console.log("QfSocket");
export const qfSocket = new QfSocket("ws://localhost:7891", btoa(JSON.stringify(QfSocket.jwt)));