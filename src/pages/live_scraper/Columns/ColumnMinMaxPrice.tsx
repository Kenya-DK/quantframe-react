import { Group, Text } from "@mantine/core";
import { faEdit } from "@fortawesome/free-solid-svg-icons";
import { ButtonIntervals } from "@components/Shared/ButtonIntervals";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { useTranslateCommon } from "../../../hooks/useTranslate.hook";
export type ColumnMinMaxPriceProps = {
  i18nKey?: string;
  id: number;
  minimum_price: number | undefined;
  onEdit: (id: number, minimum_price: number) => void;
  onUpdate: (id: number, minimum_price: number) => void;
};

export function ColumnMinMaxPrice({ i18nKey, minimum_price, id, onEdit, onUpdate }: ColumnMinMaxPriceProps) {
  // Functions
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateCommon(`datatable_columns.${i18nKey ? i18nKey : `minimum_price`}.${key}`, { ...context }, i18Key);
  return (
    <Group gap={"sm"} justify="space-between">
      <Text>{minimum_price || "N/A"}</Text>
      <Group gap={"xs"}>
        <ButtonIntervals
          intervals={[5, 10]}
          minimum_price={minimum_price || 0}
          OnClick={async (val) => {
            if (!id) return;
            onUpdate(id, val);
          }}
        />
        <ActionWithTooltip
          tooltip={useTranslate("edit_tooltip")}
          icon={faEdit}
          onClick={(e) => {
            e.stopPropagation();
            if (!id) return;
            onEdit(id, minimum_price || 0);
          }}
          actionProps={{ size: "sm" }}
          iconProps={{ size: "xs" }}
        />
      </Group>
    </Group>
  );
}
