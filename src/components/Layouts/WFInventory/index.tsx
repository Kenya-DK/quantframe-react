import { AddMetric } from "@api/index";
import { Header } from "@components/Layouts/Shared/Header";
import { NavbarLinkProps, NavbarMinimalColored } from "@components/Layouts/Shared/NavbarMinimalColored";
import { faUndo, faGem, faStar } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { AppShell, Box } from "@mantine/core";
import { useCallback, useMemo, useState } from "react";
import { Outlet, useNavigate } from "react-router-dom";
import classes from "./WFInventoryLayout.module.css";

/**
 * Full-screen layout for the "next" WF inventory surface mirrors the LogIn
 * layout: shared header (no clock) and the app's icon navigation rail, with a
 * "back to original" link at the bottom exiting the surface.
 */
export function WFInventoryLayout() {
  // States
  const [lastPage, setLastPage] = useState<string>("");
  // Contexts
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`layout.wf_inventory.${key}`, { ...context }, i18Key);
  const useTranslateNavBar = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`navbar.${key}`, { ...context }, i18Key);

  // States
  const navigate = useNavigate();
  const handleNavigate = useCallback(
    (link: NavbarLinkProps) => {
      navigate(link.link);

      if (link.id == lastPage || !link.id) return;
      setLastPage(link.id || "");
      AddMetric("page_view", { page: link.id });
    },
    [navigate, lastPage],
  );

  const links = useMemo(
    () => [
      {
        align: "top",
        id: "mods",
        link: "/wf_inventory_next/mods",
        icon: <FontAwesomeIcon size={"lg"} icon={faStar} />,
        label: useTranslateNavBar("mods"),
        onClick: (e: NavbarLinkProps) => handleNavigate(e),
      },
      {
        align: "top",
        id: "rivens",
        link: "/wf_inventory_next/rivens",
        icon: <FontAwesomeIcon size={"lg"} icon={faGem} />,
        label: useTranslateNavBar("rivens"),
        onClick: (e: NavbarLinkProps) => handleNavigate(e),
      },
      {
        align: "bottom",
        id: "back_to_original",
        link: "/wf_inventory",
        icon: <FontAwesomeIcon size={"lg"} icon={faUndo} />,
        label: useTranslateNavBar("back_to_original"),
        onClick: (e: NavbarLinkProps) => handleNavigate(e),
      },
    ],
    [handleNavigate],
  );

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
        <Header showClock={false} userMenuBackTo="/wf_inventory" />
      </AppShell.Header>

      <AppShell.Navbar withBorder={false}>
        <NavbarMinimalColored links={links} defaultActive="mods" />
      </AppShell.Navbar>

      <AppShell.Main>
        <Box>
          <Outlet />
        </Box>
      </AppShell.Main>
    </AppShell>
  );
}
