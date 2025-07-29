import { useTranslateModals } from "@hooks/useTranslate.hook";
import { AppError } from "../../../model/appError";
import { Text, Stack, Paper, Accordion, Code, Box, JsonInput } from "@mantine/core";
import { TauriTypes } from "$types";
export interface ErrorModalProps {
  app_info: TauriTypes.AppInfo | undefined;
  error: AppError;
}

export function ErrorModal({ error }: ErrorModalProps) {
  const useTranslateModal = (key: string, ctx?: Record<string, any>, raw?: boolean) => useTranslateModals(`app_error.${key}`, ctx, raw);

  return (
    <Box>
      <Stack gap="md">
        {/* Component */}
        <Box>
          <Text size="sm" fw={500} mb="xs" c="dimmed">
            {useTranslateModal("component.label")}
          </Text>
          <Paper withBorder p={"3"}>
            <Code>{error.error.component}</Code>
          </Paper>
        </Box>
        {/* Cause */}
        <Box>
          <Text size="sm" fw={500} mb="xs" c="dimmed">
            {useTranslateModal("cause.label")}
          </Text>
          <Paper withBorder p="3">
            <Text fw={500}>{error.error.cause}</Text>
          </Paper>
        </Box>

        {/* Message */}
        <Box>
          <Text size="sm" fw={500} mb="xs" c="dimmed">
            {useTranslateModal("message.label")}
          </Text>
          <Paper withBorder p="3">
            <Text lineClamp={5}>{error.error.message}</Text>
          </Paper>
        </Box>

        {/* Location */}
        <Box>
          <Text size="sm" fw={500} mb="xs" c="dimmed">
            {useTranslateModal("location.label")}
          </Text>
          <Paper withBorder p="3">
            <Code>{error.error.location}</Code>
          </Paper>
        </Box>

        {/* Context */}
        {error.error.context && Object.keys(error.error.context).length > 0 && (
          <Accordion variant="contained">
            <Accordion.Item value="context">
              <Accordion.Control>
                <Text size="sm" fw={500}>
                  {useTranslateModal("context.label")}
                </Text>
              </Accordion.Control>
              <Accordion.Panel>
                <JsonInput value={JSON.stringify(error.error.context, null, 2)} autosize maxRows={5} readOnly />
              </Accordion.Panel>
            </Accordion.Item>
            <Accordion.Item value="conteaxt">
              <Accordion.Control>
                <Text size="sm" fw={500}>
                  {useTranslateModal("context.label")}
                </Text>
              </Accordion.Control>
              <Accordion.Panel>
                <JsonInput value={JSON.stringify(error.error.context, null, 2)} autosize maxRows={5} readOnly />
              </Accordion.Panel>
            </Accordion.Item>
          </Accordion>
        )}

        {/* Technical Details */}
        <Accordion variant="contained">
          <Accordion.Item value="technical">
            <Accordion.Control>
              <Text size="sm" fw={500}>
                {useTranslateModal("technical.label")}
              </Text>
            </Accordion.Control>
            <Accordion.Panel>
              <JsonInput value={JSON.stringify(error, null, 2)} autosize maxRows={5} readOnly />
            </Accordion.Panel>
          </Accordion.Item>
        </Accordion>
      </Stack>
    </Box>
  );
}
