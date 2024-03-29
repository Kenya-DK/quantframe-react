import { Group, TextInput } from "@mantine/core";
import { StockItemDto } from "../../../../types";
interface GeneralProps {
  item: StockItemDto
}

export function GeneralPanel({ item }: GeneralProps) {
  return (
    <>
      <Group >
        <TextInput
          readOnly
          label={("Rank")}
          value={item.rank}
        />
        <TextInput
          readOnly
          label={("Bought Price")}
          value={item.price}
        />
        <TextInput
          readOnly
          label={("Posted Price On Market")}
          value={item.listed_price || "N/A"}
        />
      </Group >
      <Group >
        <TextInput
          readOnly
          label={("Owned")}
          value={item.owned}
        />
        <TextInput
          readOnly
          label={("Min Price")}
          value={item.minium_price || "N/A"}
        />
        <TextInput
          readOnly
          label={("Status")}
          value={item.status}
        />
      </Group>
      <Group >
        <TextInput
          readOnly
          label={("Current Sellers")}
          value={item.trades?.length || 0}
        />
      </Group>
      <Group >
        {item.trades?.slice(0, 5).map((trade, index) => (
          <TextInput
            key={index}
            readOnly
            label={("Seller") + ` ${index + 1}`}
            value={trade.platinum}
          />
        ))}
      </Group>
    </>
  );
}