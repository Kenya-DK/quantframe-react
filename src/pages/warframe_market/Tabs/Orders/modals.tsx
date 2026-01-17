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
      modalKey: "prompt",
      title: useTranslateCommon("prompts.sell_manual.title"),
      innerProps: {
        fields: [
          {
            name: "quantity",
            label: useTranslateCommon("prompts.sell_manual.fields.quantity.label"),
            attributes: {
              min: 0,
              max: order.quantity,
            },
            value: order.quantity,
            type: "number",
          },
          {
            name: "sell",
            label: useTranslateCommon("prompts.sell_manual.fields.sell.label"),
            attributes: {
              min: 0,
            },
            value: order.platinum * order.quantity,
            type: "number",
          },
        ],
        onConfirm: async (data: { sell: number; quantity: number }) => {
          if (!order) return;
          const { sell, quantity } = data;
          await sellStockMutation.mutateAsync({ ...order, quantity, platinum: sell });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };
  const OpenBoughtModal = (order: WFMarketTypes.Order) => {
    modals.openContextModal({
      modalKey: "prompt",
      title: useTranslateCommon("prompts.bought_manual.title"),
      innerProps: {
        fields: [
          {
            name: "quantity",
            label: useTranslateCommon("prompts.bought_manual.fields.quantity.label"),
            attributes: {
              min: 0,
              max: order.quantity,
            },
            value: order.quantity,
            type: "number",
          },
          {
            name: "bought",
            label: useTranslateCommon("prompts.bought_manual.fields.bought.label"),
            attributes: {
              min: 0,
            },
            value: order.platinum * order.quantity,
            type: "number",
          },
        ],
        onConfirm: async (data: { bought: number; quantity: number }) => {
          if (!order) return;
          const { bought, quantity } = data;
          await createStockMutation.mutateAsync({ ...order, quantity, platinum: bought });
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
