import { createStyles, rem, Tooltip, UnstyledButton, Navbar, Stack } from "@mantine/core";
import { ReactNode, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faChartSimple, faDesktop, faGlobe, faHome } from "@fortawesome/free-solid-svg-icons";
import { useTranslateLayout } from "../hooks";
import { useNavigate } from "react-router-dom";

interface NavbarLinkProps {
  icon: ReactNode;
  label: string;
  link: string;
  active?: boolean;
  onClick?(url: string): void;
}
function NavbarLink({ link, icon, label, active, onClick }: NavbarLinkProps) {
  const { classes, cx } = useStyles();
  return (
    <Tooltip label={label} position="right" transitionProps={{ duration: 0 }}>
      <UnstyledButton onClick={() => onClick && onClick(link)} className={cx(classes.link, { [classes.active]: active })}>
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
  const goTo = useNavigate();
  const [active, setActive] = useState(2);
  const useTranslate = (key: string, context?: { [key: string]: any }) => useTranslateLayout(`navigation.${key}`, { ...context })
  const mockdata = [
    { link: "/", icon: <FontAwesomeIcon icon={faHome} />, label: useTranslate("home") },
    { link: "live-trading", icon: <FontAwesomeIcon icon={faGlobe} />, label: useTranslate("live_trading") },
    { link: "statistics", icon: <FontAwesomeIcon icon={faChartSimple} />, label: useTranslate("statistics") },
    { link: "warframe-market", icon: <FontAwesomeIcon icon={faChartSimple} />, label: useTranslate("warframe_market") },
    { link: "rivens", icon: <FontAwesomeIcon icon={faChartSimple} />, label: useTranslate("rivens") },
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
        <Stack justify="center" spacing={0}>
          {links}
        </Stack>
      </Navbar.Section>
    </Navbar>
  )
}