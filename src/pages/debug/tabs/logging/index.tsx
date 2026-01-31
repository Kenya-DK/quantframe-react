import { Box, Button } from "@mantine/core";
import api from "@api/index";
import { useQuery } from "@tanstack/react-query";
import { SearchField } from "@components/Forms/SearchField";
import { useState } from "react";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faTrash } from "@fortawesome/free-solid-svg-icons";
import { DataTable } from "mantine-datatable";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import classes from "../../Debug.module.css";
import { modals } from "@mantine/modals";
interface LoggingPanelProps {}
export const LoggingPanel = ({}: LoggingPanelProps) => {
  const [search, setSearch] = useState("");
  const { data, refetch } = useQuery({
    queryKey: ["getLogging", search],
    queryFn: () => api.getLogging(search),
    retry: 0,
  });

  // Translate general
  const useTranslateTabLogging = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`debug.tabs.logging.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabLogging(`datatable.columns.${key}`, { ...context }, i18Key);
  const useTranslateBasePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabLogging(`prompt.${key}`, { ...context }, i18Key);
  const OpenAddLogModal = () => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateBasePrompt("name.title"),
      innerProps: {
        fields: [
          {
            name: "name",
            label: useTranslateBasePrompt("name.fields.name.label"),
            type: "text",
          },
        ],
        onConfirm: async (data: { name: string }) => {
          await api.addLog(data.name);
          refetch();
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };
  return (
    <Box>
      <SearchField value={search} onChange={(e) => setSearch(e)} />
      <DataTable
        className={`${classes.databaseLogging}`}
        data-alert={useHasAlert()}
        mt={10}
        striped
        idAccessor={"command"}
        records={data || []}
        columns={[
          { accessor: "command", title: useTranslateDataGridColumns("command") },
          { accessor: "count", title: useTranslateDataGridColumns("count") },
          {
            accessor: "actions",
            title: useTranslateDataGridColumns("actions.title"),
            render: (record) => (
              <ActionWithTooltip
                tooltip={useTranslateDataGridColumns("actions.buttons.remove_tooltip")}
                icon={faTrash}
                color="red"
                onClick={async (e) => {
                  e.stopPropagation();
                  await api.removeLog(record.command);
                  refetch();
                }}
              />
            ),
          },
        ]}
      />
      <Button mt={10} onClick={async () => OpenAddLogModal()}>
        {useTranslateTabLogging("buttons.add_log")}
      </Button>
    </Box>
  );
};
