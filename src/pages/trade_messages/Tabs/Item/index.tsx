import { useState } from "react";
import { TradeEntryList } from "../../helpers/TradeEntryList";
import { TauriTypes } from "$types";
import { SelectTradableItem } from "@components/Forms/SelectTradableItem";
interface ItemPanelProps {
  isActive?: boolean;
}

export const ItemPanel = ({ isActive }: ItemPanelProps = {}) => {
  // States
  const [tradeEntry, setTradeEntry] = useState<TauriTypes.CreateTradeEntry & { wfm_url: string }>({
    raw: "",
    wfm_url: "",
    price: 1,
    group: "item",
  });

  return (
    <TradeEntryList
      createComponent={
        <SelectTradableItem
          value={tradeEntry?.wfm_url || ""}
          onChange={(item) => setTradeEntry({ ...tradeEntry, sub_type: item.sub_type, raw: item.wfm_id, wfm_url: item.wfm_url_name })}
        />
      }
      isActive={isActive}
      group="item"
      hideColumns={["potential_profit", "min_price"]}
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
