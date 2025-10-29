import { Group, Select, Tooltip, Box, Checkbox, MultiSelect, Button, Divider, Paper, RangeSlider, Modal, Flex, Stack } from "@mantine/core";
import { TauriTypes } from "$types";
import { useForm } from "@mantine/form";
import { useTranslateCommon, useTranslateEnums, useTranslateForms } from "@hooks/useTranslate.hook";
import { useEffect, useState } from "react";
import { SelectMultipleItems } from "@components/Forms/SelectMultipleItems";
import api from "@api/index";
import { useQuery } from "@tanstack/react-query";
import { faHandshake, faTrash } from "@fortawesome/free-solid-svg-icons";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { SelectItemTags } from "@components/Forms/SelectItemTags";
import { FieldFilter, Operator, OperatorType } from "@utils/filter.helper";
import { useDisclosure } from "@mantine/hooks";
import { TextTranslate } from "@components/Shared/TextTranslate";
import { notifications } from "@mantine/notifications";
import { DataTable } from "mantine-datatable";
import { CreateItemForm } from "@components/Forms/CreateItem";
export type GeneralPanelProps = {
  value: TauriTypes.SettingsLiveScraper;
  onSubmit: (value: TauriTypes.SettingsLiveScraper) => void;
  setHideTab?: (value: boolean) => void;
};
enum ViewMode {
  General = "general",
  Blacklist = "blacklist",
  BuyList = "buy_list",
}
interface BlackList extends TauriTypes.BlackListItemSetting {
  name?: string;
  trade_tax?: number;
  mr_requirement?: number;
}

interface FilterProps {
  query: string;
  tags?: string[];
  tradeTaxRange?: [number, number];
  mrRequirementRange?: [number, number];
  disable_for?: TauriTypes.TradeMode[];
}

