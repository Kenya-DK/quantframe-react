import { Box, Card, CardProps, Center, Group, Stack, Text } from "@mantine/core";
import classes from "./About.module.css";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faBookOpen, faCoffee, faQuestion } from "@fortawesome/free-solid-svg-icons";
import { faDiscord } from "@fortawesome/free-brands-svg-icons";
import { useAppContext } from "@contexts/app.context";
import { useTranslateModals, useTranslatePages } from "@hooks/useTranslate.hook";
import { TextTranslate } from "@components/TextTranslate";
import { PremiumModal } from "@components/Modals/Premium";
import api from "@api/index";
import { open } from "@tauri-apps/plugin-shell";
import React from "react";
import { modals } from "@mantine/modals";
import { useAuthContext } from "../../contexts/auth.context";
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
        api.analytics.sendMetric("open_web", link);
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
  // Contexts
  const { patreon_link } = useAuthContext();
  const { app_info } = useAppContext();
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
              link="https://quantframe.app/faq"
              icon={<FontAwesomeIcon size="5x" icon={faQuestion} />}
              title={useTranslateCards("faq.title")}
            />
            <InfoCard
              link="https://discord.gg/dPSFPG5Qt6"
              icon={<FontAwesomeIcon size="5x" icon={faDiscord} />}
              title={useTranslateCards("discord.title")}
            />
          </Group>
          <Group gap={"5vw"}>
            <InfoCard
              link="https://www.buymeacoffee.com/kenyadk"
              icon={<FontAwesomeIcon size="3x" icon={faCoffee} style={{ color: "#ffa000" }} />}
              title={useTranslateCards("coffee.title")}
            />
            <InfoCard
              // cardProps={{ style: { width: "1000px" } }}
              onClick={() => {
                modals.open({
                  title: useTranslateModals("base.titles.premium"),
                  children: <PremiumModal link={patreon_link} />,
                  size: "50vw",
                  centered: true,
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
        <TextTranslate size="lg" i18nKey={useTranslateText("version", undefined, true)} values={{ version: app_info?.version || "0.0.0" }} />
        <TextTranslate size="lg" i18nKey={useTranslateText("disclaimer", undefined, true)} values={{}} />
      </Box>
    </Box>
  );
}
