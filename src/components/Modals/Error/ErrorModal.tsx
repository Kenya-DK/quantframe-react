import { useTranslateModals } from "@hooks/useTranslate.hook";
import { Text, Alert, Box, Button } from "@mantine/core";
import { ResponseError, TauriTypes } from "$types";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faExclamationTriangle } from "@fortawesome/free-solid-svg-icons";
import { TextTranslate } from "../../Shared/TextTranslate";
import api from "@api/index";

export interface ErrorModalProps {
  app_info: TauriTypes.AppInfo | undefined;
  error: ResponseError | undefined;
}

export function ErrorModal({ error, app_info }: ErrorModalProps) {
  const useTranslateModal = (key: string, ctx?: Record<string, any>, raw?: boolean) => useTranslateModals(`app_error.${key}`, ctx, raw);

  const mutateExportLogs = api.log.export_logs();
  return (
    <Box>
      <Alert
        color="red"
        autoContrast
        title={useTranslateModal("title", { component: error?.component })}
        icon={<FontAwesomeIcon icon={faExclamationTriangle} />}
      >
        {app_info?.version && (
          <TextTranslate
            size="md"
            color="white"
            i18nKey={useTranslateModal("version", undefined, true)}
            values={{ version: app_info?.version || "0.0.0" }}
          />
        )}
        <TextTranslate size="md" color="white" i18nKey={useTranslateModal("cause", undefined, true)} values={{ cause: error?.cause || "Unknown" }} />
        <TextTranslate
          size="md"
          color="white"
          i18nKey={useTranslateModal("message", undefined, true)}
          values={{ message: error?.message || "No message provided" }}
        />
        <TextTranslate color="white" i18nKey={useTranslateModal("location", undefined, true)} values={{ location: error?.location || "Unknown" }} />
        <Text>{useTranslateModal("footer", {})}</Text>
      </Alert>
      <Button
        mt={16}
        loading={mutateExportLogs.isPending}
        onClick={() => {
          mutateExportLogs.mutate();
        }}
      >
        {useTranslateModal("export_log")}
      </Button>
    </Box>
  );
}
