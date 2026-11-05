import { Group, Text } from "@mantine/core";
import { PreviewCard } from "@components/Shared/PreviewCard";
import { RivenGrade } from "@components/DataDisplay/RivenGrade";
import { RivenAttribute } from "@components/DataDisplay/RivenAttribute";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { DisplayPlatinum } from "@components/DataDisplay/DisplayPlatinum";
import { Center } from "@mantine/core";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { faAdd, faCoins } from "@fortawesome/free-solid-svg-icons";
import { upperFirst } from "@mantine/hooks";
import { useState } from "react";
import api from "@api/index";
import { ItemRiven } from "$types";

/**
 * A single rendered inventory broken out of the Rivens tab so it can be
 * reused by other WF inventory surfaces (e.g. the "next" layout).
 * Owns its own price-estimate action; the caller owns the add-to-stock flow via `onAdd`.
 */
export const RivenPreviewCard = ({ riven, onAdd }: { riven: ItemRiven; onAdd: (riven: ItemRiven) => void }) => {
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`wf_inventory.tabs.riven.riven_card.${key}`, { ...context }, i18Key);

  return (
    <PreviewCard
      pos={"relative"}
      value={riven}
      headerLeft={{
        hide: riven.properties?.price === undefined,
        fz: "lg",
        i18nKey: `components.riven_preview.types.${riven.riven_type}.header_left`,
        values: {
          price: riven.properties?.price || 0,
        },
      }}
      headerCenter={{
        fz: "lg",
        i18nKey: "components.riven_preview.riven_name",
        values: { name: riven.name, mod_name: upperFirst(riven.mod_name) },
      }}
      headerRight={
        <>
          {riven.riven_type == "veiled" && <RivenGrade value={riven.properties.grade || "unknown"} pos={"absolute"} right={10} size={35} />}
        </>
      }
      renderBody={() => (
        <>
          {riven.riven_type == "veiled" &&
            riven.attributes.map((attr) => (
              <RivenAttribute key={attr.url_name} i18nKey="full" groupProps={{ p: 1 }} value={attr} hideDetails centered hideGrade />
            ))}
          {riven.riven_type != "veiled" && (
            <Center h={"100%"}>
              <Text>{riven.properties.challenge_description_with_complication}</Text>
            </Center>
          )}
        </>
      )}
      footerLeft={{
        fz: "lg",
        i18nKey: `components.riven_preview.types.${riven.riven_type}.footer_left`,
        values: {
          quantity: riven.quantity || 1,
          rank: riven.sub_type?.rank || 0,
          required: riven.properties?.required || 0,
          progress: riven.properties?.progress || 0,
        },
      }}
      footerCenter={
        <Group gap={3}>
          {riven.riven_type === "veiled" && (
            <ActionWithTooltip
              icon={faAdd}
              color={riven.properties?.is_in_stock ? "var(--mantine-color-green-6)" : "var(--mantine-color-red-6)"}
              actionProps={{ size: "sm" }}
              iconProps={{ size: "xs" }}
              tooltip={useTranslate("stock_status." + (riven.properties?.is_in_stock ? "found" : "not_found"))}
              onClick={async () => onAdd(riven)}
            />
          )}
          {riven.riven_type === "veiled" && <RivenPriceButton riven={riven} />}
        </Group>
      }
      footerRight={{
        hide: riven.riven_type != "veiled",
        fz: "lg",
        i18nKey: `components.riven_preview.types.${riven.riven_type}.footer_right`,
        values: { mastery: riven.mastery_rank },
      }}
    />
  );
};

interface RivenPriceButtonProps {
  riven: ItemRiven;
}

const RivenPriceButton: React.FC<RivenPriceButtonProps> = ({ riven }) => {
  const [loading, setLoading] = useState(false);
  const [price, setPrice] = useState<number | null>(null);
  const [unavailable, setUnavailable] = useState(false);

  const useTranslate = (key: string) => useTranslatePages(`wf_inventory.tabs.riven.riven_card.${key}`);
  const getPriceLabel = useTranslate("get_price");
  const unavailableLabel = useTranslate("price_unavailable");

  const HandleGetPrice = async () => {
    setLoading(true);
    setUnavailable(false);
    try {
      const estimate = await api.riven.predictPrice({
        weapon: riven.wfm_url || riven.name,
        re_rolls: riven.re_rolls,
        positives: riven.attributes.filter((attr) => attr.positive).map((attr) => attr.wfmUrl),
        negative: riven.attributes.find((attr) => !attr.positive)?.wfmUrl,
      });
      if (estimate) {
        setPrice(estimate.price);
      } else {
        setPrice(null);
        setUnavailable(true);
      }
    } catch (e) {
      console.error(e);
      setPrice(null);
      setUnavailable(true);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Group gap={4}>
      <ActionWithTooltip
        icon={faCoins}
        color={price !== null ? "var(--mantine-color-yellow-6)" : "var(--mantine-color-blue-6)"}
        actionProps={{ size: "sm" }}
        iconProps={{ size: "xs" }}
        loading={loading}
        tooltip={unavailable ? unavailableLabel : getPriceLabel}
        onClick={HandleGetPrice}
      />
      {price !== null && <DisplayPlatinum value={Math.round(price)} size="sm" />}
    </Group>
  );
};
