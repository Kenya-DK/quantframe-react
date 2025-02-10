import { Button, Container } from "@mantine/core";

import { useGetUser } from "@hooks/useGetUser.hook";
import { useAppContext } from "@contexts/app.context";
import { open } from "@tauri-apps/plugin-shell";
export default function TestPage() {
  const user = useGetUser();
  const { app_info } = useAppContext();
  return (
    <Container size={"100%"}>
      <Button
        disabled={!user?.id || !user?.check_code}
        onClick={async () =>
          open(
            `https://www.patreon.com/oauth2/authorize?response_type=code&client_id=6uDrK7uhMBAidiAvzQd7ukmHFz4NUXO1wocruae24C4_04rXrUMSvCzC9RKbQpmN&scope=identity%20identity%5Bemail%5D&redirect_uri=${
              app_info?.is_development ? "http://localhost:6969/patreon/link" : "https://api.quantframe/patreon/link"
            }&state=${user?.id}|${user?.check_code}`
          )
        }
      >
        Click Me
      </Button>
    </Container>
  );
}
