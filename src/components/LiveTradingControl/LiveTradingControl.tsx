import { useLiveScraperContext } from "@contexts/liveScraper.context";
import { Button, Center, Group, Stack } from "@mantine/core";
import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import api from "@api/index";
import { TextTranslate } from "@components/TextTranslate";
import { useEffect, useState } from "react";
import { useTranslateComponent } from "@hooks/useTranslate.hook";

export function LiveTradingControl() {

  // States
  const { is_running, message } = useLiveScraperContext();
  const [showMessage, setShowMessage] = useState<boolean>(false);

  // Translate general
  const useTranslateLiveTrading = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`live_trading_control.${key}`, { ...context }, i18Key)
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateLiveTrading(`buttons.${key}`, { ...context }, i18Key)
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateLiveTrading(`errors.${key}`, { ...context }, i18Key)

  // Mutations
  const StartTradingMutation = useMutation({
    mutationFn: () => api.live_scraper.start(),
    onSuccess: async () => { },
    onError: () => notifications.show({ title: useTranslateErrors("login.title"), message: useTranslateErrors("login.message"), color: "red.7" })
  })

  useEffect(() => {
    if (message && message?.i18nKey != "" && message?.i18nKey != "idle")
      setShowMessage(true);
    else if ((message && message?.i18nKey.endsWith("idle")) || !is_running)
      setShowMessage(false);
  }, [message, is_running])

  return (
    <Center>
      <Stack gap={5} justify="center">
        <Group justify="center">
          <Button onClick={() => is_running ? api.live_scraper.stop() : StartTradingMutation.mutate()}>{useTranslateButtons(is_running ? "stop" : "start")}{ }</Button>
        </Group>
        {showMessage && (<TextTranslate i18nKey={useTranslateLiveTrading(message?.i18nKey || "", undefined, true)} values={message?.values || {}} />)}
      </Stack>
    </Center>
  );
}