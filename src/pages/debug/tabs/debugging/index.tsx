import { TauriTypes } from "$types";
import api, { SendTauriEvent } from "@api/index";
import { ItemName } from "@components/DataDisplay/ItemName";
import { DebuggingLiveItemEntryForm } from "@components/Forms/DebuggingLiveItemEntry";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { useAppContext } from "@contexts/app.context";
import { faPen, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { Badge, Box, Checkbox, Group, Tooltip } from "@mantine/core";
import { useState } from "react";
import { DataTable } from "mantine-datatable";
import classes from "../../Debug.module.css";
interface DebuggingPanelProps {}
export const DebuggingPanel = ({}: DebuggingPanelProps) => {
  // Context
  const { settings } = useAppContext();

  // Translate general
  const useTranslateTabDebugging = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`debug.tabs.debugging.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabDebugging(`datatable.columns.${key}`, { ...context }, i18Key);

  const [editingEntry, setEditingEntry] = useState<TauriTypes.DebuggingLiveItemEntry | null>(null);

  return (
    <Box>
      <Group align="center">
        <DebuggingLiveItemEntryForm
          key={editingEntry?.wfm_url ?? "new"}
          initialValues={editingEntry ?? undefined}
          isEditing={!!editingEntry}
          onCancel={() => setEditingEntry(null)}
          onSubmit={async (values) => {
            if (!settings) return;
            let items = [...(settings?.debugging.live_scraper.entries || [])];
            if (editingEntry) {
              const index = items.findIndex((e) => e.wfm_url === editingEntry.wfm_url);
              if (index !== -1) items[index] = values;
            } else {
              items.push(values);
            }
            await api.app.updateSettings({
              ...settings,
              debugging: { ...settings.debugging, live_scraper: { ...settings.debugging.live_scraper, entries: items } },
            });
            setEditingEntry(null);
            SendTauriEvent(TauriTypes.Events.RefreshSettings);
          }}
        />
        <Tooltip label={useTranslateTabDebugging("fake_orders_tooltip")}>
          <Checkbox
            label={useTranslateTabDebugging("fake_orders_label")}
            checked={settings?.debugging.live_scraper.fake_orders || false}
            size="sm"
            onChange={async (e) => {
              if (!settings) return;
              await api.app.updateSettings({
                ...settings,
                debugging: { ...settings.debugging, live_scraper: { ...settings.debugging.live_scraper, fake_orders: e.currentTarget.checked } },
              });
              SendTauriEvent(TauriTypes.Events.RefreshSettings);
            }}
          />
        </Tooltip>
      </Group>
      <DataTable
        className={`${classes.dataTableLogging}`}
        data-alert={useHasAlert()}
        mt={10}
        striped
        idAccessor={"wfm_url"}
        records={settings?.debugging.live_scraper.entries || []}
        columns={[
          {
            accessor: "wfm_url",
            title: useTranslateDataGridColumns("wfm_url.title"),
            render: (row) => <ItemName value={row} />,
          },
          { accessor: "stock_id", title: useTranslateDataGridColumns("stock_id") },
          { accessor: "wish_list_id", title: useTranslateDataGridColumns("wish_list_id") },
          { accessor: "priority", title: useTranslateDataGridColumns("priority") },
          { accessor: "buy_quantity", title: useTranslateDataGridColumns("buy_quantity") },
          { accessor: "sell_quantity", title: useTranslateDataGridColumns("sell_quantity") },
          {
            accessor: "operations",
            title: useTranslateDataGridColumns("operations"),
            render: (row) => (
              <Group gap={"sm"} justify="flex-start">
                {row.operations.map((op: string, index: number) => (
                  <Badge key={index}>{op}</Badge>
                ))}
              </Group>
            ),
          },
          {
            accessor: "actions",
            title: useTranslateDataGridColumns("actions.title"),
            width: 180,
            render: (row) => (
              <Group gap={"sm"} justify="flex-end">
                <ActionWithTooltip
                  tooltip={useTranslateDataGridColumns("actions.buttons.edit_tooltip")}
                  color={"blue.7"}
                  icon={faPen}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={(e) => {
                    e.stopPropagation();
                    setEditingEntry(row);
                  }}
                />
                <ActionWithTooltip
                  tooltip={useTranslateDataGridColumns("actions.buttons.delete_tooltip")}
                  color={"red.7"}
                  icon={faTrashCan}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={async (e) => {
                    e.stopPropagation();
                    if (!settings) return;
                    let items = settings?.debugging.live_scraper.entries.filter((item) => item.wfm_url !== row.wfm_url);
                    await api.app.updateSettings({
                      ...settings,
                      debugging: { ...settings.debugging, live_scraper: { ...settings.debugging.live_scraper, entries: items } },
                    });
                    SendTauriEvent(TauriTypes.Events.RefreshSettings);
                  }}
                />
              </Group>
            ),
          },
        ]}
      />
    </Box>
  );
};
