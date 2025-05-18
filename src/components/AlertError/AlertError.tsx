import { Alert, Text } from "@mantine/core";
import { ResponseError } from "$types/index";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { faExclamationTriangle } from "@fortawesome/free-solid-svg-icons";

export type AlertErrorProps = {
  version?: string;
  error?: ResponseError;
};
export function AlertError({ version, error }: AlertErrorProps) {
  const useTranslateAlertError = (key: string, ctx?: Record<string, any>, raw?: boolean) => useTranslateComponent(`alert_error.${key}`, ctx, raw);
  return (
    <Alert
      color="red"
      title={useTranslateAlertError("title", { component: error?.component })}
      icon={<FontAwesomeIcon icon={faExclamationTriangle} />}
      maw={"75%"}
    >
      {version && <Text>{useTranslateAlertError("version", { version })}</Text>}
      <Text>{useTranslateAlertError("backtrace", { backtrace: error?.backtrace })}</Text>
      <Text>{useTranslateAlertError("cause", { cause: error?.cause })}</Text>
      <Text>{useTranslateAlertError("backtrace", { error: error?.backtrace })}</Text>
      <Text>{useTranslateAlertError("footer", {})}</Text>
    </Alert>
  );
}
