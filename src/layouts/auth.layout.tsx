
import { Outlet } from "react-router-dom";
import { useState } from 'react';
import { AppShell, useMantineTheme } from '@mantine/core';
import HeaderC from "./header";
export default function AuthLayout() {
  const theme = useMantineTheme();
  const [opened, setOpened] = useState(false);
  return (
    <AppShell
      styles={{
        main: {
          background: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0],
        },
      }}
      header={<HeaderC user={undefined} opened={opened} onOpenedClick={() => setOpened((o) => !o)} hideSidebar={false} setHideSidebar={() => { }} />}
    >
      <Outlet />
    </AppShell>
  );
}
