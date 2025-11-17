import { Button, Card, Center, Group, Overlay, Text } from "@mantine/core";
import classes from "./PatreonOverlay.module.css";
import { HasPermission } from "@api/index";
import { TauriTypes } from "$types";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { TextTranslate } from "../TextTranslate";
import { modals } from "@mantine/modals";
import { useGetPatreonInfo } from "@hooks/useGetPatreonInfo.hook";
import { useEffect, useState } from "react";
export type PatreonOverlayProps = {
  permission: TauriTypes.PermissionsFlags;
  tier?: string;
};

export function PatreonOverlay({ permission, tier }: PatreonOverlayProps) {
  const patreonInfo = useGetPatreonInfo();
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`patreon_overlay.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);
  const [hasPermission, setHasPermission] = useState<boolean>(false);
  // Check permissions for export on mount
  useEffect(() => {
    HasPermission(permission).then((res) => setHasPermission(res));
  }, [permission]);
  return (
    <Overlay hidden={hasPermission} backgroundOpacity={0.5} blur={3}>
      <Center style={{ height: "100%" }}>
        <Card shadow="xl" radius="md" padding="lg" withBorder className={classes.patreonCard}>
          <Group gap="xs" mb="sm">
            <Text fw={700} size="lg">
              {useTranslateForm("title")}
            </Text>
          </Group>

          <TextTranslate
            textProps={{ size: "sm", c: "dimmed", mb: "md" }}
            i18nKey={useTranslateForm("description", undefined, true)}
            values={{ tier: tier || "T1+" }}
          />
          <Text size="sm" c="dimmed" mb="md">
            You need to be a <b>Patreon {tier || "T1+"}</b> to access this content.
          </Text>

          <Button
            fullWidth
            size="md"
            radius="sm"
            onClick={() => {
              modals.openContextModal({
                modalKey: "patreon",
                withCloseButton: false,
                size: "50vw",
                innerProps: patreonInfo,
              });
            }}
            className={classes.patreonButton}
          >
            {useTranslateButtons("become_a_patron")}
          </Button>
        </Card>
      </Center>
    </Overlay>
  );
}
