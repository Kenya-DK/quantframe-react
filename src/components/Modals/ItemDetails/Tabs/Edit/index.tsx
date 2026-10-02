import { TauriTypes } from "$types";
import { DynamicForm, DynamicFormItem } from "@components/Forms/DynamicForm";
import { SelectSubType } from "@components/Forms/SelectSubType";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
import { useTranslateCommon, useTranslateModals } from "@hooks/useTranslate.hook";
import { Box } from "@mantine/core";

interface Properties {
  t_type?: TauriTypes.CacheTradableItemSubType;
  bulk_tradable: boolean;
  [key: string]: any;
}
export type EditTabProps = {
  lookup: string;
  value: TauriTypes.StockItem<Properties> | TauriTypes.SyndicateItem<Properties> | TauriTypes.WishListItem<Properties>;
  onSave?: (item: TauriTypes.UpdateStockItem | TauriTypes.UpdateSyndicateItem | TauriTypes.UpdateWishListItem) => void;
};

type EditFormValues = {
  t_type?: TauriTypes.CacheTradableItemSubType;
  sub_type?: TauriTypes.SubType;
  bought?: number;
  owned?: number;
  quantity?: number;
  min_sma?: number;
  min_profit?: number;
  min_price?: number;
  max_price?: number;
  is_bulk?: boolean;
};

const PropertyFields = ["min_sma", "min_profit", "min_price", "max_price", "is_bulk"] as const;

export function EditTab({ lookup, value, onSave }: EditTabProps) {
  // Translate general
  const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`item_details.tabs.edit.${key}`, { ...context }, i18Key);

  const ShowField = (lookupKeys: string[]) => (lookupKeys.some((key) => key === lookup) ? "block" : "none");

  const GetProperty = (key: string) => {
    return value[key as keyof typeof value] ?? value.properties?.[key];
  };

  const formValue = { ...(value as any), ...(value.properties ?? {}) } as EditFormValues;

  const canUseBulk = (lookupKeys: string[]) => {
    let bulkTradable = GetProperty("bulk_tradable") ?? false;
    return lookupKeys.some((key) => key === lookup) && bulkTradable ? "block" : "none";
  };

  const items: DynamicFormItem<EditFormValues>[] = [
    {
      field: "sub_type",
      type: "custom",
      render: ({ value: subType, onChange, form }) => {
        const t_type = (form.values as any).t_type as TauriTypes.CacheTradableItemSubType | undefined;
        if (!t_type || !subType) return null;
        return <SelectSubType availableSubTypes={t_type} value={subType} onChange={onChange} />;
      },
    },
    {
      type: "group",
      props: { grow: true, mt: "xs" },
      fields: [
        { field: "bought", type: "number", props: { min: 0, display: ShowField(["stock_item"]) } },
        { field: "owned", type: "number", props: { min: 1, display: ShowField(["stock_item"]) } },
        { field: "quantity", type: "number", props: { min: 1, display: ShowField(["wish_list_item"]) } },
      ],
    },
    {
      type: "group",
      props: { grow: true, mt: "xs" },
      fields: [
        {
          field: "min_price",
          type: "number",
          props: {
            min: 0,
            display: ShowField(["stock_item", "syndicate_item", "wish_list_item"]),
            rightSection: <TooltipIcon label={useTranslateTab("tooltips.minimum_price")} />,
          },
        },
        {
          field: "max_price",
          type: "number",
          props: {
            display: ShowField(["wish_list_item"]),
            rightSection: <TooltipIcon label={useTranslateTab("tooltips.maximum_price")} />,
          },
        },
      ],
    },
    {
      type: "group",
      props: { grow: true, mt: "xs" },
      fields: [
        {
          field: "min_sma",
          type: "number",
          props: {
            min: -1,
            display: ShowField(["stock_item"]),
            rightSection: <TooltipIcon label={useTranslateTab("tooltips.minimum_sma")} />,
          },
        },
        {
          field: "min_profit",
          type: "number",
          props: {
            min: -1,
            display: ShowField(["stock_item"]),
            rightSection: <TooltipIcon label={useTranslateTab("tooltips.minimum_profit")} />,
          },
        },
      ],
    },
    { field: "is_bulk", type: "checkbox", props: { display: canUseBulk(["wish_list_item", "stock_item"]), mt: "xs" } },
  ];

  const handleSave = (values: EditFormValues) => {
    debugger;
    const properties: Record<string, any> = { ...(value.properties ?? {}) };

    for (const key of PropertyFields) {
      const fieldValue = (values as any)[key];
      if (fieldValue === "" || fieldValue === undefined || fieldValue === null) delete properties[key];
      else properties[key] = fieldValue;
    }

    const payload: Record<string, any> = {
      ...(value as any),
      properties,
    };

    for (const key of ["sub_type", "bought", "owned", "quantity"] as const) {
      const fieldValue = (values as any)[key];
      if (fieldValue === "" || fieldValue === undefined || fieldValue === null) delete payload[key];
      else payload[key] = fieldValue;
    }

    onSave?.(payload as any);
  };

  return (
    <Box>
      <DynamicForm<EditFormValues>
        i18n_key="components.modals.item_details.tabs.edit.fields"
        value={formValue}
        items={items}
        onSubmit={handleSave}
        confirmLabel={useTranslateCommon("buttons.save.label")}
      />
    </Box>
  );
}
