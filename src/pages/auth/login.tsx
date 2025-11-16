import { Center, Progress } from "@mantine/core";
import { LogInForm } from "@components/Forms/LogIn";
import api, { SendTauriDataEvent } from "@api/index";
import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { TextTranslate } from "@components/Shared/TextTranslate";
import { ResponseError, TauriTypes } from "$types";
import { useState } from "react";

export default function LoginPage() {
  // States
  // const navigate = useNavigate();
  const [interval, setInterval] = useState(0);
  const [progressText, setProgressText] = useState("");
  const [is_banned, setIsBanned] = useState(false);
  const [banned_reason, setBannedReason] = useState<string | undefined>(undefined);

  // Translate general
  const useTranslatePage = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`auth.${key}`, { ...context }, i18Key);
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePage(`errors.${key}`, { ...context }, i18Key);
  const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePage(`success.${key}`, { ...context }, i18Key);
  const useTranslateProgress = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePage(`progress.${key}`, { ...context }, i18Key);

  // Mutations
  const logInMutation = useMutation({
    mutationFn: (data: { email: string; password: string }) => {
      setInterval(0);
      setProgressText(useTranslateProgress("logging_in"));
      return api.auth.login(data.email, data.password);
    },
    onSuccess: async (u) => {
      if (!u) return;
      setIsBanned(u.qf_banned);
      if (u.qf_banned)
        return notifications.show({ title: useTranslateErrors("login.title"), message: useTranslateErrors("login.banned"), color: "red.7" });
      if (!u.verification)
        return notifications.show({ title: useTranslateErrors("login.title"), message: useTranslateErrors("login.verification"), color: "red.7" });
      setBannedReason(u.qf_banned_reason);
      notifications.show({
        title: useTranslateSuccess("login.title"),
        message: useTranslateSuccess("login.message", { name: u.wfm_username }),
        color: "green.7",
      });
      SendTauriDataEvent(TauriTypes.Events.UpdateUser, TauriTypes.EventOperations.SET, u);
    },
    onError: (err: ResponseError) => {
      console.error(err);
      const { type } = err.context as any;
      return notifications.show({
        title: useTranslateErrors("login.title"),
        message: useTranslateErrors(`login.${type}`),
        color: "red.7",
      });
    },
  });

  return (
    <Center w={"100%"} h={"92vh"}>
      <LogInForm
        hide_submit={is_banned}
        is_loading={logInMutation.isPending}
        onSubmit={(d: { email: string; password: string }) => logInMutation.mutate(d)}
        footerContent={
          <>
            {logInMutation.isPending && (
              <Progress.Root size="xl">
                <Progress.Section value={(interval / 6) * 100}>
                  <Progress.Label>{progressText}</Progress.Label>
                </Progress.Section>
              </Progress.Root>
            )}
            {is_banned && <TextTranslate i18nKey={useTranslateErrors("login.ban_reason")} values={{ reason: banned_reason || "" }} />}
          </>
        }
      />
    </Center>
  );
}
