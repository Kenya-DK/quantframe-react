import { Button, Text, Container, Group, Paper, Stack, Title } from "@mantine/core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowUpRightFromSquare } from "@fortawesome/free-solid-svg-icons";
import { open } from "@tauri-apps/plugin-shell";
import { useTranslateModals } from "@hooks/useTranslate.hook";
export type PremiumModalProps = {
  link?: string;
};
export function PremiumModal({ link }: PremiumModalProps) {
  // Translate general
  const useTranslateTOS = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`premium_modal.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTOS(`buttons.${key}`, { ...context }, i18Key);
  return (
    <Container fluid>
      <Stack align={"center"} justify={"center"} w={"100%"}>
        <Title order={2}>{useTranslateTOS("title")}</Title>
        <Paper w={"75%"} bg={"gray.8"} style={{}} withBorder p={"lg"}>
          <Group grow>
            <Text>{useTranslateTOS("text")}</Text>
            <Button
              onClick={() => {
                open("https://www.patreon.com/Quantframe");
              }}
              rightSection={<FontAwesomeIcon icon={faArrowUpRightFromSquare} />}
            >
              {useTranslateButtons("info")}
            </Button>
          </Group>
          <Group mt={"lg"} grow>
            <Text>{useTranslateTOS("text2")}</Text>
            <Button
              onClick={() => {
                if (!link) return;
                open(link);
              }}
              rightSection={<FontAwesomeIcon icon={faArrowUpRightFromSquare} />}
            >
              {useTranslateButtons("link")}
              <pre>
                <code>{link}</code>
              </pre>
            </Button>
          </Group>
        </Paper>
      </Stack>
    </Container>
  );
}
