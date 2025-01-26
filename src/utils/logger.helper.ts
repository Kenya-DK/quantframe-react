import { invoke } from "@tauri-apps/api/core";

export interface LogSettings {
  console?: boolean;
  file?: string;
}

export enum LogLevel {
  Info = "info",
  Warning = "warning",
  Error = "error",
  Debug = "debug",
  Trace = "trace",
  Critical = "critical",
}

export const doLog = async (component: string, msg: string, level: LogLevel, settings: LogSettings = { console: true }) => {
  await invoke("log", {
    component,
    msg: msg,
    level: level,
    console: settings.console,
    file: settings.file,
  });
};

export const info = async (component: string, msg: string, settings: LogSettings = { console: true }) => {
  await doLog(component, msg, LogLevel.Info, settings);
};

export const warning = async (component: string, msg: string, settings: LogSettings = { console: true }) => {
  await doLog(component, msg, LogLevel.Warning, settings);
};

export const error = async (component: string, msg: string, settings: LogSettings = { console: true }) => {
  await doLog(component, msg, LogLevel.Error, settings);
};

export const debug = async (component: string, msg: string, settings: LogSettings = { console: true }) => {
  await doLog(component, msg, LogLevel.Debug, settings);
};

export const trace = async (component: string, msg: string, settings: LogSettings = { console: true }) => {
  await doLog(component, msg, LogLevel.Trace, settings);
};

export const critical = async (component: string, msg: string, settings: LogSettings = { console: true }) => {
  await doLog(component, msg, LogLevel.Critical, settings);
};
