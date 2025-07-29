import { AppShell } from "@mantine/core";
import { Outlet, useNavigate } from "react-router-dom";
import { Header } from "@components/Layouts/Shared/Header";
import classes from "./LogOutLayout.module.css";
import { useEffect } from "react";
import { useAuthContext } from "@contexts/auth.context";

export function LogOutLayout() {
  const { user } = useAuthContext();
  const navigate = useNavigate();

  useEffect(() => {
    if (user?.qf_banned || user?.wfm_banned) navigate("/error/banned");
  }, [user]);
  return (
    <AppShell classNames={classes} header={{ height: 65 }}>
      <AppShell.Header withBorder={false}>
        <Header />
      </AppShell.Header>

      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
}
