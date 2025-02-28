import WebSocket from "@tauri-apps/plugin-websocket";
import { ComposedListener } from "@utils/listener/Composed.listener";
import { error, info } from "@utils/logger.helper";
export class SocketBase {
  private _host = "";
  private _token = "";
  private socket: WebSocket | undefined;
  private _colors = ["color: #FFD700", "color: #fff"];
  private listener = new ComposedListener();
  // Reconnect after 3 minutes, retry after 10 seconds if failed
  private _intervals = [3 * 60 * 1000, 10 * 1000];
  private _current_interval = this._intervals[0];
  private _last_received: Date | undefined = undefined;
  private _cookiePrefix = "JWT";
  private _user_agent = "QuantframeWS";
  public constructor(host: string, token?: string, cookieKey?: string, user_agent?: string) {
    this._host = host;
    if (token) this._token = token;
    if (cookieKey) this._cookiePrefix = cookieKey;
    if (user_agent) this._user_agent = user_agent;
    console.group("%cInitializing New Socket", this._colors[0]);
    console.log(`%cHost: %c${host}`, this._colors[0], this._colors[1]);
    console.log(`%cToken: %c${token}`, this._colors[0], this._colors[1]);
    console.log(`%cCookie Prefix: %c${cookieKey}`, this._colors[0], this._colors[1]);
    console.groupEnd();
    this.loop();
  }
  async connect() {
    if (this.socket) await this.disconnect();

    try {
      this.socket = await WebSocket.connect(this._host, {
        headers: {
          cookie: `${this._cookiePrefix}=${this._token}`,
          ["user-agent"]: this._user_agent,
        },
      });

      // If the connection is successful, set the interval to the first one
      this._current_interval = this._intervals[0];
      this.listener.fire("connect");

      // Create a listener for the socket
      this.socket.addListener((data) => {
        if (!data.data || typeof data.data !== "string") return;
        const json = JSON.parse(data.data as string);
        this._last_received = new Date();
        this.OnEvent(json);
      });
    } catch (e) {
      this._current_interval = this._intervals[1];
      error("Socket:Connect", `Will retry in ${this._current_interval / 1000} seconds`);
      this.listener.fire("error", e);
      await this.disconnect();
    }
  }

  async disconnect() {
    this.listener.fire("disconnect");
    try {
      if (this.socket) await this.socket.disconnect();
    } catch (e: any) {
      error("Socket:Disconnect", `Error while disconnecting: ${e.message | e}`);
    }
    this.socket = undefined;
  }

  loop() {
    setInterval(async () => {
      try {
        // If the last event received is more than 3 minutes ago, reconnect
        const timeDiff = this._last_received ? new Date().valueOf() - this._last_received.valueOf() : this._current_interval + 1;
        if (timeDiff > this._current_interval) {
          this._last_received = new Date();
          await this.connect();
          info("Socket:Reconnect", `Reconnecting to ${this._host}`);
        }
      } catch (e) {
        this.listener.fire("error", e);
        await this.disconnect();
      }
    }, 1000);
  }

  public updateToken = (token: string) => {
    this._token = token;
    this.connect();
  };
  public FireEvent = (event: string, payload: Record<string, any>) => {
    this.listener.fire(event, payload);
  };
  protected OnEvent = (_payload: Record<string, any>) => {};
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
