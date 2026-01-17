import { Box, Text, Center, Group, Stack, CardProps, Card, Button } from "@mantine/core";
import { modals } from "@mantine/modals";
import { useGetPatreonInfo } from "@hooks/useGetPatreonInfo.hook";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { useAppContext } from "@contexts/app.context";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faBookOpen, faCoffee } from "@fortawesome/free-solid-svg-icons";
import { faDiscord } from "@fortawesome/free-brands-svg-icons";
import classes from "./About.module.css";
import { TextTranslate } from "@components/Shared/TextTranslate";
import { useState } from "react";

interface InfoCardProps {
  icon: React.ReactNode;
  title: string;
  link?: string;
  onClick?: () => void;
  cardProps?: CardProps;
}
const InfoCard = ({ icon, title, cardProps, link, onClick }: InfoCardProps) => {
  return (
    <Card
      {...cardProps}
      className={classes.card}
      onClick={() => {
        if (onClick) onClick();
        if (!link) return;
        open(link);
      }}
    >
      {icon}
      <Group justify="space-between">
        <Text size="xl">{title}</Text>
      </Group>
    </Card>
  );
};

export default function AboutPage() {
  const patreonInfo = useGetPatreonInfo();
  const { app_info, checkForUpdates } = useAppContext();
  // State
  const [isCheckingForUpdates, setIsCheckingForUpdates] = useState(false);
  // Translate general
  const useTranslatePage = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`about.${key}`, { ...context }, i18Key);
  const useTranslateCards = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePage(`cards.${key}`, { ...context }, i18Key);
  const useTranslateText = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePage(`text.${key}`, { ...context }, i18Key);

  return (
    <Box p={"md"}>
      <Center w={"100%"} h={"80vh"}>
        <Stack gap={"lg"} align={"center"}>
          <Group grow gap={"5vw"}>
            <InfoCard link="https://quantframe.app" icon={<FontAwesomeIcon size="5x" icon={faBookOpen} />} title={useTranslateCards("guide.title")} />
            <InfoCard
              link="https://www.buymeacoffee.com/kenyadk"
              icon={<FontAwesomeIcon size="5x" icon={faCoffee} style={{ color: "#ffa000" }} />}
              title={useTranslateCards("coffee.title")}
            />
            <InfoCard
              link="https://discord.gg/dPSFPG5Qt6"
              icon={<FontAwesomeIcon size="5x" icon={faDiscord} />}
              title={useTranslateCards("discord.title")}
            />
          </Group>
          <Group gap={"5vw"}>
            <InfoCard
              // cardProps={{ style: { width: "1000px" } }}
              onClick={() => {
                modals.openContextModal({
                  modalKey: "patreon",
                  withCloseButton: false,
                  size: "50vw",
                  innerProps: patreonInfo,
                });
              }}
              icon={
                <Text fw={700} fz={"3rem"} style={{ color: "#e07550" }}>
                  Patreon
                </Text>
              }
              title={useTranslateCards("patreon.title")}
            />
          </Group>
        </Stack>
      </Center>
      <Box>
        <TextTranslate
          size="lg"
          i18nKey={useTranslateText("version", undefined, true)}
          values={{ version: app_info?.version || "0.0.0", year: new Date().getFullYear() }}
        />
        <TextTranslate
          size="lg"
          i18nKey={useTranslateText("patreon_thanks", undefined, true)}
          values={{ users: patreonInfo.user_names.join(", ") }}
        />
        <Group justify="space-between">
          <TextTranslate size="lg" i18nKey={useTranslateText("disclaimer", undefined, true)} values={{}} />
          <Button
            loading={isCheckingForUpdates}
            onClick={async () => {
              if (app_info && checkForUpdates) {
                setIsCheckingForUpdates(true);
                await checkForUpdates(app_info, true, true);
                setIsCheckingForUpdates(false);
                // if (!rep) useTranslatePage("no_updates_available");
              }
            }}
          >
            {useTranslatePage("button_check_for_updates")}
          </Button>
        </Group>
      </Box>
    </Box>
  );
}
