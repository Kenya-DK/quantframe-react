import { Center } from "@mantine/core";
import { LogInForm } from "@components";
import api, { SendTauriDataEvent } from "@api/index";
import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { useTranslatePages } from "@hooks/index";
import { useNavigate } from "react-router-dom";
import { QfSocketEvent, QfSocketEventOperation } from "@api/types";

export default function LoginPage() {
  // States
  const navigate = useNavigate();


  // Translate general
  const useTranslatePage = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`auth.${key}`, { ...context }, i18Key)
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`errors.${key}`, { ...context }, i18Key)
  const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`success.${key}`, { ...context }, i18Key)

  // Mutations
  const logInMutation = useMutation({
    mutationFn: (data: { email: string; password: string }) => api.auth.login(data.email, data.password),
    onSuccess: async (u) => {
      notifications.show({ title: useTranslateSuccess("login.title"), message: useTranslateSuccess("login.message", { name: u.ingame_name }), color: "green.7" });
      await api.order.refresh();
      await api.auction.refresh();
      await api.chat.refresh();
      SendTauriDataEvent(QfSocketEvent.UpdateUser, QfSocketEventOperation.SET, u);
      navigate('/')
    },
    onError: () => notifications.show({ title: useTranslateErrors("login.title"), message: useTranslateErrors("login.message"), color: "red.7" })
  })

  return (
    <Center w={"100%"} h={"92vh"}>
      <LogInForm onSubmit={async (d: any) => await logInMutation.mutateAsync(d)} />
    </Center>
  );
}
