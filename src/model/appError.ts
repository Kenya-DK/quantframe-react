import { ResponseError } from "../types";

export class AppError {
  constructor(public error: ResponseError) {}

  isWebSocket() {
    return this.error.component === "WebSocket";
  }

  isV1() {
    return this.error.context.version === "https://api.warframe.market/v1";
  }

  isWebSocketError() {
    return (this.isWebSocket() && this.error.cause === "disconnected") || this.error.cause === "reconnecting";
  }
}
