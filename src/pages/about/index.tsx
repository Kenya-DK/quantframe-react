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
  titleColor?: string;
  borderColor?: string;
}
const InfoCard = ({ icon, title, cardProps, link, onClick, titleColor, borderColor }: InfoCardProps) => {
  return (
    <Card
      {...cardProps}
      className={classes.card}
      style={{ ...cardProps?.style, borderColor: borderColor }}
      onClick={() => {
        if (onClick) onClick();
        if (!link) return;
        open(link);
        api.analytics.sendMetric("open_web", link);
      }}
    >
      {icon}
      <Group justify="space-between">
        <Text size="xl" style={{ color: titleColor }}>{title}</Text>
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
        <Stack gap={"xl"} align={"center"} w={"100%"} maw={800}>
          <Group grow gap={"xl"} w={"100%"}>
            <InfoCard 
              link="https://quantframe.app" 
              icon={<FontAwesomeIcon size="5x" icon={faBookOpen} />} 
              title={useTranslateCards("guide.title")} 
              cardProps={{ h: 200, w: "100%" }}
            />
            <InfoCard
              link="https://quantframe.app/faq"
              icon={<FontAwesomeIcon size="5x" icon={faQuestion} />}
              title={useTranslateCards("faq.title")}
              cardProps={{ h: 200, w: "100%" }}
            />
            <InfoCard
              link="https://discord.gg/dPSFPG5Qt6"
              icon={<FontAwesomeIcon size="5x" icon={faDiscord} />}
              title={useTranslateCards("discord.title")}
              cardProps={{ h: 200, w: "100%" }}
            />
          </Group>
          <Group gap={"xl"} justify="center" w={"100%"}>
            <InfoCard
              link="https://www.buymeacoffee.com/kenyadk"
              icon={<FontAwesomeIcon size="5x" icon={faCoffee} style={{ color: "#ffffff" }} />}
              title={useTranslateCards("coffee.title")}
              cardProps={{ h: 200, w: "calc((100% - var(--mantine-spacing-xl)) / 3)", style: { backgroundColor: "#ffa000" } }}
              titleColor="#ffffff"
              borderColor="#ffb941ff"
            />
            <InfoCard
              onClick={() => {
                modals.open({
                  title: useTranslateModals("base.titles.premium"),
                  children: <PremiumModal link={patreon_link} />,
                  size: "50vw",
                  centered: true,
                });
              }}
              icon={
                <img 
                  src="/about/PATREON_SYMBOL_1_WHITE_RGB.svg" 
                  alt="Patreon" 
                  style={{ width: "80px", height: "80px" }}
                />
              }
              title={useTranslateCards("patreon.title")}
              cardProps={{ h: 200, w: "calc((100% - var(--mantine-spacing-xl)) / 3)", style: { backgroundColor: "#fc674d" } }}
              titleColor="#ffffff"
              borderColor="#ff9d8bff"
            />
          </Group>
        </Stack>
      </Center>
      <Box mt={"xl"}>
        <TextTranslate size="lg" i18nKey={useTranslateText("version", undefined, true)} values={{ version: app_info?.version || "0.0.0" }} />
        <TextTranslate size="lg" i18nKey={useTranslateText("disclaimer", undefined, true)} values={{}} />
      </Box>
    </Box>
  );
}
