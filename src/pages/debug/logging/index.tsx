import { Card, MultiSelect } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";

export const Logging = () => {
  const [dbPath, setDbPath] = useLocalStorage<string>({ key: "dbPath", defaultValue: "" });
  const [type, setType] = useLocalStorage<string>({ key: "type", defaultValue: "" });

  return (
    <Card>
      <MultiSelect
        label={'label'}
        description={'description'}
        placeholder={'placeholder'}
        data={
          [
            { label: 'All', value: "*" },
            { label: 'WFM Auth', value: "wfm_client_auth" },
            { label: 'WFM Order', value: "wfm_client_order" },
            { label: 'WFM Item', value: "wfm_client_item" },
            { label: 'WFM Auction', value: "wfm_client_auction" },
            { label: 'WFM Chat', value: "wfm_client_chat" },
          ]
        }
        // value={value}
        // onChange={(value) => onChange(value as GdsShopApi.OrderStatus[])}
        // icon={<FontAwesomeIcon icon={faSearch} />}
        clearable
        searchable
        maw={400}
      />
    </Card>
  );
}