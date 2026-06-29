import { TauriTypes, UserStatus } from "$types";
import api, { SendTauriDataEvent, SendTauriEvent, WFMThumbnail } from "@api/index";
import { SettingsForm } from "@components/Forms/Settings";
import { useAppContext } from "@contexts/app.context";
import { useAuthContext } from "@contexts/auth.context";
import { faGear, faRightFromBracket } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useTranslateCommon, useTranslateComponent, useTranslateEnums } from "@hooks/useTranslate.hook";
import { Avatar, Button, Group, Indicator, Menu, Text } from "@mantine/core";
import { modals } from "@mantine/modals";
import { notifications } from "@mantine/notifications";
import { useMutation } from "@tanstack/react-query";
import classes from "./UserMenu.module.css";

export function UserMenu() {
  // States
  const { user } = useAuthContext();
  const { settings, app_error, setLang } = useAppContext();

  // Translate general
  const useTranslateUserMenu = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`user_menu.${key}`, { ...context }, i18Key);
  const useTranslateUserStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`user_status.${key}`, { ...context }, i18Key);
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateUserMenu(`errors.${key}`, { ...context }, i18Key);
  const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateUserMenu(`success.${key}`, { ...context }, i18Key);
  const useTranslatePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateUserMenu(`prompts.${key}`, { ...context }, i18Key);

  // Mutations
  const logOutMutation = useMutation({
    mutationFn: () => api.auth.logout(),
    onSuccess: (u) => {
      notifications.show({ title: useTranslateSuccess("logout.title"), message: useTranslateSuccess("logout.message"), color: "green.7" });
      SendTauriDataEvent(TauriTypes.Events.UpdateUser, TauriTypes.EventOperations.SET, u);
    },
    onError: () => notifications.show({ title: useTranslateErrors("logout.title"), message: useTranslateErrors("logout.message"), color: "red.7" }),
  });
  const updateSettingsMutation = useMutation({
    mutationFn: (s: TauriTypes.Settings) => api.app.updateSettings(s),
    onSuccess: () => {
      notifications.show({
        title: useTranslateCommon("notifications.update_settings.success.title"),
        message: useTranslateCommon("notifications.update_settings.success.message"),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({
        title: useTranslateCommon("notifications.update_settings.error.title"),
        message: useTranslateCommon("notifications.update_settings.error.message", e),
        color: "red.7",
      });
    },
  });

  const IsConnected = () => {
    if (!user) return false;
    if (user.anonymous) return false;
    if (user.qf_banned || user.wfm_banned) return false;
    if (app_error && app_error.hasOperation("Main:Disconnected")) return false;
    // return !!user && !user.anonymous && !app_error;
    return true;
  };

  const IsAuthenticated = () => {
    if (!user) return false;
    if (user.anonymous) return false;
    if (user.qf_banned || user.wfm_banned) return false;
    if (!user.verification) return false;
    return true;
  };

  return (
    <Menu shadow="md" width={200} transitionProps={{ transition: "fade-down", duration: 150 }} position="bottom-end" offset={5}>
      <Menu.Target>
        <Group>
          <Indicator
            inline
            hidden
            size={16}
            offset={7}
            position="bottom-start"
            withBorder
            classNames={classes}
            disabled={!IsConnected()}
            data-color-mode="text"
            data-user-status={user?.wfm_status || UserStatus.Invisible}
          >
            <Avatar
              data-error={!IsConnected()}
              className={classes.avatar}
              variant="subtle"
              src={user?.wfm_avatar && user?.wfm_avatar != "" ? WFMThumbnail(user?.wfm_avatar) : "/default_avatar.png"}
              alt={user?.wfm_username}
              radius="xl"
              size="48px"
            />
          </Indicator>
        </Group>
      </Menu.Target>

      <Menu.Dropdown>
        <Menu.Item
          leftSection={
            <Avatar
              variant="subtle"
              src={user?.wfm_avatar && user?.wfm_avatar != "" ? WFMThumbnail(user?.wfm_avatar) : "/default_avatar.png"}
              alt={user?.wfm_username}
              radius="xl"
              size={"md"}
            />
          }
        >
          {user?.wfm_username || "Unknown"}
        </Menu.Item>
        <Menu.Divider />
        {!IsConnected() && (
          <Group mt="xs" grow>
            <Text ta={"center"} size="lg" c="red.7" fw={700}>
              {useTranslateErrors("connection_websocket_error")}
            </Text>
          </Group>
        )}
        {IsConnected() && (
          <Group gap={3} mt="xs" classNames={{ root: classes.user_status }}>
            {Object.values(UserStatus).map((status) => (
              <Button
                key={status}
                disabled={user?.wfm_status == status}
                p={3}
                fullWidth
                variant="subtle"
                data-color-mode="text"
                data-user-status={status}
                data-active={status == user?.wfm_status}
                onClick={() => api.user.set_status(status)}
              >
                <Text tt="uppercase" fw={500}>
                  {useTranslateUserStatus(status)}
                </Text>
              </Button>
            ))}
          </Group>
        )}
        <Menu.Divider />
        <Menu.Label>{useTranslateUserMenu("items.app_label")}</Menu.Label>
        <Menu.Item
          leftSection={<FontAwesomeIcon icon={faGear} />}
          onClick={() => {
            if (!settings) return;
            modals.open({
              size: "100%",
              withCloseButton: false,
              children: (
                <SettingsForm
                  value={settings}
                  onSubmit={async (s) => {
                    if (s.lang != settings.lang && setLang) setLang(s.lang);
                    await updateSettingsMutation.mutateAsync(s);
                    SendTauriEvent(TauriTypes.Events.RefreshSettings);
                    modals.closeAll();
                  }}
                />
              ),
            });
          }}
        >
          {useTranslateUserMenu("items.settings")}
        </Menu.Item>
        <Menu.Item
          disabled={!IsAuthenticated()}
          leftSection={<FontAwesomeIcon icon={faRightFromBracket} />}
          onClick={() => {
            if (!IsAuthenticated()) return;
            modals.openConfirmModal({
              title: useTranslatePrompt("logout.title"),
              children: <Text size="sm">{useTranslatePrompt("logout.message")}</Text>,
              labels: { confirm: useTranslatePrompt("logout.confirm"), cancel: useTranslatePrompt("logout.cancel") },
              onConfirm: async () => logOutMutation.mutateAsync(),
            });
          }}
        >
          {useTranslateUserMenu("items.logout")}
        </Menu.Item>
      </Menu.Dropdown>
    </Menu>
  );
}
