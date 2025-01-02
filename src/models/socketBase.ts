import WebSocket from "tauri-plugin-websocket-api";
import { ComposedListener } from "@utils/listener/Composed.listener";

export class SocketBase {
  private socket: WebSocket | undefined;
  private listener = new ComposedListener();
  private _host = "";
  private _token = "";
  private _colors = ["color: #000", "color: #000"];
  private _last_event_received: Date | undefined;
  private _reconnect_interval = 3 * 60 * 1000; // 3 minutes
  private _cookiePrefix = "JWT";
  public constructor(host: string, token?: string, cookieKey?: string) {
    this._host = host;
    if (token) this._token = token;
    if (cookieKey) this._cookiePrefix = cookieKey;
    console.group("%cInitializing New Socket", this._colors[0]);
    console.log(`%cHost: %c${host}`, this._colors[0], this._colors[1]);
    console.log(`%cToken: %c${token}`, this._colors[0], this._colors[1]);
    console.log(`%cCookie Prefix: %c${cookieKey}`, this._colors[0], this._colors[1]);
    console.groupEnd();
    this.reconnect();
    this.scheduleReconnectionCheck();
  }

  private reconnect = async (): Promise<boolean> => {
    if (!this._token) return false;
    if (this.socket) {
      this.listener.fire("disconnect");
      await this.socket.disconnect();
      this.socket = undefined;
    }

    WebSocket.connect(this._host, {
      headers: {
        Cookie: `${this._cookiePrefix}=${this._token}`,
      },
    })
      .then((ws) => {
        this.listener.fire("connect");
        console.group("%cReconnecting to Socket", this._colors[0]);
        console.log(`%cHost: %c${this._host}`, this._colors[0], this._colors[1]);
        console.log(`%cSocket:`, this._colors[0], ws);
        console.log(`%cSuccess`, "color: green");
        console.groupEnd();
        this._last_event_received = new Date();
        ws.addListener((cd) => {
          try {
            if (!cd.data || typeof cd.data !== "string") return;
            const json = JSON.parse(cd.data as string);
            this._last_event_received = new Date();
            this.OnEvent(json);
          } catch (e) {
            this.socket = undefined;
            this.listener.fire("disconnect");
            this.listener.fire("error", e);
          }
        });
        this.socket = ws;
      })
      .catch((e) => {
        console.group("%cReconnecting to Socket", this._colors[0]);
        console.log(`%cHost: %c${this._host}`, this._colors[0], this._colors[1]);
        console.log(`%cError`, "color: red", e);
        console.groupEnd();
        this.socket = undefined;
        this.listener.fire("disconnect");
        this.listener.fire("error", e);
        return false;
      });
    return true;
  };

  protected OnEvent = (_payload: Record<string, any>) => {};

  public FireEvent = (event: string, payload: Record<string, any>) => {
    this.listener.fire(event, payload);
  };
  private scheduleReconnectionCheck = () => {
    setInterval(async () => {
      try {
        // If the last event received is more than 3 minutes ago, reconnect
        if (this.shouldReconnect()) {
          {
            if (await this.reconnect()) this._reconnect_interval = 1 * 60 * 1000; // Reset the interval to 3 minutes
            else this._reconnect_interval = 10 * 1000; // Retry after 10 seconds
            this._last_event_received = new Date();
          }
        }
      } catch (e) {
        this.socket = undefined;
      }
    }, 1000);
  };

  public shouldReconnect = () => {
    if (!this._token) return false;
    if (!this._last_event_received) return true;
    return new Date().valueOf() - this._last_event_received.valueOf() > this._reconnect_interval;
  };

  public updateToken = (token: string) => {
    this._token = token;
    this.reconnect();
  };
  public on = (event: string, callback: (payload: any) => void) => {
    this.listener.add(event, callback);
  };
  public off = (event: string, callback: (payload: any) => void) => {
    this.listener.remove(event, callback);
  };
  public emit = async (payload: Record<string, any>) => {
    if (!this.socket) return;
    await this.socket.send(JSON.stringify(payload));
  };
  public isConnected = () => {
    return this.socket == undefined;
  };
}
