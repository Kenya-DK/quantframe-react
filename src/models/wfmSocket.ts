import { SocketBase } from "./socketBase";

export class WFMSocket extends SocketBase {
  public constructor(host: string) {
    super(host);
  }

  // Override the OnEvent method
  protected OnEvent = (data: Record<string, any>) => {
    const { type, payload } = data as { type: string, payload: any };
    const event = type.replace("@WS/", "");
    console.log("Event received", event, payload);
    this.FireEvent(event, payload);
  }
}
export const wfmSocket = new WFMSocket("wss://warframe.market/socket?platform=pc");