import { SocketBase } from "./socketBase";

export class WFMSocket extends SocketBase {
  public constructor(host: string) {
    console.log("Initializing WFM Socket");
    super(host);
  }

  // Override the OnEvent method
  protected OnEvent = (data: Record<string, any>) => {
    const { type, payload } = data as { type: string; payload: any };
    const event = type.replace("@WS/", "");
    this.FireEvent(event, payload);
  };
}
const wfmSocket = new WFMSocket("wss://warframe.market/socket?platform=pc");
// const wfmSocket = new WFMSocket("ws://127.0.0.1:7369");
window.wfmSocket = wfmSocket;
export default wfmSocket;
