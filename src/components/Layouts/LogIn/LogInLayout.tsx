import { AppShell, Box } from "@mantine/core";
import classes from "./LogInLayout.module.css";
import { Outlet, useNavigate } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faBug, faGlobe, faHome } from "@fortawesome/free-solid-svg-icons";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { useEffect, useState } from "react";
import { NavbarLinkProps, NavbarMinimalColored } from "@components/Layouts/Shared/NavbarMinimalColored";
import { Header } from "@components/Layouts/Shared/Header";
import { useAuthContext } from "@contexts/auth.context";
import { open } from "@tauri-apps/plugin-shell";
import { AddMetric } from "@api/index";
import { Ticker } from "@components/Layouts/Shared/Ticker";
import { QuantframeApiTypes } from "$types";
import { useAppContext } from "@contexts/app.context";
import faWarframeMarket from "@icons/facWarframeMarket";

export function LogInLayout() {
  // States
  const [lastPage, setLastPage] = useState<string>("");
  // Contexts
  const { user } = useAuthContext();
  const { alerts } = useAppContext();
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
      icon: <FontAwesomeIcon size={"lg"} icon={faHome} />,
      label: useTranslateNavBar("home"),
      onClick: (e: NavbarLinkProps) => handleNavigate(e),
    },
    {
      align: "top",
      id: "live_scraper",
      link: "live_scraper",
      icon: <FontAwesomeIcon size={"lg"} icon={faGlobe} />,
      label: useTranslateNavBar("live_scraper"),
      onClick: (e: NavbarLinkProps) => handleNavigate(e),
    },
    {
      align: "top",
      id: "warframe_market",
      link: "warframe-market",
      icon: <FontAwesomeIcon size={"xl"} icon={faWarframeMarket} />,
      label: useTranslateNavBar("warframe_market"),
      onClick: (e: NavbarLinkProps) => handleNavigate(e),
    },
    {
      align: "top",
      id: "debug",
      link: "debug",
      hide: !import.meta.env.DEV,
      icon: <FontAwesomeIcon size={"lg"} icon={faBug} color="red" />,
      label: useTranslateNavBar("debug"),
      onClick: (e: NavbarLinkProps) => handleNavigate(e),
    },
  ];
  // Effects
  useEffect(() => {
    if (user?.qf_banned || user?.wfm_banned) navigate("/error/banned");
  }, [user]);
  const handleNavigate = (link: NavbarLinkProps) => {
    if (link.web) open(link.link, "_blank");
    else navigate(link.link);

    if (link.id == lastPage || !link.id) return;
    setLastPage(link.id || "");
    AddMetric("active_page", link.id);
  };
  const handleAlertClick = (alert: QuantframeApiTypes.AlertDto) => {
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
        <Ticker
          data={alerts.map((alert) => ({
            label: alert.context,
            props: {
              "data-alert-type": alert.type,
              "data-color-mode": "text",
            },
            onClick: alert.properties ? () => handleAlertClick(alert) : undefined,
          }))}
          loop
        />
      </AppShell.Header>

      <AppShell.Navbar withBorder={false}>
        <NavbarMinimalColored links={links} />
      </AppShell.Navbar>

      <AppShell.Main>
        <Box>
          <Outlet />
        </Box>
      </AppShell.Main>
    </AppShell>
  );
}
