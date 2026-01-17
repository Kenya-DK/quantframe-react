import { Center } from "@mantine/core";
import { useEffect } from "react";
import { useAppContext } from "@contexts/app.context";
import { useNavigate } from "react-router-dom";
import { AlertError } from "../../components/AlertError";

export default function ErrorPage() {
  const { app_error, app_info } = useAppContext();
  const navigate = useNavigate();
  useEffect(() => {
    if (!app_error) navigate("/");
  }, [app_error]);

  return (
    <Center w={"100%"} h={"92vh"}>
      <AlertError version={app_info?.version} error={app_error} />
    </Center>
  );
}
