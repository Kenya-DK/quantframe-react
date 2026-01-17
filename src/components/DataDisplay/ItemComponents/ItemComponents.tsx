import { Group, Title } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { ItemComponent } from "../ItemComponent";

export function ItemComponents({ components }: { components: TauriTypes.ItemComponent[] }) {
  return (
    <>
      <Title order={3} mt={"md"}>
        {useTranslateComponent("item_components")}
      </Title>
      <Group align="center">
        {components
          .filter((x) => x.tradable)
          .map((component, index) => (
            <ItemComponent key={index} component={component} />
          ))}
      </Group>
    </>
  );
}
