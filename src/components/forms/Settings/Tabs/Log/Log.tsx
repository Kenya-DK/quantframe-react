import { Box, Button, Group } from "@mantine/core";
import { useTranslateForms } from "@hooks/index";
import api from "@api/index";
import { useMutation } from "@tanstack/react-query";

export type LogPanelProps = {
}
export const LogPanel = ({ }: LogPanelProps) => {

  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForms(`settings.tabs.log.${key}`, { ...context }, i18Key)
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`buttons.${key}`, { ...context }, i18Key)

  const openLogFolderMutation = useMutation({
    mutationFn: () => api.log.open(),
    onError: (e) => { console.error(e); }
  })
  const exportLogsMutation = useMutation({
    mutationFn: () => api.log.export(),
    onError: (e) => { console.error(e); }
  })

  return (
    <Box p={"md"}>
      <Group>
        <Button onClick={() => openLogFolderMutation.mutate()} color="blue">
          {useTranslateButtons('open.label')}
        </Button>
        <Button onClick={() => exportLogsMutation.mutate()} color="blue">
          {useTranslateButtons('export.label')}
        </Button>
      </Group>
    </Box>
  );
};