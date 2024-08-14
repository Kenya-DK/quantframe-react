import { Box, Card, Center, Group, Text } from "@mantine/core";
import classes from './About.module.css';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faBookOpen, faQuestion } from "@fortawesome/free-solid-svg-icons";
import { faDiscord } from "@fortawesome/free-brands-svg-icons";
export default function AboutPage() {

  // Translate general
  // const useTranslatePage = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`about.${key}`, { ...context }, i18Key)

  return (
    <Center w={"100%"} h={"92vh"}>
      <Box>
        <Group grow gap={"5vw"}>
          <Card className={classes.card}>
            <Card.Section >
              <FontAwesomeIcon size="5x" icon={faBookOpen} />
            </Card.Section>
            <Group justify="space-between" mt="md" mb="xs">
              <Text size="xl">Wiki/Docs</Text>
            </Group>
          </Card>
          <Card className={classes.card}>
            <Card.Section >
              <FontAwesomeIcon size="5x" icon={faQuestion} />
            </Card.Section>
            <Group justify="space-between" mt="md" mb="xs">
              <Text size="xl">FAQ</Text>
            </Group>
          </Card>
          <Card className={classes.card}>
            <Card.Section >
              <FontAwesomeIcon size="5x" icon={faDiscord} />
            </Card.Section>
            <Group justify="space-between" mt="md" mb="xs">
              <Text size="xl">Discord</Text>
            </Group>
          </Card>
        </Group>
      </Box>
      <Box>

      </Box>
    </Center>
  );
}