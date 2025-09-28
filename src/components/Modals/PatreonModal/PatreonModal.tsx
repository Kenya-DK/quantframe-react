import { Button, Text, Container, Group, Stack, Title, Badge, Card, Divider, ThemeIcon } from "@mantine/core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowUpRightFromSquare, faHeart, faCrown, faRocket, faStar } from "@fortawesome/free-solid-svg-icons";
import { open } from "@tauri-apps/plugin-shell";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import classes from "./PatreonModal.module.css";
import { TauriTypes } from "$types";
import { ContextModalProps } from "@mantine/modals";

// Patreon Variables
const PatreonCreatorLink = "https://www.patreon.com/Quantframe";
const PatreonRedirectUriProd = "https://api.quantframe.app/auth/patreon/link";
const PatreonRedirectUriDev = "http://localhost:6969/auth/patreon/link";
const PatreonScope = ["identity", "identity[email]"];
const PatreonClientId = "6uDrK7uhMBAidiAvzQd7ukmHFz4NUXO1wocruae24C4_04rXrUMSvCzC9RKbQpmN"; // Replace with your actual Patreon Client ID
const PatreonOauthUrl = "https://www.patreon.com/oauth2/authorize?response_type=code";

export type PatreonModalProps = {
  is_dev: boolean;
  user: TauriTypes.User;
};

export function PatreonModal({ innerProps: { user, is_dev } }: ContextModalProps<PatreonModalProps>) {
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`patreon_modal.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`buttons.${key}`, { ...context }, i18Key);

  const OpenPatreonLink = () => {
    open(
      `${PatreonOauthUrl}&client_id=${PatreonClientId}&redirect_uri=${
        is_dev ? PatreonRedirectUriDev : PatreonRedirectUriProd
      }&scope=${PatreonScope.join("%20")}&state=${user.wfm_id}|${user.check_code}`
    );
  };

  return (
    <Container fluid>
      <Stack align="center" justify="center" gap="xl" className={classes.contentStack}>
        {/* Header Section */}
        <Group gap="md" className={classes.headerGroup}>
          <ThemeIcon size={60} radius="xl" className={classes.patreonIcon} variant="gradient" gradient={{ from: "#ff424d", to: "#ff7b54", deg: 45 }}>
            <FontAwesomeIcon icon={faHeart} size="lg" />
          </ThemeIcon>
          <Stack gap={4} align="center">
            <Title order={1} className={classes.mainTitle}>
              {useTranslate("title")}
            </Title>
            <Badge variant="gradient" gradient={{ from: "#ff424d", to: "#ff7b54", deg: 45 }} size="lg" className={classes.premiumBadge}>
              <FontAwesomeIcon icon={faCrown} style={{ marginRight: 8 }} />
              {useTranslate("premium_content")}
            </Badge>
          </Stack>
        </Group>

        {/* Description Card */}
        <Card shadow="xl" radius="lg" padding="xl" className={classes.descriptionCard} withBorder>
          <Stack gap="md" align="center">
            <Text size="lg" ta="center" className={classes.description}>
              {useTranslate("description")}
            </Text>

            <Divider w="60%" className={classes.gradientDivider} />

            <Group gap="xs" justify="center">
              <ThemeIcon size="sm" color="orange" variant="light">
                <FontAwesomeIcon icon={faStar} />
              </ThemeIcon>
              <Text size="sm" c="dimmed">
                {useTranslate("join_the_community")}
              </Text>
              <ThemeIcon size="sm" color="orange" variant="light">
                <FontAwesomeIcon icon={faStar} />
              </ThemeIcon>
            </Group>
          </Stack>
        </Card>

        {/* Action Buttons */}
        <Stack gap="md" w="100%">
          <Group justify="center" gap="lg">
            <Button
              size="lg"
              className={classes.primaryButton}
              onClick={() => open(PatreonCreatorLink)}
              leftSection={<FontAwesomeIcon icon={faRocket} />}
              rightSection={<FontAwesomeIcon icon={faArrowUpRightFromSquare} />}
            >
              {useTranslateButtons("info")}
            </Button>
          </Group>

          <Group justify="center" gap="md">
            <Text size="sm" c="dimmed">
              {useTranslate("link_your_account")}
            </Text>
            <Button
              variant="subtle"
              size="sm"
              onClick={() => OpenPatreonLink()}
              rightSection={<FontAwesomeIcon icon={faArrowUpRightFromSquare} />}
              className={classes.secondaryButton}
            >
              {useTranslateButtons("link")}
            </Button>
          </Group>
        </Stack>
      </Stack>
    </Container>
  );
}
