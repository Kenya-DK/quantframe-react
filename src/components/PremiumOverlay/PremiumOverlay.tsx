import { Button, Center, Group, LoadingOverlay, Stack, Title } from "@mantine/core";
import classes from "./PremiumOverlay.module.css";
import { useTranslateComponent, useTranslateModals } from "@hooks/useTranslate.hook";
import { TextTranslate } from "../TextTranslate";
import { open } from "@tauri-apps/plugin-shell";
import { modals } from "@mantine/modals";
import { PremiumModal } from "../Modals/Premium";
export type PremiumOverlayProps = {
  tier: string;
};
export function PremiumOverlay({ tier }: PremiumOverlayProps) {
  // Translate general
  const useTranslateSearchField = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`premium_overlay.${key}`, { ...context }, i18Key);
  const useTranslateSearchFieldButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateSearchField(`buttons.${key}`, { ...context }, i18Key);
  return (
    <LoadingOverlay
      visible
      overlayProps={{ radius: "sm", blur: 2 }}
      loaderProps={{
        children: (
          <Center classNames={classes}>
            <Stack align="center" mb={10}>
              <Title order={3} c={"white"} mb={10}>
                <TextTranslate
                  color="white"
                  textProps={{
                    fw: 700,
                    fz: "h2",
                  }}
                  i18nKey={useTranslateSearchField("title", undefined, true)}
                  values={{ tier }}
                />
              </Title>
              <Group mb={10}>
                <Button
                  onClick={() => {
                    open("https://www.patreon.com/Quantframe");
                  }}
                >
                  {useTranslateSearchFieldButtons("info")}
                </Button>
                <Button
                  onClick={() => {
                    modals.open({
                      title: useTranslateModals("base.titles.premium"),
                      children: <PremiumModal />,
                      size: "50vw",
                      centered: true,
                    });
                  }}
                >
                  {useTranslateSearchFieldButtons("subscribe")}
                </Button>
              </Group>
            </Stack>
          </Center>
        ),
      }}
    ></LoadingOverlay>
  );
}
