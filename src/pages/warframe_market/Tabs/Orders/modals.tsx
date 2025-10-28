import { modals } from "@mantine/modals";
import { Text } from "@mantine/core";
import { WFMarketTypes } from "$types";
import { useTranslateCommon } from "@hooks/useTranslate.hook";
import { WFMOrderDetailsModal } from "@components/Modals/WFMOrderDetails";

interface ModalHooks {
  useTranslateBasePrompt: (key: string, context?: { [key: string]: any }) => string;
  deleteStockMutation: {
    mutateAsync: (id: string) => Promise<any>;
  };
  createStockMutation: {
    mutateAsync: (data: WFMarketTypes.Order) => Promise<any>;
  };
  sellStockMutation: {
    mutateAsync: (data: WFMarketTypes.Order) => Promise<any>;
  };
}

export const useStockModals = ({ deleteStockMutation, createStockMutation, sellStockMutation }: ModalHooks) => {
  const OpenSellModal = (order: WFMarketTypes.Order) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateCommon("prompts.sell_manual.title"),
      innerProps: {
        fields: [
          {
            name: "sell",
            label: useTranslateCommon("prompts.sell_manual.fields.sell.label"),
            attributes: {
              min: 0,
            },
            value: 0,
            type: "number",
          },
        ],
        onConfirm: async (data: { sell: number }) => {
          if (!order) return;
          const { sell } = data;
          await sellStockMutation.mutateAsync({ ...order, quantity: 1, platinum: sell });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };
  const OpenBoughtModal = (order: WFMarketTypes.Order) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateCommon("prompts.bought_manual.title"),
      innerProps: {
        fields: [
          {
            name: "bought",
            label: useTranslateCommon("prompts.bought_manual.fields.bought.label"),
            attributes: {
              min: 0,
            },
            value: 0,
            type: "number",
          },
        ],
        onConfirm: async (data: { bought: number }) => {
          if (!order) return;
          const { bought } = data;
          await createStockMutation.mutateAsync({ ...order, quantity: 1, platinum: bought });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  const HandleModalOrder = (order: WFMarketTypes.Order) => {
    switch (order.type) {
      case WFMarketTypes.OrderType.Buy:
        OpenBoughtModal(order);
        break;
      case WFMarketTypes.OrderType.Sell:
        OpenSellModal(order);
        break;
    }
  };

  const OpenDeleteModal = (id: string) => {
    modals.openConfirmModal({
      title: useTranslateCommon("prompts.delete_item.title"),
      children: <Text size="sm">{useTranslateCommon("prompts.delete_item.message", { count: 1 })}</Text>,
      labels: { confirm: useTranslateCommon("prompts.delete_item.confirm"), cancel: useTranslateCommon("prompts.delete_item.cancel") },
      onConfirm: async () => await deleteStockMutation.mutateAsync(id),
    });
  };
  const OpenInfoModal = (item: WFMarketTypes.Order) => {
    modals.open({
      size: "100%",
      withCloseButton: false,
      children: <WFMOrderDetailsModal value={item.id} />,
    });
  };
  return {
    HandleModalOrder,
    OpenDeleteModal,
    OpenInfoModal,
  };
};
