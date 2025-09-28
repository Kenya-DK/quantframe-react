import { Button, Card, Center, Group, Overlay, Text } from "@mantine/core";
import classes from "./PatreonOverlay.module.css";

export type PatreonOverlayProps = {};

export function PatreonOverlay({}: PatreonOverlayProps) {
  return (
    <Overlay color="#5a5858ff" backgroundOpacity={0.8} blur={6}>
      <Center style={{ height: "100%" }}>
        <Card shadow="xl" radius="md" padding="lg" withBorder className={classes.patreonCard}>
          <Group gap="xs" mb="sm">
            <Text fw={700} size="lg">
              Restricted Content
            </Text>
          </Group>

          <Text size="sm" c="dimmed" mb="md">
            You need to be a <b>Patreon T1+</b> to access this content.
          </Text>

          <Button fullWidth size="md" radius="sm" component="a" href="https://patreon.com/yourpage" target="_blank" className={classes.patreonButton}>
            Become a Patron
          </Button>
        </Card>
      </Center>
    </Overlay>
  );
}
