import { Avatar, Group, Header, Menu, createStyles, rem, Container, ActionIcon, useMantineTheme, Indicator } from "@mantine/core";
import { useEffect, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFolder, faRightFromBracket } from "@fortawesome/free-solid-svg-icons";
import { useTranslateLayout } from "@hooks/index";
import { SettingsModal } from "@components/modals/settings";
import { DeepPartial, Settings, Wfm } from "$types/index";
import { faGear } from "@fortawesome/free-solid-svg-icons/faGear";
import { modals } from "@mantine/modals";
import { Logo } from "../components/logo";
import Clock from "../components/clock";
import api, { wfmThumbnail } from "@api/index";
import { useAppContext, useCacheContext, useSocketContextContext } from "../contexts";
import { notifications } from "@mantine/notifications";
import { getUserStatusColor } from "../utils";
interface TopMenuProps {
  opened: boolean;
  user: Wfm.UserDto | undefined;
  onOpenedClick: () => void;
  hideSidebar: boolean;
  setHideSidebar: (show: boolean) => void;
}

const HEADER_HEIGHT = rem(50);

const useStyles = createStyles((theme) => ({
  inner: {
    height: HEADER_HEIGHT,
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  burger: {
    [theme.fn.largerThan('sm')]: {
      display: 'none',
    },
  }

}));

export default function HeaderC({ user }: TopMenuProps) {
  const theme = useMantineTheme();
  const { classes } = useStyles();
  const { socket } = useSocketContextContext();
  const [, setUserMenuOpened] = useState(false);
  const [avatar, setAvatar] = useState<string | undefined>(undefined);
  const { settings } = useAppContext();
  const { items } = useCacheContext();
  useEffect(() => {
    setAvatar(`${wfmThumbnail(user?.avatar || "")}`);
  }, [user?.avatar]);


  const useTranslateHeader = (key: string, context?: { [key: string]: any }) => useTranslateLayout(`header.${key}`, { ...context })

  const handleUpdateSettings = async (settingsData: DeepPartial<Settings>) => {
    if (!settingsData) return;
    const data = { ...settings, ...settingsData } as Settings;
    await api.base.updatesettings(data as any); // add 'as any' to avoid type checking
    notifications.show({
      title: useTranslateHeader("notifications.settings_updated"),
      message: useTranslateHeader("notifications.settings_updated_message"),
      color: 'green',
      autoClose: 5000,
    });
  }

  const SetUserStatus = async (status: Wfm.UserStatus) => {
    socket?.send(JSON.stringify({
      type: "@WS/USER/SET_STATUS",
      payload: status
    }));
  }

  return (
    <Header height={HEADER_HEIGHT} sx={{ borderBottom: 0 }} mb={120}>
      <Container className={classes.inner} fluid>
        <Group>
          <Group position="left" grow>
            <Logo color={theme.colors.blue[7]} />
          </Group>
        </Group>
        <Clock />
        <Group spacing={20}>
          <Menu
            width={"auto"}
            position="bottom-end"
            transitionProps={{ transition: 'pop-top-right' }}
            onClose={() => setUserMenuOpened(false)}
            onOpen={() => setUserMenuOpened(true)}
          >
            <Menu.Target>
              <ActionIcon color="pink" size="xs">
                <Indicator
                  withBorder
                  styles={{ indicator: { border: '0.2rem solid #1D1E30' } }}
                  disabled={!user}
                  inline
                  size={15}
                  offset={5}
                  position="bottom-start"
                  color={getUserStatusColor(user?.status || Wfm.UserStatus.Invisible)}
                >
                  <Avatar
                    variant="subtle"
                    src={avatar}
                    alt={user?.ingame_name}
                    radius="xl"
                    size="md"
                  />
                </Indicator>
              </ActionIcon>
            </Menu.Target>
            <Menu.Dropdown>
              <Menu.Item icon={<Avatar variant="subtle" src={avatar} alt={user?.ingame_name} radius="xl" size={"sm"} />}>
                {user?.ingame_name || "Unknown"}
              </Menu.Item>
              <Menu.Divider />
              <Menu.Item icon={<FontAwesomeIcon icon={faGear} />} onClick={async () => {
                modals.open({
                  size: "100%",
                  withCloseButton: false,
                  children: <SettingsModal settings={settings} updateSettings={handleUpdateSettings} tradable_items={items} />,
                })
              }}>
                {useTranslateHeader("profile.settings")}
              </Menu.Item>
              <Menu.Item icon={<FontAwesomeIcon icon={faFolder} />} onClick={async () => {
                await api.base.openLogsFolder();
              }}>
                {useTranslateHeader("profile.open_logs_folder")}
              </Menu.Item>
              <Menu.Item icon={<FontAwesomeIcon icon={faFolder} />} onClick={async () => {
                await api.base.export_logs();
              }}>
                {useTranslateHeader("profile.export_logs")}
              </Menu.Item>
              {user && (
                <Menu.Item icon={<FontAwesomeIcon icon={faRightFromBracket} />} onClick={async () => { await api.auth.logout(); }}>
                  {useTranslateHeader("profile.logout")}
                </Menu.Item>
              )}
              <Menu.Divider />

              {user && (<>
                <Menu.Label>{useTranslateHeader("profile.status.title")}</Menu.Label>
                <Menu.Item color="darkgreen" onClick={() => SetUserStatus(Wfm.UserStatus.Online)}>
                  {useTranslateHeader("profile.status.online")}
                </Menu.Item>
                <Menu.Item color="mediumpurple" onClick={() => SetUserStatus(Wfm.UserStatus.Ingame)}>
                  {useTranslateHeader("profile.status.ingame")}
                </Menu.Item>
                <Menu.Item color="gray.5" onClick={() => SetUserStatus(Wfm.UserStatus.Invisible)}>
                  {useTranslateHeader("profile.status.invisible")}
                </Menu.Item>
              </>)}
            </Menu.Dropdown>
          </Menu>
        </Group>
      </Container>
    </Header>
  )
}