import { Box, Card, CardProps, Center, Group, Stack, Text } from "@mantine/core";
import classes from './About.module.css';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faBookOpen, faCoffee, faQuestion } from "@fortawesome/free-solid-svg-icons";
import { faDiscord } from "@fortawesome/free-brands-svg-icons";
import { useAppContext } from "@contexts/app.context";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { TextTranslate } from "../../components/TextTranslate";
import api from "@api/index";

interface InfoCardProps {
  icon: any;
  title: string;
  link: string;
  ga_id: string;
  cardProps?: CardProps;
  iconProps?: any;
}
const InfoCard = ({ icon, title, cardProps, iconProps, link, ga_id }: InfoCardProps) => {
  return (
    <Card {...cardProps} className={classes.card} onClick={() => {
      window.open(link, "_blank")
      api.analytics.sendMetric(ga_id, "");
    }}>
      <FontAwesomeIcon size="5x" {...iconProps} icon={icon} className={classes.icon} />
      <Group justify="space-between">
        <Text size="xl">{title}</Text>
      </Group>
    </Card>
  )
}

export default function AboutPage() {
  // Contexts
  const { app_info } = useAppContext();
  // Translate general
  const useTranslatePage = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`about.${key}`, { ...context }, i18Key)
  const useTranslateCards = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`cards.${key}`, { ...context }, i18Key)
  const useTranslateText = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`text.${key}`, { ...context }, i18Key)


  return (
    <Box p={"md"}>
      <Center w={"100%"} h={"80vh"}>
        <Stack gap={"lg"} align={"center"}>
          <Group grow gap={"5vw"}>
            <InfoCard ga_id="Website_WikiOpened" link="https://quantframe.app" icon={faBookOpen} title={useTranslateCards("guide.title")} />
            <InfoCard ga_id="Website_QuestionOpened" link="https://quantframe.app/faq" icon={faQuestion} title={useTranslateCards("faq.title")} />
            <InfoCard ga_id="Website_DiscordOpened" link="https://discord.gg/dPSFPG5Qt6" icon={faDiscord} title={useTranslateCards("discord.title")} />
          </Group>
          <InfoCard ga_id="BuyMeACoffee_WebOpened" link="https://www.buymeacoffee.com/kenyadk" icon={faCoffee} iconProps={{ size: "3x", color: "#ffa000" }} title={useTranslateCards("coffee.title")} />
        </Stack>
      </Center>
      <Box >
        <TextTranslate size="lg" i18nKey={useTranslateText("version", undefined, true)} values={{ version: app_info?.version || "0.0.0" }} />
        <TextTranslate size="lg" i18nKey={useTranslateText("disclaimer", undefined, true)} values={{}} />
      </Box>
    </Box>
  );
}