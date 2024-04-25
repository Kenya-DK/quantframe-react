import { AppShell, Indicator } from "@mantine/core";
import { SvgIcon, Header, NavbarMinimalColored, SvgType, NavbarLinkProps } from "@components";
import classes from "./LogInLayout.module.css";
import { Outlet, useNavigate } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faDesktop, faEnvelope, faGlobe, faHome } from "@fortawesome/free-solid-svg-icons";
import { useTranslateComponent } from "@hooks/index";
import { useChatContext } from "@contexts/index";
export function LogInLayout() {
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`layout.log_in.${key}`, { ...context }, i18Key)
  const useTranslateNavBar = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`navbar.${key}`, { ...context }, i18Key)
  const { unread_messages } = useChatContext();
  // States
  const navigate = useNavigate();
  const links = [
    { link: "/", icon: <FontAwesomeIcon icon={faHome} />, label: useTranslateNavBar("home"), onClick: (e: NavbarLinkProps) => handleNavigate(e) },
    { link: "live-trading", icon: <FontAwesomeIcon icon={faGlobe} />, label: useTranslateNavBar("live_trading"), onClick: (e: NavbarLinkProps) => handleNavigate(e) },
    {
      hide: false, link: "chats", icon: <Indicator disabled={unread_messages == 0} label={unread_messages > 0 ? unread_messages : undefined} inline size={16} position="top-start"  >
        <FontAwesomeIcon icon={faEnvelope} />
      </Indicator>, label: useTranslateNavBar("chats")
    },
    // { link: "statistics", icon: <FontAwesomeIcon icon={faChartSimple} />, label: useTranslate("statistics") },
    { link: "warframe-market", icon: <SvgIcon svgProp={{ width: 32, height: 32, fill: "#d5d7e0" }} iconType={SvgType.Default} iconName={"wfm_logo"} />, label: useTranslateNavBar("warframe_market"), onClick: (e: NavbarLinkProps) => handleNavigate(e) },
    { link: "debug", icon: <FontAwesomeIcon icon={faDesktop} />, label: useTranslateNavBar("debug"), onClick: (e: NavbarLinkProps) => handleNavigate(e) },
  ];


  const handleNavigate = (link: NavbarLinkProps) => {
    navigate(link.link);
  };
  return (
    <AppShell
      classNames={classes}
      header={{ height: 65 }}
      navbar={{
        width: 70,
        breakpoint: 'sm',
      }}

    >
      <AppShell.Header withBorder={false}>
        <Header isHidden={false} />
      </AppShell.Header>

      <AppShell.Navbar withBorder={false}>
        <NavbarMinimalColored links={links} />
      </AppShell.Navbar>

      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
}