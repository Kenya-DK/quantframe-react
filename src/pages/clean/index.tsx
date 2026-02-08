import { Text } from "@mantine/core";
import { useTranslateCommon } from "@hooks/useTranslate.hook";

import { ProcessTradePopup } from "@components/Popups/ProcessTrade";

const components = {
  process_trade: () => <ProcessTradePopup />,
};

export default function CleanPage() {
  // Get parameters from URL
  const urlParams = new URLSearchParams(window.location.search);
  const type = urlParams.get("type") || "process_trade";
  return (
    <>
      {components[type as keyof typeof components] ? components[type as keyof typeof components]() : <Text>{useTranslateCommon("not_found")}</Text>}
    </>
  );
}
