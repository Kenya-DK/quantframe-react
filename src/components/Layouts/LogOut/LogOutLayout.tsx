import { AppShell } from "@mantine/core";
import { Outlet } from "react-router-dom";
import { Header } from "@components";
import classes from "./LogOutLayout.module.css";

export function LogOutLayout() {
  return (
    <AppShell
      classNames={classes}
      header={{ height: 65 }}
    >
      <AppShell.Header withBorder={false}>
        <Header isHidden />
      </AppShell.Header>

      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
}