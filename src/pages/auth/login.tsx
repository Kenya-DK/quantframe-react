import { Center, Progress } from "@mantine/core";
import { LogInForm } from "@components/forms/LogIn";
import api, { SendTauriDataEvent } from "@api/index";
import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { TextTranslate } from "@components/TextTranslate";
import { QfSocketEvent, QfSocketEventOperation, ResponseError } from "@api/types";
import { useState } from "react";
import { wfmSocket } from "@models/wfmSocket";

export default function LoginPage() {
  // States
  // const navigate = useNavigate();
  const [interval, setInterval] = useState(0);
  const [progressText, setProgressText] = useState("")
  const [is_banned, setIsBanned] = useState(false);
  const [banned_reason, setBannedReason] = useState<string | undefined>(undefined)

  // Translate general
  const useTranslatePage = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`auth.${key}`, { ...context }, i18Key)
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`errors.${key}`, { ...context }, i18Key)
  const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`success.${key}`, { ...context }, i18Key)
  const useTranslateProgress = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`progress.${key}`, { ...context }, i18Key)

  // Mutations
  const logInMutation = useMutation({
    mutationFn: (data: { email: string; password: string }) => {
      setInterval(0);
      setProgressText(useTranslateProgress("logging_in"));
      return api.auth.login(data.email, data.password)
    },
    onSuccess: async (u) => {
      if (!u) return;
      setIsBanned(u.qf_banned);
      if (u.qf_banned)
        return notifications.show({ title: useTranslateErrors("login.title"), message: useTranslateErrors("login.banned"), color: "red.7" });

      setBannedReason(u.qf_banned_reason);
      notifications.show({ title: useTranslateSuccess("login.title"), message: useTranslateSuccess("login.message", { name: u.ingame_name }), color: "green.7" });
      setInterval(1);
      setProgressText(useTranslateProgress("refreshing_orders"));
      await api.order.refresh();

      setProgressText(useTranslateProgress("refreshing_auctions"));
      setInterval(2);
      await api.auction.refresh();

      setProgressText(useTranslateProgress("refreshing_chat"));
      setInterval(3);
      await api.chat.refresh();

      setProgressText(useTranslateProgress("refreshing_cache"));
      setInterval(4);
      await api.cache.reload();

      setProgressText(useTranslateProgress("refreshing_transaction"));
      setInterval(5);
      await api.transaction.reload();
      setProgressText(useTranslateProgress("refreshing_stock_items"));
      setInterval(6);
      await api.stock.item.reload();
      setProgressText(useTranslateProgress("refreshing_stock_riven"));
      setInterval(7);
      await api.stock.riven.reload();

      setProgressText(useTranslateProgress("login.progress_text_4"));
      setInterval(8);
      SendTauriDataEvent(QfSocketEvent.UpdateUser, QfSocketEventOperation.SET, u);
      if (u.wfm_access_token)
        wfmSocket.updateToken(u.wfm_access_token);
    },
    onError: (err: ResponseError) => {
      console.error(err);
      const { ApiError }: { ApiError: { messages: string[] } } = err.extra_data as any;
      if (ApiError.messages.some((m) => m.includes("app.account.email_not_exist")))
        return notifications.show({ title: useTranslateErrors("login.title"), message: useTranslateErrors("login.email_not_exist"), color: "red.7" });
      if (ApiError.messages.some((m) => m.includes("app.account.password_invalid")))
        return notifications.show({ title: useTranslateErrors("login.title"), message: useTranslateErrors("login.password_invalid"), color: "red.7" });

      return notifications.show({ title: useTranslateErrors("login.title"), message: useTranslateErrors("login.message"), color: "red.7" });
    }
  })

  return (
    <Center w={"100%"} h={"92vh"}>
      <LogInForm hide_submit={is_banned} is_loading={logInMutation.isPending} onSubmit={(d: { email: string; password: string }) => logInMutation.mutate(d)} footerContent={
        <>
          {logInMutation.isPending && (
            <Progress.Root size="xl">
              <Progress.Section value={interval / 8 * 100} >
                <Progress.Label>{progressText}</Progress.Label>
              </Progress.Section>
            </Progress.Root>
          )}
          {is_banned && (
            <TextTranslate i18nKey={useTranslateErrors("login.ban_reason")}
              values={{ reason: banned_reason || "" }}
            />
          )}
        </>}
      />
    </Center>
  );
}
