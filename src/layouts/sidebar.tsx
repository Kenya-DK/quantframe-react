import { createStyles, rem, Tooltip, UnstyledButton, Navbar, Stack, Indicator } from "@mantine/core";
import { ReactNode, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCoffee, faDesktop, faEnvelope, faGlobe, faHome } from "@fortawesome/free-solid-svg-icons";
import { useTranslateLayout } from "../hooks";
import { useNavigate } from "react-router-dom";
import { WFMLogo } from "../components/icons/wfm_logo";
import { useChatContext } from "../contexts";

interface NavbarLinkProps {
  icon: ReactNode;
  label: string;
  link: string;
  active?: boolean;
  button?: boolean;
  onClick?(url: string): void;
}
function NavbarLink({ link, icon, label, active, onClick, button }: NavbarLinkProps) {
  const { classes, cx } = useStyles();
  return (
    <Tooltip label={label} position="right" transitionProps={{ duration: 0 }}>
      <UnstyledButton sx={{ ...(button ? { position: "absolute", bottom: 0 } : undefined) }} onClick={() => onClick && onClick(link)} className={cx(classes.link, { [classes.active]: active })}>
        {icon}
      </UnstyledButton>
    </Tooltip>
  );
}

const useStyles = createStyles((theme) => ({
  link: {
    width: rem(50),
    height: rem(50),
    borderRadius: theme.radius.md,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.colors.gray[7],
    marginBottom: theme.spacing.xs,
    '&:hover': {
      backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[5] : theme.colors.gray[0],
    },
  },

  active: {
    '&, &:hover': {
      backgroundColor: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).background,
      color: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).color,
    },
  },
}));

export default function SideBar({ }) {
  const { unread_messages } = useChatContext();
  const goTo = useNavigate();
  const [active, setActive] = useState(0);
  const useTranslate = (key: string, context?: { [key: string]: any }) => useTranslateLayout(`navigation.${key}`, { ...context })
  const mockdata = [
    { link: "/", icon: <FontAwesomeIcon icon={faHome} />, label: useTranslate("home") },
    { link: "live-trading", icon: <FontAwesomeIcon icon={faGlobe} />, label: useTranslate("live_trading") },
    {
      link: "chats", icon: <Indicator disabled={unread_messages == 0} label={unread_messages > 0 ? unread_messages : undefined} inline size={16} position="top-start" color={status} >
        <FontAwesomeIcon icon={faEnvelope} />
      </Indicator>, label: useTranslate("chats")
    },
    // { link: "statistics", icon: <FontAwesomeIcon icon={faChartSimple} />, label: useTranslate("statistics") },
    { link: "warframe-market", icon: <WFMLogo color="#d5d7e0" />, label: useTranslate("warframe_market") },
    { link: "debug", icon: <FontAwesomeIcon icon={faDesktop} />, label: useTranslate("debug") },
  ];
  const links = mockdata.map((link, index) => (
    <NavbarLink
      {...link}
      key={link.label}
      active={index === active}
      onClick={(url) => {
        goTo(url);
        setActive(index)
      }}
    />
  ));
  return (
    <Navbar width={{ base: 70 }} p="xs">
      <Navbar.Section grow>
        <Stack spacing={0} sx={{ position: "relative", height: "100%" }}>
          {links}
          <NavbarLink
            button
            link={""}
            icon={<FontAwesomeIcon icon={faCoffee} />}
            label={useTranslate("buy_me_a_coffee")}
            key={"coffee"}
            active={false}
            onClick={() => {
              window.open("https://www.buymeacoffee.com/kenyadk", "_blank")
            }}
          />
        </Stack>
      </Navbar.Section>
    </Navbar>
  )
}