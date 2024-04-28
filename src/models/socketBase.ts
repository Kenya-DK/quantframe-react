import WebSocket from "tauri-plugin-websocket-api";
import { ComposedListener } from "@utils/listener/Composed.listener";

export class SocketBase {
  private socket: WebSocket | undefined;
  private listener = new ComposedListener();
  private _host = "ws://localhost:7891";
  private _token = "";
  private _last_event_received: Date | undefined;
  private _reconnect_interval = 3 * 60 * 1000; // 3 minutes
  private _cookieKey = "JWT";
  public constructor(host: string, token?: string, cookieKey?: string) {
    this._host = host;
    if (token)
      this._token = token;
    if (cookieKey)
      this._cookieKey = cookieKey;
    console.log(`SocketBase initialized with host: ${this._host}, cookieKey: ${this._cookieKey}`);
    this.reconnect();
    this.scheduleReconnectionCheck();
  }

  private reconnect = async (): Promise<boolean> => {
    if (!this._token) return false;
    if (this.socket) {
      this.listener.fire("disconnect");
      this.socket.disconnect();
    }

    WebSocket.connect(this._host, {
      headers: {
        Cookie: `${this._cookieKey}=${this._token}`
      }
    }).then((ws) => {
      this.listener.fire("connect");
      console.log("Connected to socket successfully");
      ws.addListener((cd) => {
        try {
          if (!cd.data || typeof cd.data !== "string") return;
          const json = JSON.parse(cd.data as string);
          this._last_event_received = new Date();
          this.OnEvent(json);
        } catch (e) {
          console.log("Error while receiving event", e);
          this.socket = undefined;
          this.listener.fire("disconnect");
          this.listener.fire("error", e);
          return;
        }
      })
      this.socket = ws;
    }).catch((e) => {
      console.log("Error while connecting to socket", e);
      this.socket = undefined;
      this.listener.fire("disconnect");
      this.listener.fire("error", e);
      return false;
    })
    return true;

  };

  protected OnEvent = (_payload: Record<string, any>) => { }

  public FireEvent = (event: string, payload: Record<string, any>) => {
    this.listener.fire(event, payload);
  }
  private scheduleReconnectionCheck = () => {
    setInterval(async () => {
      // If the last event received is more than 3 minutes ago, reconnect
      if (this.shouldReconnect()) {
        {
          if (await this.reconnect())
            this._reconnect_interval = 3 * 60 * 1000; // Reset the interval to 3 minutes
          else
            this._reconnect_interval = 10 * 1000; // Retry after 10 seconds
          this._last_event_received = new Date();
        }
      }
    }, 1000);
  }

  public shouldReconnect = () => {
    if (!this._token) return false;
    if (!this._last_event_received) return true;
    return (new Date().valueOf() - this._last_event_received.valueOf()) > this._reconnect_interval;
  }

  public updateToken = (token: string) => {
    this._token = token;
    this.reconnect();
  }


  public on = (event: string, callback: (payload: any) => void) => {
    this.listener.add(event, callback);
  }
  public off = (event: string, callback: (payload: any) => void) => {
    this.listener.remove(event, callback);
  }
  public emit = async (payload: Record<string, any>) => {
    if (!this.socket) return;
    await this.socket.send(JSON.stringify(payload));
  }
  public isConnected = () => {
    return this.socket == undefined;
  }
}