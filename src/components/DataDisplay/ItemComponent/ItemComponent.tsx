import { Group } from "@mantine/core";
import { TauriTypes } from "$types";
import { TextTranslate } from "../../Shared/TextTranslate";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
export type ItemComponentProps = {
  component: TauriTypes.ItemComponent;
};

export function ItemComponent({ component }: ItemComponentProps) {
  return (
    <Group align="center">
      <TextTranslate
        size="lg"
        i18nKey={useTranslateComponent("item_component", undefined, true)}
        values={{ name: component.name, count: component.itemCount }}
      />
    </Group>
  );
}
