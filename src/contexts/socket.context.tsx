import React, { createContext, useContext, useEffect, useState } from "react";
import WebSocket from "tauri-plugin-websocket-api";
import { useAuthContext } from ".";
import { SendSocketEvent, debug, error } from "../utils";
import { Wfm } from "../types";

type SocketContextProps = {
  socket: WebSocket | undefined;
}

type SocketContextProviderProps = {
  children: React.ReactNode;
}

export const SocketContext = createContext<SocketContextProps>({
  socket: undefined,
});
export const useSocketContextContext = () => useContext(SocketContext);
export const SocketContextProvider = ({ children }: SocketContextProviderProps) => {
  const [socket, setSocket] = useState<WebSocket | undefined>();
  const { user } = useAuthContext();
  const [token, setToken] = useState<string | undefined>();
  const [last_event_received, setLastEventReceived] = useState<Date | undefined>();

  const SetupSocket = async (token: string | undefined) => {
    if (!token) return;
    const ws = await WebSocket.connect("wss://warframe.market/socket?platform=pc", {
      headers: {
        Cookie: `JWT=${token}`
      },
    });
    ws.addListener((cd) => {
      const json = JSON.parse(cd.data as string) as { type: string, payload: any };
      const event = json.type.replace("@WS/", "");

      if (!event.includes("MESSAGE/ONLINE_COUNT"))
        debug("SocketEvent", JSON.stringify({
          event,
          payload: json.payload
        }), {
          file: "socketEvents.log",
        });
      if (event.includes("ERROR"))
        throw new Error(event);
      SendSocketEvent(event, json.payload);
    });
    return ws;
  }
  useEffect(() => {
    setToken(user?.access_token);
  }, [user?.access_token])

  useEffect(() => {
    const reconnect = async () => {
      const ws = await SetupSocket(token);
      setSocket(ws);
    };
    reconnect().catch((e) => {
      error("Socket", e, {
        file: "socket.log",
      });
      console.log("Error while connecting to socket");
    });
  }, [token, last_event_received]);


  useEffect(() => {
    let tempDate = new Date();
    const interval = setInterval(() => {
      // if (last_event_received && (new Date().valueOf() - last_event_received.valueOf()) > 180000) {
      if (tempDate && (new Date().valueOf() - tempDate.valueOf()) > 180000) {
        // Disconnect socket if it exists
        setSocket((preSocket) => { if (preSocket) preSocket.disconnect(); return undefined; });
        console.log("Socket connection lost, reconnecting");
        tempDate = new Date();
        setLastEventReceived(tempDate);
      }
    }, 3000); // Update every second

    return () => {
      clearInterval(interval);
    };
  }, []);


  useEffect(() => {
    if (!socket) return;
    socket.send(JSON.stringify({
      type: "@WS/USER/SET_STATUS",
      payload: user?.status || Wfm.UserStatus.Invisible
    }));

    return () => { };
  }, [socket]);


  return (
    <SocketContext.Provider value={{ socket }}>
      {children}
    </SocketContext.Provider>
  )
}


