import { AppShell, Box, Indicator } from "@mantine/core";
import classes from "./LogInLayout.module.css";
import { Outlet, useNavigate } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faBug, faChartSimple, faDesktop, faEnvelope, faGlobe, faHome, faInfoCircle } from "@fortawesome/free-solid-svg-icons";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { useAppContext } from "@contexts/app.context";
import { useEffect, useState } from "react";
import { NavbarLinkProps, NavbarMinimalColored } from "@components/NavbarMinimalColored";
import { SvgIcon, SvgType } from "@components/SvgIcon";
import { Header } from "@components/Header";
import api from "@api/index";
import { useAuthContext } from "@contexts/auth.context";
import { Ticker } from "@components/Ticker";
import { Alert } from "@api/types";
import { open } from "@tauri-apps/plugin-shell";

export function LogInLayout() {
  // States
  const [lastPage, setLastPage] = useState<string>("");
  // Contexts
  const { app_error, alerts } = useAppContext();
  const { user } = useAuthContext();
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`layout.log_in.${key}`, { ...context }, i18Key);
  const useTranslateNavBar = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`navbar.${key}`, { ...context }, i18Key);
  // States
  const navigate = useNavigate();
  const links = [
    {
      align: "top",
      id: "home",
      link: "/",
      icon: <FontAwesomeIcon icon={faHome} />,
      label: useTranslateNavBar("home"),
      onClick: (e: NavbarLinkProps) => handleNavigate(e),
    },
    {
      align: "top",
      id: "live-trading",
      link: "live-trading",
      icon: <FontAwesomeIcon icon={faGlobe} />,
      label: useTranslateNavBar("live_trading"),
      onClick: (e: NavbarLinkProps) => handleNavigate(e),
    },
    {
      align: "top",
      id: "chats",
      link: "chats",
      icon: (
        <Indicator
          disabled={(user?.unread_messages || 0) <= 0}
          label={(user?.unread_messages || 0) > 0 ? user?.unread_messages : undefined}
          inline
          size={16}
          position="top-start"
        >
          <FontAwesomeIcon icon={faEnvelope} />
        </Indicator>
      ),
      onClick: (e: NavbarLinkProps) => handleNavigate(e),
      label: useTranslateNavBar("chats"),
    },
    // { link: "statistics", icon: <FontAwesomeIcon icon={faChartSimple} />, label: useTranslate("statistics") },
    {
      align: "top",
      id: "warframe_market",
      link: "warframe-market",
      icon: <SvgIcon svgProp={{ width: 32, height: 32, fill: "#d5d7e0" }} iconType={SvgType.Default} iconName={"wfm_logo"} />,
      label: useTranslateNavBar("warframe_market"),
      onClick: (e: NavbarLinkProps) => handleNavigate(e),
    },
    {
      align: "top",
      id: "prices",
      link: "prices",
      icon: <FontAwesomeIcon icon={faChartSimple} />,
      label: useTranslateNavBar("prices"),
      onClick: (e: NavbarLinkProps) => handleNavigate(e),
    },
    {
      align: "top",
      id: "debug",
      link: "debug",
      icon: <FontAwesomeIcon icon={faDesktop} />,
      label: useTranslateNavBar("debug"),
      onClick: (e: NavbarLinkProps) => handleNavigate(e),
    },
    {
      align: "top",
      id: "test",
      link: "test",
      hide: !import.meta.env.DEV,
      icon: <FontAwesomeIcon icon={faBug} color="red" />,
      label: useTranslateNavBar("test"),
      onClick: (e: NavbarLinkProps) => handleNavigate(e),
    },
    {
      align: "bottom",
      id: "nav_about",
      link: "about",
      icon: <FontAwesomeIcon icon={faInfoCircle} />,
      label: useTranslateNavBar("about"),
      onClick: (e: NavbarLinkProps) => handleNavigate(e),
    },
  ];
  // Effects
  useEffect(() => {
    if (app_error) navigate("/error");
  }, [app_error]);
  useEffect(() => {
    if (user?.qf_banned || user?.wfm_banned) navigate("/error/banned");
  }, [user]);
  const handleNavigate = (link: NavbarLinkProps) => {
    console.log("Navigate to: ", link);
    if (link.web) open(link.link, "_blank");
    else navigate(link.link);

    if (link.id == lastPage || !link.id) return;
    setLastPage(link.id || "");
    switch (link.id) {
      default:
        api.analytics.sendMetric("Active_Page", link.id);
        break;
    }
  };
  const handleAlertClick = (alert: Alert) => {
    console.log("Alert clicked: ", alert);
    if (!alert.properties) return;
    const { event, payload } = alert.properties as { event: string; payload: any };
    if (!event) return;
    switch (event) {
      case "open_url":
        if (payload) open(payload);
        break;
      default:
        break;
    }
    console.log("Alert clicked: ", alert);
  };
  return (
    <AppShell
      classNames={classes}
      header={{ height: 65 }}
      navbar={{
        width: 70,
        breakpoint: "sm",
      }}
    >
      <AppShell.Header withBorder={false}>
        <Header />
        {alerts.length > 0 && (
          <Ticker
            data={alerts.map((alert) => ({
              label: alert.context,
              props: {
                "data-alert-type": alert.type,
                "data-color-mode": "text",
              },
              onClick: alert.properties ? () => handleAlertClick(alert) : undefined,
            }))}
          />
        )}
      </AppShell.Header>

      <AppShell.Navbar withBorder={false}>
        <NavbarMinimalColored links={links} />
      </AppShell.Navbar>

      <AppShell.Main>
        <Box mt={alerts.length > 0 ? 30 : 0}>
          <Outlet />
        </Box>
      </AppShell.Main>
    </AppShell>
  );
}
