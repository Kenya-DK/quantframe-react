import { Button, Center, Group, LoadingOverlay, Stack, Title } from "@mantine/core";
import classes from "./PremiumOverlay.module.css";
import { useTranslateComponent, useTranslateModals } from "@hooks/useTranslate.hook";
import { TextTranslate } from "../TextTranslate";
import { open } from "@tauri-apps/plugin-shell";
import { modals } from "@mantine/modals";
import { PremiumModal } from "../Modals/Premium";
import { HasPermission, PermissionsFlags } from "@utils/permissions";
import { useGetUser } from "@hooks/useGetUser.hook";
import { useEffect } from "react";
import { useAuthContext } from "../../contexts/auth.context";

export type PremiumOverlayProps = {
  tier: string;
  link?: string;
  permission?: PermissionsFlags;
};

export function PremiumOverlay({ permission, link, tier }: PremiumOverlayProps) {
  const user = useGetUser();
  const { patreon_link } = useAuthContext();

  const useTranslateSearchField = (key: string, ctx?: Record<string, any>, raw?: boolean) =>
    useTranslateComponent(`premium_overlay.${key}`, ctx, raw);
  const tButton = (key: string, ctx?: Record<string, any>, raw?: boolean) => useTranslateSearchField(`buttons.${key}`, ctx, raw);

  const shouldShowOverlay = permission ? !HasPermission(user?.permissions, permission) : true;

  useEffect(() => {
    if (!user) return;
  }, [user]);

  return (
    <LoadingOverlay
      visible={shouldShowOverlay}
      zIndex={10}
      overlayProps={{ radius: "sm", blur: 2 }}
      loaderProps={{
        children: (
          <Center className={classes.root}>
            <Stack align="center" mb="md">
              <Title order={3} className={classes.titleGlow} mb="sm">
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

              <Group mb="sm">
                {link && <Button onClick={() => open(link)}>{tButton("info")}</Button>}
                <Button
                  onClick={() =>
                    modals.open({
                      title: useTranslateModals("base.titles.premium"),
                      children: <PremiumModal link={patreon_link} />,
                      size: "50vw",
                      centered: true,
                    })
                  }
                >
                  {tButton("subscribe")}
                </Button>
              </Group>
            </Stack>
          </Center>
        ),
      }}
    />
  );
}
