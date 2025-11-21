import { useState } from "react";
import { TradeEntryList } from "../../helpers/TradeEntryList";
import { TauriTypes } from "$types";
import { SelectRivenWeapon } from "@components/Forms/SelectRivenWeapon";
interface RivenPanelProps {
  isActive?: boolean;
}

export const RivenPanel = ({ isActive }: RivenPanelProps = {}) => {
  // States
  const [tradeEntry, setTradeEntry] = useState<TauriTypes.CreateTradeEntry & { wfm_url: string }>({
    raw: "",
    wfm_url: "",
    price: 1,
    group: "riven",
  });
  return (
    <TradeEntryList
      createComponent={
        <SelectRivenWeapon
          value={tradeEntry?.wfm_url || ""}
          onChange={(item) => setTradeEntry({ ...tradeEntry, raw: item.wfm_id, wfm_url: item.wfm_url_name })}
        />
      }
      isActive={isActive}
      group="riven"
      tradeEntry={tradeEntry}
      setTradeEntry={setTradeEntry}
      defaultDisplaySettings={{
        prefix: "WTB ",
        suffix: " :heart:",
        template: "[<link>] <type><variant><rank><amber_stars><cyan_stars><price>",
        displaySettings: {},
      }}
    />
  );
};
