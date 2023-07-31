
import { Outlet } from "react-router-dom";
import { useContext, useState } from 'react';
import { AppShell, useMantineTheme } from '@mantine/core';
import Hedder from "./header";
import { AuthContext } from "$contexts/index";
import { useLocalStorage } from "@mantine/hooks";

export default function MainLayout() {
  const theme = useMantineTheme();
  const [opened, setOpened] = useState(false);
  const [hideSidebar, setHideSidebar] = useLocalStorage<boolean>({ key: "sidebar-opened", defaultValue: false });
  const authState = useContext(AuthContext)
  return (
    <AppShell
      styles={{
        main: {
          background: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0],
        },
      }}
      navbarOffsetBreakpoint="sm"
      asideOffsetBreakpoint="sm"
      header={<Hedder user={authState.user} opened={opened} onOpenedClick={() => setOpened((o) => !o)} hideSidebar={hideSidebar} setHideSidebar={setHideSidebar} />}
    >
      <Outlet />
    </AppShell>
  );
}
