import { Container, ScrollArea, SimpleGrid } from "@mantine/core";
import { AuctionListItem } from "../../components/AuctionListItem";

export default function TestPage() {
  const data = [
    {
      buyout_price: 70,
      closed: false,
      id: "66ec4ebaadc83a31e84d6159",
      info: {
        auctions: [],
        is_dirty: true,
        price_history: [],
      },
      is_direct_sell: true,
      item: {
        attributes: [
          {
            positive: true,
            url_name: "critical_chance",
            value: 23.4,
          },
          {
            positive: true,
            url_name: "base_damage_/_melee_damage",
            value: 24.3,
          },
        ],
        damage: null,
        element: null,
        extra_attributes: [
          {
            positive: true,
            url_name: "critical_chance",
            value: 23.4,
          },
          {
            positive: true,
            url_name: "base_damage_/_melee_damage",
            value: 24.3,
          },
        ],
        having_ephemera: null,
        mastery_level: 16,
        missing_attributes: [
          {
            positive: true,
            url_name: "cold_damage",
            value: 17.3,
          },
          {
            positive: true,
            url_name: "heat_damage",
            value: 15,
          },
          {
            positive: false,
            url_name: "damage_vs_infested",
            value: 0.97,
          },
        ],
        mod_rank: 0,
        name: "critaata",
        polarity: "naramon",
        quirk: null,
        re_rolls: 0,
        similarity: 0,
        type: "riven",
        weapon_url_name: "ogris",
      },
      minimal_reputation: 0,
      note: "",
      operation: [],
      owner: {
        avatar: null,
        id: "64f44f24facd4809cf846ca6",
        ingame_name: "xiaoye16",
        last_seen: "2024-09-23T15:18:26.565+00:00",
        locale: "zh-hans",
        region: "en",
        reputation: 0,
        status: "ingame",
      },
      starting_price: 70,
      visible: true,
    },
    {
      buyout_price: 98,
      closed: false,
      id: "66ec4bfc9454322ee0440851",
      info: {
        auctions: [],
        is_dirty: true,
        price_history: [],
      },
      is_direct_sell: true,
      item: {
        attributes: [
          {
            positive: true,
            url_name: "critical_damage",
            value: 17,
          },
          {
            positive: true,
            url_name: "toxin_damage",
            value: 12.5,
          },
        ],
        damage: null,
        element: null,
        extra_attributes: [
          {
            positive: true,
            url_name: "critical_damage",
            value: 17,
          },
          {
            positive: true,
            url_name: "toxin_damage",
            value: 12.5,
          },
        ],
        having_ephemera: null,
        mastery_level: 16,
        missing_attributes: [
          {
            positive: true,
            url_name: "cold_damage",
            value: 17.3,
          },
          {
            positive: true,
            url_name: "heat_damage",
            value: 15,
          },
          {
            positive: false,
            url_name: "damage_vs_infested",
            value: 0.97,
          },
        ],
        mod_rank: 0,
        name: "acritox",
        polarity: "vazarin",
        quirk: null,
        re_rolls: 8,
        similarity: 0,
        type: "riven",
        weapon_url_name: "ogris",
      },
      minimal_reputation: 0,
      note: "",
      operation: [],
      owner: {
        avatar: null,
        id: "66b2566fe25fe30008179dcd",
        ingame_name: "Ji-Huang",
        last_seen: "2024-09-23T15:05:26.958+00:00",
        locale: "zh-hans",
        region: "en",
        reputation: 0,
        status: "ingame",
      },
      starting_price: 98,
      visible: true,
    },
    {
      buyout_price: 100,
      closed: false,
      id: "66f172535ca3222eedcf745c",
      info: {
        auctions: [],
        is_dirty: true,
        price_history: [],
      },
      is_direct_sell: true,
      item: {
        attributes: [
          {
            positive: true,
            url_name: "toxin_damage",
            value: 46.9,
          },
          {
            positive: true,
            url_name: "fire_rate_/_attack_speed",
            value: 32.5,
          },
          {
            positive: true,
            url_name: "zoom",
            value: 29.1,
          },
        ],
        damage: null,
        element: null,
        extra_attributes: [
          {
            positive: true,
            url_name: "toxin_damage",
            value: 46.9,
          },
          {
            positive: true,
            url_name: "fire_rate_/_attack_speed",
            value: 32.5,
          },
          {
            positive: true,
            url_name: "zoom",
            value: 29.1,
          },
        ],
        having_ephemera: null,
        mastery_level: 9,
        missing_attributes: [
          {
            positive: true,
            url_name: "cold_damage",
            value: 17.3,
          },
          {
            positive: true,
            url_name: "heat_damage",
            value: 15,
          },
          {
            positive: false,
            url_name: "damage_vs_infested",
            value: 0.97,
          },
        ],
        mod_rank: 0,
        name: "croni-toxilis",
        polarity: "vazarin",
        quirk: null,
        re_rolls: 60,
        similarity: 0,
        type: "riven",
        weapon_url_name: "ogris",
      },
      minimal_reputation: 0,
      note: "",
      operation: [],
      owner: {
        avatar: null,
        id: "6666b4f9c405f61939eb3ecc",
        ingame_name: "flowerthorner",
        last_seen: "2024-09-23T13:49:28.239+00:00",
        locale: "zh-hans",
        region: "en",
        reputation: 1,
        status: "ingame",
      },
      starting_price: 100,
      visible: true,
    },
    {
      buyout_price: 100,
      closed: false,
      id: "66f163a39454322ff4e8e1fd",
      info: {
        auctions: [],
        is_dirty: true,
        price_history: [],
      },
      is_direct_sell: true,
      item: {
        attributes: [
          {
            positive: true,
            url_name: "critical_chance",
            value: 20.7,
          },
          {
            positive: true,
            url_name: "critical_damage",
            value: 27.1,
          },
          {
            positive: false,
            url_name: "status_chance",
            value: -5.9,
          },
        ],
        damage: null,
        element: null,
        extra_attributes: [
          {
            positive: true,
            url_name: "critical_chance",
            value: 20.7,
          },
          {
            positive: true,
            url_name: "critical_damage",
            value: 27.1,
          },
          {
            positive: false,
            url_name: "status_chance",
            value: -5.9,
          },
        ],
        having_ephemera: null,
        mastery_level: 12,
        missing_attributes: [
          {
            positive: true,
            url_name: "cold_damage",
            value: 17.3,
          },
          {
            positive: true,
            url_name: "heat_damage",
            value: 15,
          },
          {
            positive: false,
            url_name: "damage_vs_infested",
            value: 0.97,
          },
        ],
        mod_rank: 0,
        name: "critatis",
        polarity: "naramon",
        quirk: null,
        re_rolls: 30,
        similarity: 0,
        type: "riven",
        weapon_url_name: "ogris",
      },
      minimal_reputation: 0,
      note: "",
      operation: [],
      owner: {
        avatar: null,
        id: "63bbae77f553531159583ec2",
        ingame_name: "zsn88",
        last_seen: "2024-09-23T14:51:21.838+00:00",
        locale: "zh-hans",
        region: "en",
        reputation: 3,
        status: "ingame",
      },
      starting_price: 100,
      visible: true,
    },
  ] as any[];
  return (
    <Container size={"100%"}>
      <ScrollArea mt={"md"} h={"calc(100vh - 300px)"}>
        <SimpleGrid cols={{ base: 1, sm: 2, lg: 2 }} spacing="lg">
          {data.map((order, i) => (
            <AuctionListItem
              key={i}
              // compacted
              show_image
              auction={order}
            />
          ))}
        </SimpleGrid>
      </ScrollArea>
    </Container>
  );
}
