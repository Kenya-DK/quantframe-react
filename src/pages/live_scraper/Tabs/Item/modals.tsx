import { modals } from "@mantine/modals";
import { Text } from "@mantine/core";
import { TauriTypes } from "$types";
import { StockItemDetailsModal } from "@components/Modals/StockItemDetails";

interface ModalHooks {
  useTranslateBasePrompt: (key: string, context?: { [key: string]: any }) => string;
  useTranslatePrompt: (key: string, context?: { [key: string]: any }) => string;
  updateStockMutation: {
    mutateAsync: (data: TauriTypes.UpdateStockItem) => Promise<any>;
  };
  sellStockMutation: {
    mutateAsync: (data: TauriTypes.SellStockItem) => Promise<any>;
  };
  deleteStockMutation: {
    mutateAsync: (id: number) => Promise<any>;
  };
}

export const useStockModals = ({
  useTranslateBasePrompt,
  useTranslatePrompt,
  updateStockMutation,
  sellStockMutation,
  deleteStockMutation,
}: ModalHooks) => {
  const OpenMinimumPriceModal = (id: number, minimum_price: number) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateBasePrompt("minimum_price.title"),
      innerProps: {
        fields: [
          {
            name: "minimum_price",
            label: useTranslateBasePrompt("minimum_price.fields.minimum_price.label"),
            attributes: {
              min: 0,
              description: useTranslateBasePrompt("minimum_price.fields.minimum_price.description"),
            },
            value: minimum_price,
            type: "number",
          },
        ],
        onConfirm: async (data: { minimum_price: number }) => {
          if (!id) return;
          const { minimum_price } = data;
          await updateStockMutation.mutateAsync({ id, minimum_price });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  const OpenSellModal = (stock: TauriTypes.StockItem) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateBasePrompt("sell.title"),
      innerProps: {
        fields: [
          {
            name: "sell",
            label: useTranslateBasePrompt("sell.fields.sell.label"),
            attributes: {
              min: 0,
            },
            value: 0,
            type: "number",
          },
        ],
        onConfirm: async (data: { sell: number }) => {
          if (!stock) return;
          const { sell } = data;
          await sellStockMutation.mutateAsync({ id: stock.id, wfm_url: stock.wfm_url, sub_type: stock.sub_type, price: sell, quantity: 1 });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  const OpenInfoModal = (item: TauriTypes.StockItem) => {
    modals.open({
      size: "100%",
      withCloseButton: false,
      children: <StockItemDetailsModal value={item.id} />,
    });
  };

  const OpenUpdateModal = (_items: TauriTypes.UpdateStockItem[]) => {
    modals.open({
      title: useTranslatePrompt("update_bulk.title"),
      children: <></>,
    });
  };

  const OpenDeleteModal = (id: number) => {
    modals.openConfirmModal({
      title: useTranslateBasePrompt("delete.title"),
      children: <Text size="sm">{useTranslateBasePrompt("delete.message", { count: 1 })}</Text>,
      labels: { confirm: useTranslateBasePrompt("delete.confirm"), cancel: useTranslateBasePrompt("delete.cancel") },
      onConfirm: async () => await deleteStockMutation.mutateAsync(id),
    });
  };

  return {
    OpenMinimumPriceModal,
    OpenSellModal,
    OpenInfoModal,
    OpenUpdateModal,
    OpenDeleteModal,
  };
};
