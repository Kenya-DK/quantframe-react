import { ErrorModal } from "@components/Modals/Error";
import { useAppContext } from "@contexts/app.context";
import { Center } from "@mantine/core";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";

export default function ErrorPage() {
  const { app_error, app_info } = useAppContext();
  const navigate = useNavigate();
  useEffect(() => {
    if (!app_error) navigate("/");
  }, [app_error]);

  return (
    <Center w={"100%"} h={"92vh"}>
      <ErrorModal app_info={app_info} error={app_error?.error} />
    </Center>
  );
}
