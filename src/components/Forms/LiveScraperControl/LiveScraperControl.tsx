import { useLiveScraperContext } from "@contexts/liveScraper.context";
import { Button, Center, Group, Stack, Text } from "@mantine/core";
import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import api from "@api/index";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { useAppContext } from "@contexts/app.context";
import { modals } from "@mantine/modals";
import { useEffect, useState } from "react";
import { TextTranslate } from "../../Shared/TextTranslate";

export function LiveScraperControl() {
  // States
  const { is_running, message } = useLiveScraperContext();
  const { settings } = useAppContext();

  // State
  const [showMessage, setShowMessage] = useState(false);

  // Translate general
  const useTranslateLiveScraper = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`live_scraper_control.${key}`, { ...context }, i18Key);
  const useTranslatePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateLiveScraper(`prompts.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateLiveScraper(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateLiveScraper(`errors.${key}`, { ...context }, i18Key);

  // Mutations
  const StartTradingMutation = useMutation({
    mutationFn: () => api.live_scraper.toggle(),
    onSuccess: async () => {},
    onError: () => notifications.show({ title: useTranslateErrors("login.title"), message: useTranslateErrors("login.message"), color: "red.7" }),
  });

  useEffect(() => {
    setShowMessage(!!message && message.i18nKey !== "");
  }, [message]);

  const ToggleLiveTrading = (start: boolean) => {
    if (start) {
      if (settings?.live_scraper.auto_delete)
        modals.openConfirmModal({
          title: useTranslatePrompt("start.title"),
          children: <Text size="sm">{useTranslatePrompt("start.message")}</Text>,
          labels: { confirm: useTranslatePrompt("start.confirm"), cancel: useTranslatePrompt("start.cancel") },
          onConfirm: async () => StartTradingMutation.mutate(),
        });
      else StartTradingMutation.mutate();
    } else {
      api.live_scraper.toggle();
    }
  };

  return (
    <Center>
      <Stack gap={5} justify="center">
        <Group justify="center">
          <Button loading={StartTradingMutation.isPending} onClick={() => ToggleLiveTrading(!is_running)}>
            {useTranslateButtons(is_running ? "stop" : "start")}
          </Button>
        </Group>
        {is_running && showMessage && (
          <TextTranslate i18nKey={useTranslateLiveScraper(message?.i18nKey || "", undefined, true)} values={message?.values || {}} />
        )}
      </Stack>
    </Center>
  );
}
