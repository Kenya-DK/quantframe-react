import { useState } from "react";
import { TradeEntryList } from "../../helpers/TradeEntryList";
import { TauriTypes } from "$types";
import { SelectRivenWeapon } from "@components/Forms/SelectRivenWeapon";
import dayjs from "dayjs";
import utc from "dayjs";
import api, { HasPermission } from "@api/index";
import { modals } from "@mantine/modals";
import { FindInterestingRivensModal } from "./FindInterestingRivensModal";
dayjs.extend(utc);
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

  // Mutations
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
      onFindInteresting={async ({ createMultipleMutation }) => {
        let date = dayjs().subtract(1, "hours").startOf("hour").utc().toISOString();

        let items: any = [];
        if (await HasPermission(TauriTypes.PermissionsFlags.FIND_INTERESTING_RIVENS))
          items = await api.riven.getAll({ page: 1, limit: -1, from_date: date, to_date: date });
        modals.open({
          size: "100%",
          title: "",
          withCloseButton: false,
          children: (
            <FindInterestingRivensModal
              date={new Date(date)}
              rivens={items.results || []}
              onSubmit={(data) => {
                if (!createMultipleMutation) return;
                createMultipleMutation.mutate(
                  data.rivens.map((riven) => ({
                    raw: riven.wfm_id,
                    override_existing: data.overrideExistingPrices,
                    price: riven.discount_price,
                    group: "riven",
                    tags: data.tags,
                    properties: {
                      min_price: riven.min_price,
                      last_updated: date,
                      potential_profit: riven.potential_profit,
                    },
                  }))
                );
                modals.closeAll();
              }}
            />
          ),
        });
      }}
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
