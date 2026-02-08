import { AppShell } from "@mantine/core";
import { Outlet } from "react-router-dom";
import classes from "./CleanLayout.module.css";

export function CleanLayout() {
  return (
    <AppShell classNames={classes}>
      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
}
