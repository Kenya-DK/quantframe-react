import { AppShell } from "@mantine/core";
import { Outlet, useNavigate } from "react-router-dom";
import { Header } from "@components/Header";
import classes from "./LogOutLayout.module.css";
import { useAppContext } from "@contexts/app.context";
import { useEffect } from "react";

export function LogOutLayout() {
  const { app_error } = useAppContext();
  const navigate = useNavigate();
  useEffect(() => {
    if (app_error)
      navigate('/error')
  }, [app_error])
  return (
    <AppShell
      classNames={classes}
      header={{ height: 65 }}
    >
      <AppShell.Header withBorder={false}>
        <Header />
      </AppShell.Header>

      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
}