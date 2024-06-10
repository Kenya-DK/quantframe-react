import { Alert, Center, Text } from "@mantine/core";
import { useEffect } from "react";
import { useAppContext } from "@contexts/index";
import { useNavigate } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faExclamationTriangle } from "@fortawesome/free-solid-svg-icons";
import { useTranslatePages } from "../../hooks";

export default function ErrorPage() {
  const { app_error } = useAppContext();
  const navigate = useNavigate();
  useEffect(() => {
    if (!app_error)
      navigate('/')
  }, [app_error])

  // States

  // Translate general
  const useTranslatePage = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`error.${key}`, { ...context }, i18Key)

  return (
    <Center w={"100%"} h={"92vh"}>
      <Alert color="red" title={useTranslatePage("title", { component: app_error?.component })} icon={<FontAwesomeIcon icon={faExclamationTriangle} />} maw={"75%"}>
        <Text>{useTranslatePage("backtrace", { backtrace: app_error?.backtrace })}</Text>
        <Text>{useTranslatePage("cause", { cause: app_error?.cause })}</Text>
        <Text>{useTranslatePage("backtrace", { backtrace: app_error?.backtrace })}</Text>
        <Text>{useTranslatePage("footer", {})}</Text>
      </Alert>
    </Center>
  );
}
