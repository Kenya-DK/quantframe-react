import { Alert, Center } from "@mantine/core";
import { useEffect } from "react";
import { useAppContext } from "@contexts/index";
import { useNavigate } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faExclamationTriangle } from "@fortawesome/free-solid-svg-icons";

export default function ErrorPage() {
  const { app_error } = useAppContext();
  const navigate = useNavigate();
  useEffect(() => {
    if (!app_error)
      navigate('/')
  }, [app_error])

  // States

  // // Translate general
  // const useTranslatePage = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`auth.${key}`, { ...context }, i18Key)
  // const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`errors.${key}`, { ...context }, i18Key)
  // const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`success.${key}`, { ...context }, i18Key)
  // const useTranslateProgress = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`progress.${key}`, { ...context }, i18Key)

  return (
    <Center w={"100%"} h={"92vh"}>
      <Alert color="red" title="Alert title" icon={<FontAwesomeIcon icon={faExclamationTriangle} />} maw={"75%"}>
        <code>
          {JSON.stringify(app_error, null, 2)}
        </code>
      </Alert>
    </Center>
  );
}
