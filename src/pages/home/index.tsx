import { Grid } from "@mantine/core";
import { MultiSelectListBox } from "../../components/multiSelectListBox";
import { useTauriContext } from "../../contexts";
import { useState } from "react";

export default function HomePage() {
  const { tradable_items } = useTauriContext();
  const [selectedItems, setSelectedItems] = useState<Array<string>>([]);
  return (
    <Grid>
      <Grid.Col md={12}>
        <MultiSelectListBox availableItems={tradable_items.map((warframe) => ({ label: warframe.item_name, value: warframe.url_name }))} selectedItems={selectedItems} onChange={(items: string[]) => {
          console.log(items);

          setSelectedItems(items);
        }} />
      </Grid.Col>
    </Grid>
  );
}