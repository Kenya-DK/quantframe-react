import { Avatar, Burger, Group, Header, Menu, createStyles, rem, Container, ActionIcon } from "@mantine/core";
import { useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faBars, faRightFromBracket, faUser } from "@fortawesome/free-solid-svg-icons";
import { useNavigate } from "react-router-dom";
import { useTranslateLayout } from "@hooks/index";
import { Wfm } from "$types/index";
import i18next from "i18next"
interface TopMenuProps {
  opened: boolean;
  user: Wfm.User | undefined;
  onOpenedClick: () => void;
  hideSidebar: boolean;
  setHideSidebar: (show: boolean) => void;
}

const HEADER_HEIGHT = rem(60);

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

export default function Hedder({ user, opened, onOpenedClick, setHideSidebar, hideSidebar }: TopMenuProps) {
  const { classes } = useStyles();
  const [, setUserMenuOpened] = useState(false);
  const useTranslateHedder = (key: string, context?: { [key: string]: any }) => useTranslateLayout(`header.${key}`, { ...context })
  const goTo = useNavigate();
  return (
    <Header height={HEADER_HEIGHT} sx={{ borderBottom: 0 }} mb={120}>
      <Container className={classes.inner} fluid>
        <Group>
          <Burger opened={opened} onClick={() => onOpenedClick()} className={classes.burger} size="sm" />
        </Group>
        <Group spacing={20}>
          {user && (
            <Menu
              width={"auto"}
              position="bottom-end"
              transitionProps={{ transition: 'pop-top-right' }}
              onClose={() => setUserMenuOpened(false)}
              onOpen={() => setUserMenuOpened(true)}
            >
              <Menu.Target>
                <ActionIcon color="pink" size="xs">
                  <Avatar variant="subtle" src={user?.avatar} alt={user.ingame_name} radius="xl" size={"sm"} />
                </ActionIcon>
              </Menu.Target>
              <Menu.Dropdown>
                <Menu.Item icon={<Avatar variant="subtle" src={user?.avatar} alt={user.ingame_name} radius="xl" size={"sm"} />}>
                  {user.ingame_name}
                </Menu.Item>
                <Menu.Divider />
                <Menu.Item icon={<FontAwesomeIcon icon={faUser} />} onClick={() => goTo("/users/me")}>
                  {i18next.t("profile.view_profile", { asds: "" })}
                </Menu.Item>
                <Menu.Divider />
                <Menu.Item icon={<FontAwesomeIcon icon={faBars} />} onClick={() => setHideSidebar(!hideSidebar)}>
                  {useTranslateHedder("profile.toggle_menu")}
                </Menu.Item>
                <Menu.Item icon={<FontAwesomeIcon icon={faRightFromBracket} />}>
                  {useTranslateHedder("profile.logout")}
                </Menu.Item>
              </Menu.Dropdown>
            </Menu>)}
        </Group>
      </Container>
    </Header>
  )
}