export const GeneralPanel = ({ value, onSubmit, setHideTab }: GeneralPanelProps) => {
  // States
  const [viewMode, setViewMode] = useState<ViewMode>(ViewMode.General);
  const [opened, { open, close }] = useDisclosure(false);
  const [availableItems, setAvailableItems] = useState<BlackList[]>([]);

  // Fetch data from rust side
  const { data: tradableItems } = useQuery({
    queryKey: ["cache_items"],
    queryFn: () => api.cache.getTradableItems(),
  });
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.live_scraper.general.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateStockMode = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`stock_mode.${key}`, { ...context }, i18Key);
  const useTranslateTradeMode = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`trade_mode.${key}`, { ...context }, i18Key);

  const useTranslateBlacklist = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`black_list.${key}`, { ...context }, i18Key);

  useEffect(() => {
    if (!tradableItems) return;
    setAvailableItems(
      tradableItems.map((item) => {
        return {
          ...item,
          name: item.name,
          trade_tax: item.trade_tax,
          mr_requirement: item.mr_requirement,
          tags: item.tags,
          disabled_for: value.stock_item.blacklist?.find((bl) => bl.wfm_id === item.wfm_id)?.disabled_for || [],
        } as BlackList;
      })
    );
  }, [tradableItems]);

  // Form
  const form = useForm({
    initialValues: value,
    validate: {},
  });
  const formLeft = useForm({
    initialValues: {
      query: "",
      tags: [],
      tradeTaxRange: [0, 2100000],
      mrRequirementRange: [0, 15],
      disable_for: [],
    } as FilterProps,
  });
  const formRight = useForm({ initialValues: { query: "" } });

  const BuildFilter = (values: FilterProps) => {
    const filters: FieldFilter[] = [];

    // Search by name (case-insensitive substring)
    if (values.query) {
      filters.push({
        name: {
          type: OperatorType.STRING,
          [Operator.MATCHES]: values.query, // regex-like match
          isCaseSensitive: false,
        },
      });
    }

    // Tags (overlap check)
    if (values?.tags && values?.tags?.length > 0) {
      filters.push({
        tags: {
          type: OperatorType.ARRAY,
          [Operator.CONTAINS_VALUE]: values.tags, // checks tags array contains one of values
          isCaseSensitive: false,
        },
      });
    }

    // Trade tax range
    if (values.tradeTaxRange)
      filters.push({
        trade_tax: {
          type: OperatorType.NUMBER,
          [Operator.BETWEEN_VALUES]: values.tradeTaxRange,
        },
      });

    // MR requirement range
    if (values.mrRequirementRange)
      filters.push({
        mr_requirement: {
          type: OperatorType.NUMBER,
          [Operator.BETWEEN_VALUES]: values.mrRequirementRange,
        },
      });

    return {
      AND: filters, // All must match
    };
  };

  const GetNameById = (id: string) => {
    const item = tradableItems?.find((item) => item.wfm_id === id);
    return item ? item.name : id;
  };

  return (
    <Box h="100%" p={"md"}>
      <Modal zIndex={299} opened={opened} onClose={close} withCloseButton={false} centered>
        <MultiSelect
          label={useTranslateFormFields("trade_modes.label")}
          description={useTranslateFormFields("trade_modes.description")}
          w={250}
          data={Object.values(TauriTypes.TradeMode).map((status) => {
            return { value: status, label: useTranslateTradeMode(status) };
          })}
          {...formLeft.getInputProps("disable_for")}
        />
      </Modal>
      {viewMode == ViewMode.General && (
        <form
          onSubmit={form.onSubmit((values) => {
            onSubmit(values);
          })}
        >
          <Group gap="md">
            <Select
              allowDeselect={false}
              label={useTranslateFormFields("stock_mode.label")}
              description={useTranslateFormFields(`stock_mode.description.${form.values.stock_mode}`)}
              placeholder={useTranslateFormFields("stock_mode.placeholder")}
              data={Object.values(TauriTypes.StockMode).map((status) => {
                return { value: status, label: useTranslateStockMode(status) };
              })}
              value={form.values.stock_mode}
              onChange={(event) => form.setFieldValue("stock_mode", event as TauriTypes.StockMode)}
              error={form.errors.stock_mode && useTranslateFormFields("stock_mode.error")}
              radius="md"
            />
            <MultiSelect
              disabled={form.values.stock_mode != TauriTypes.StockMode.Item && form.values.stock_mode != TauriTypes.StockMode.All}
              label={useTranslateFormFields("trade_modes.label")}
              w={250}
              description={useTranslateFormFields(`trade_modes.description`)}
              data={Object.values(TauriTypes.TradeMode).map((status) => {
                return { value: status, label: useTranslateTradeMode(status) };
              })}
              value={form.values.trade_modes}
              onChange={(event) => form.setFieldValue("trade_modes", event as TauriTypes.TradeMode[])}
              error={form.errors.trade_modes && useTranslateFormFields("trade_mode.error")}
              radius="md"
            />
          </Group>
          <Group gap={"md"} mt={25}>
            <Tooltip label={useTranslateFormFields("report_to_wfm.tooltip")}>
              <Checkbox
                label={useTranslateFormFields("report_to_wfm.label")}
                checked={form.values.report_to_wfm}
                onChange={(event) => form.setFieldValue("report_to_wfm", event.currentTarget.checked)}
                error={form.errors.report_to_wfm && useTranslateFormFields("report_to_wfm.error")}
              />
            </Tooltip>
            <Tooltip label={useTranslateFormFields("auto_delete.tooltip")}>
              <Checkbox
                label={useTranslateFormFields("auto_delete.label")}
                checked={form.values.auto_delete}
                onChange={(event) => form.setFieldValue("auto_delete", event.currentTarget.checked)}
                error={form.errors.auto_delete && useTranslateFormFields("auto_delete.error")}
              />
            </Tooltip>
            <Tooltip label={useTranslateFormFields("auto_trade.tooltip")}>
              <Checkbox
                label={useTranslateFormFields("auto_trade.label")}
                checked={form.values.auto_trade}
                onChange={(event) => form.setFieldValue("auto_trade", event.currentTarget.checked)}
                error={form.errors.auto_trade && useTranslateFormFields("auto_trade.error")}
              />
            </Tooltip>
            <Tooltip label={useTranslateFormFields("should_delete_other_types.tooltip")}>
              <Checkbox
                label={useTranslateFormFields("should_delete_other_types.label")}
                checked={form.values.should_delete_other_types}
                onChange={(event) => form.setFieldValue("should_delete_other_types", event.currentTarget.checked)}
                error={form.errors.should_delete_other_types && useTranslateFormFields("should_delete_other_types.error")}
              />
            </Tooltip>
          </Group>
          <Divider my={"md"} />
          <Group gap="md">
            <Button
              onClick={() => {
                if (setHideTab) setHideTab(true);
                setViewMode(ViewMode.Blacklist);
              }}
            >
              {useTranslateForm("buttons.edit_blacklist_label", { count: form.values.stock_item.blacklist?.length || 0 })}
            </Button>
            <Button
              onClick={() => {
                if (setHideTab) setHideTab(true);
                setViewMode(ViewMode.BuyList);
              }}
            >
              {useTranslateForm("buttons.edit_buy_list_label", { count: form.values.stock_item.buy_list?.length || 0 })}
            </Button>
          </Group>
          <Group
            justify="flex-end"
            style={{
              position: "absolute",
              bottom: 25,
              right: 25,
            }}
          >
            <Button type="submit" variant="light" color="blue">
              {useTranslateCommon("buttons.save.label")}
            </Button>
          </Group>
        </form>
      )}
      {viewMode == ViewMode.Blacklist && (
        <Stack>
          <SelectMultipleItems<BlackList>
            leftTitle={useTranslateBlacklist("available_items_label")}
            rightTitle={useTranslateBlacklist("selected_items_label")}
            selectedItems={form.values.stock_item.blacklist || []}
            onChange={(items) => form.setFieldValue("stock_item", { ...form.values.stock_item, blacklist: items })}
            getId={(item) => item.wfm_id}
            onBeforeAdd={(item) => {
              if (!formLeft.values.disable_for || formLeft.values.disable_for.length === 0) {
                notifications.show({ color: "red.7", message: useTranslateBlacklist("no_trade_selected") });
                open();
                return false;
              }
              item.disabled_for = formLeft.values.disable_for ?? [];
              return true;
            }}
            onBeforeAddAll={(items) => {
              if (!formLeft.values.disable_for || formLeft.values.disable_for.length === 0) {
                notifications.show({ color: "red.7", message: useTranslateBlacklist("no_trade_selected") });
                return false;
              }
              items.forEach((item) => {
                item.disabled_for = formLeft.values.disable_for ?? [];
              });
              return true;
            }}
            leftConfig={{
              items: availableItems || [],
              idAccessor: "wfm_id",
              columns: [
                { sortable: true, accessor: "name", title: useTranslateBlacklist("name_title") },
                { sortable: true, accessor: "trade_tax", title: useTranslateBlacklist("trade_tax_title") },
                { sortable: true, accessor: "mr_requirement", title: useTranslateBlacklist("mr_requirement_title") },
              ],
              searchable: true,
              searchValue: formLeft.values.query || "",
              onSearchChange: (val) => formLeft.setFieldValue("query", val),
              searchFilter: (
                <Paper p={"sm"} mt={"md"}>
                  <Group>
                    <Flex direction="column">
                      <TextTranslate
                        i18nKey={useTranslateBlacklist("mr_requirement_label", undefined, true)}
                        values={{ min: formLeft.values.mrRequirementRange?.at(0) || 0, max: formLeft.values.mrRequirementRange?.at(1) || 15 }}
                      />
                      <RangeSlider
                        w={"150px"}
                        color="blue"
                        step={1}
                        minRange={1}
                        min={0}
                        max={15}
                        {...formLeft.getInputProps("mrRequirementRange")}
                      />
                    </Flex>
                    <Flex direction="column">
                      <TextTranslate
                        i18nKey={useTranslateBlacklist("trade_tax_label", undefined, true)}
                        values={{ min: formLeft.values.tradeTaxRange?.at(0) || 0, max: formLeft.values.tradeTaxRange?.at(1) || 2100000 }}
                      />
                      <RangeSlider w={"150px"} color="blue" step={1000} min={0} max={2100000} {...formLeft.getInputProps("tradeTaxRange")} />
                    </Flex>
                  </Group>
                  <Group>
                    <SelectItemTags value={formLeft.values.tags || []} onChange={(value) => formLeft.setFieldValue("tags", value)} />
                  </Group>
                </Paper>
              ),
              filter: BuildFilter(formLeft.values),
              searchRightSectionWidth: 100,
              searchRightSection: (
                <ActionWithTooltip
                  color="yellow.7"
                  tooltip={useTranslateBlacklist("set_trade_modes_tooltip", {
                    modes: formLeft.values.disable_for && formLeft.values.disable_for.length > 0 ? formLeft.values.disable_for.join(", ") : "N/A",
                  })}
                  icon={faHandshake}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={open}
                />
              ),
            }}
            rightConfig={{
              items: availableItems || [],
              idAccessor: "wfm_id",
              searchable: true,
              searchValue: formRight.values.query || "",
              searchRightSectionWidth: 45,
              onSearchChange: (val) => formRight.setFieldValue("query", val),
              filter: BuildFilter(formRight.values),
              columns: [
                { sortable: true, accessor: "name", title: useTranslateBlacklist("name_title") },
                { sortable: true, accessor: "trade_tax", title: useTranslateBlacklist("trade_tax_title") },
                { sortable: true, accessor: "mr_requirement", title: useTranslateBlacklist("mr_requirement_title") },
                {
                  accessor: "disabled_for",
                  title: useTranslateBlacklist("disabled_for_title"),
                  render: (item) => item.disabled_for?.map((mode) => useTranslateTradeMode(mode)).join(", ") || "N/A",
                },
              ],
            }}
          />
          <Button
            color="blue"
            variant="light"
            onClick={() => {
              setHideTab && setHideTab(false);
              setViewMode(ViewMode.General);
            }}
          >
            {useTranslateBlacklist("go_back_label")}
          </Button>
        </Stack>
      )}
      {viewMode == ViewMode.BuyList && (
        <Stack>
          <CreateItemForm
            idField="wfm_id"
            hide_sub_type
            hide_quantity
            onSubmit={(values) => {
              if (form.values.stock_item.buy_list.find((buyItem) => buyItem.wfm_id === values.raw) || values.bought <= 0) return;
              form.setFieldValue("stock_item", {
                ...form.values.stock_item,
                buy_list: form.values.stock_item.buy_list.concat([{ wfm_id: values.raw, max_price: values.bought }]),
              });
            }}
          />
          <DataTable
            height={"60vh"}
            withColumnBorders
            withTableBorder
            striped
            records={form.values.stock_item.buy_list || []}
            columns={[
              {
                accessor: "wfm_id",
                title: useTranslateForm("columns.name"),
                render: (item) => GetNameById(item.wfm_id),
              },
              {
                accessor: "max_price",
                title: useTranslateForm("columns.maximum_price"),
              },
              {
                accessor: "actions",
                title: useTranslateCommon("datatable_columns.actions.title"),
                render: (item) => (
                  <ActionWithTooltip
                    tooltip={useTranslateCommon("datatable_columns.actions.buttons.delete_tooltip")}
                    icon={faTrash}
                    color="red.7"
                    actionProps={{ size: "sm" }}
                    iconProps={{ size: "xs" }}
                    onClick={() => {
                      form.setFieldValue("stock_item", {
                        ...form.values.stock_item,
                        buy_list: form.values.stock_item.buy_list.filter((buyItem) => buyItem.wfm_id !== item.wfm_id),
                      });
                    }}
                  />
                ),
              },
            ]}
          />
          <Button
            color="blue"
            variant="light"
            onClick={() => {
              setHideTab && setHideTab(false);
              setViewMode(ViewMode.General);
            }}
          >
            {useTranslateBlacklist("go_back_label")}
          </Button>
        </Stack>
      )}
    </Box>
  );
};
