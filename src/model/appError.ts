import { ResponseError } from "../types";

export class AppError {
  constructor(public error: ResponseError) {}

  isWebSocket() {
    return this.error.component === "WebSocket";
  }

  isWebSocketError() {
    return (this.isWebSocket() && this.error.cause === "disconnected") || this.error.cause === "reconnecting";
  }
}
