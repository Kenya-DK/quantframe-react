import { TradeEntryList } from "../../helpers/TradeEntryList";
import { TauriTypes } from "$types";
import { useState } from "react";
import { TextInput } from "@mantine/core";
interface CustomPanelProps {
  isActive?: boolean;
}

export const CustomPanel = ({ isActive }: CustomPanelProps = {}) => {
  // States
  const [tradeEntry, setTradeEntry] = useState<TauriTypes.CreateTradeEntry & { wfm_url: string }>({
    raw: "",
    wfm_url: "",
    price: 1,
    group: "custom",
  });
  return (
    <TradeEntryList
      createComponent={
        <TextInput
          value={tradeEntry?.wfm_url || ""}
          onChange={(e) => setTradeEntry({ ...tradeEntry, raw: e.currentTarget.value, wfm_url: e.currentTarget.value })}
          placeholder="Enter custom item name or URL"
        />
      }
      isActive={isActive}
      group="custom"
      tradeEntry={tradeEntry}
      setTradeEntry={setTradeEntry}
      defaultDisplaySettings={{
        prefix: "WTS ",
        suffix: " :heart:",
        template: "[<link>] <type><variant><rank><amber_stars><cyan_stars><price>",
        displaySettings: {},
      }}
    />
  );
};
