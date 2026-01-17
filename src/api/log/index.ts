import { useMutation } from "@tanstack/react-query";
import { TauriClient } from "..";
import { useTranslateCommon } from "@hooks/useTranslate.hook";
import { notifications } from "@mantine/notifications";

export class LogModule {
  constructor(private readonly client: TauriClient) {}

  export_logs() {
    const useTranslateBase = (key: string, context?: { [key: string]: any }, i18Key?: boolean): string =>
      useTranslateCommon(`notifications.log_export.${key}`, { ...context }, i18Key);
    const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
      useTranslateBase(`error.${key}`, { ...context }, i18Key);
    const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
      useTranslateBase(`success.${key}`, { ...context }, i18Key);
    return useMutation({
      mutationFn: () => this.client.sendInvoke<string>("log_export"),
      onError: (e) => {
        console.error(e);
        notifications.show({ title: useTranslateErrors("title"), message: useTranslateErrors("message", { path: e }), color: "red.7" });
      },
      onSuccess: (path: string) => {
        notifications.show({
          title: useTranslateSuccess("title"),
          message: useTranslateSuccess("message", { path }),
          color: "green.7",
        });
      },
    });
  }
}
